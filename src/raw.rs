/*
    supaterm – terminal manipulation library
    Copyright (C) 2026  @polyagonal1

    This program is free software: you can redistribute it and/or modify
    it under the terms of the GNU General Public License as published by
    the Free Software Foundation, either version 3 of the License, or
    (at your option) any later version.

    This program is distributed in the hope that it will be useful,
    but WITHOUT ANY WARRANTY; without even the implied warranty of
    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
    GNU General Public License for more details.

    You should have received a copy of the GNU General Public License
    along with this program.  If not, see <https://www.gnu.org/licenses/>
*/

//! API for enabling and disabling raw mode.
//!
//! # What is raw mode?
//!
//! There are 2 main 'modes' a terminal can be in:
//! - Canonical ('cooked') mode – The default mode
//! - Raw mode – less/no input/output processing is done by the TTY
//!
//! When a terminal program starts, the terminal (should) be in canonical mode.
//! The terminal program can set raw mode for more direct control over I/O but
//! it *should* disable raw mode when it exits. Otherwise, the terminal will be
//! left in raw mode which could break subsequent programs running in the
//! terminal who expect canonical mode.
//!
//! When canonical mode is enabled:
//! - When the user types something, it is echoed back to the user
//!   automatically.
//! - Input is only sent when the user presses enter.
//! - `Ctrl-C` terminates the process and `Ctrl-Z` pauses it.
//! - `Ctrl-S` pauses transmission of data to the terminal until `Ctrl-Q`
//!   is pressed [^1].
//! - When the user enters a carriage return, it gets translated into a
//!   newline.
//! - When you write a newline (`\n`) into stdout, it gets translated into
//!   a CRLF (`\r\n`).
//! - Maybe other minor things depending on the terminal/OS.
//!
//! When raw mode is enabled:
//! - Input is not echoed back to the user.
//! - Input can be read byte-by-byte rather than only being sent when the
//!   user presses enter.
//! - Prevents `Ctrl-C` from terminating the process and `Ctrl-Z` from
//!   suspending it. These are instead sent to the process.
//! - Disables `Ctrl-S` which pauses transmission of data to the terminal
//!   and `Ctrl-Q` which continues transmission of data to the terminal [^1].
//!   These are instead sent to the program running.
//! - Prevents carriage returns from stdin from being translated into
//!   newlines.
//! - Prevents newlines (`\n`) written into stdout from being translated
//!   into a carriage return and a newline (`\r\n`).
//! - Maybe more minor things depending on the terminal/OS.
//!
//! [^1]: https://viewsourcecode.org/snaptoken/kilo/02.enteringRawMode.html#disable-ctrl-s-and-ctrl-q

use std::{
	io,
	mem::ManuallyDrop,
	os::fd::AsRawFd,
};

use rustix::fd::BorrowedFd;

use crate::terminal_mode::{TerminalMode, set_terminal_mode_of_fd};

#[must_use = "If you do not use this return value, raw mode may be disabled sooner than you would like."]
pub struct RawTerminal<'tty> {
	original_mode: TerminalMode,
	fd: BorrowedFd<'tty>,
}

impl<'tty> Drop for RawTerminal<'tty> {
	fn drop(&mut self) {
		let _ = disable_raw_mode_inner(&self);
	}
}

pub use crate::terminal_mode::AccessModeError;

/// Enables raw mode for the terminal.
///
/// This returns a [`RawTerminal`] struct whose `Drop` implementation disables
/// raw mode.
///
/// ***Do not discard the returned `RawTerminal`. If you do, it will be dropped
/// immediately and raw mode will be disabled sooner than you would like:***
/// ```should-panic
/// use supaterm::raw::enable_raw_mode;
///
/// enable_raw_mode()?;
///
/// // raw mode will not be enabled
///
/// todo!("Add check for raw mode being enabled.")
/// ```
///
/// *Correct* usage:
// TODO: make this example run in a container where the stdio fds point to a terminal
/// ```no-run
/// # use std::io;
/// # use rustix::{termios::isatty, stdio};
/// use supaterm::raw::enable_raw_mode;
///
/// fn main() -> io::Result<()> {
///
///     let _raw_terminal = enable_raw_mode()?;
///
///     // ... do stuff with raw mode enabled
///
/// 	# Ok(())
/// } // the `RawTerminal` stored in the `_` is dropped and raw mode is disabled.
/// ```
pub fn enable_raw_mode<'tty>() -> Result<RawTerminal<'tty>, AccessModeError> {
	let (old_mode, fd) = TerminalMode::get_current()?;

	Ok(enable_raw_mode_with_fd_inner(fd, old_mode)?)
}

/// Enables raw mode with the given file descriptor.
pub fn enable_raw_mode_with_fd<'tty>(fd: BorrowedFd<'tty>) -> io::Result<RawTerminal<'tty>> {
	let old_mode = TerminalMode::get_mode_of_fd(fd)?;

	enable_raw_mode_with_fd_inner(fd, old_mode)
}

fn enable_raw_mode_with_fd_inner<'tty>(fd: BorrowedFd<'tty>, original_mode: TerminalMode) -> io::Result<RawTerminal<'tty>> {
	let mut raw_mode = original_mode.clone();
	raw_mode.make_raw();

	set_terminal_mode_of_fd(fd, &raw_mode)?;

	Ok(RawTerminal {
		original_mode,
		fd,
	})
}

/// Disables raw mode early and returns to canonical ('cooked') mode, the
/// default mode.
///
/// You can also just drop the `raw_terminal` but this function provides error
/// values, which the `Drop` implementation for `RawTerminal` ignores.
pub fn disable_raw_mode(raw_terminal: RawTerminal) -> io::Result<()> {
	// wrap `raw_terminal` in a `ManuallyDrop` to prevent `raw_terminal`'s
	// destructor from running and disabling raw mode twice
	let raw_terminal = ManuallyDrop::new(raw_terminal);

	disable_raw_mode_inner(&raw_terminal)
}

fn disable_raw_mode_inner(raw_terminal: &RawTerminal) -> io::Result<()> {
	set_terminal_mode_of_fd(raw_terminal.fd, &raw_terminal.original_mode)
}

// /// Attempts to infer whether raw mode is currently enabled
// pub fn is_raw_mode_enabled() -> io::Result<bool> {
// 	todo!()
// }