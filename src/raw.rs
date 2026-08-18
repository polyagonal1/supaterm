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

use std::{
	io,
	mem::ManuallyDrop,
};

use rustix::{
	io::Errno,
	stdio,
	fd::BorrowedFd,
	termios::{Termios, OptionalActions, tcgetattr, tcsetattr},
};

pub struct RawTerminal<'tty> {
	original_termios: Termios,
	fd: BorrowedFd<'tty>,
}

impl<'tty> Drop for RawTerminal<'tty> {
	fn drop(&mut self) {
		let _ = disable_raw_mode_inner(&self);
	}
}

impl<'tty> RawTerminal<'tty> {
	fn new(original_termios: Termios, tty_fd: BorrowedFd<'tty>) -> Self {
		Self {
			original_termios,
			fd: tty_fd,
		}
	}
}

fn retry_on_nonfatal<F, T>(f: F) -> rustix::io::Result<T>
where
	F: Fn() -> rustix::io::Result<T>
{
	loop {
		match f() {
			Ok(r) => return Ok(r),
			Err(Errno::INTR) | Err(Errno::AGAIN) => continue,
			Err(other) => return Err(other)
		}
	}
}

/// Calls the function provided with each stdio file descriptor until it
/// doesn't return a non-fatal error, returning the file descriptor that worked.
fn try_with_stdio_fds<F, T>(f: F) -> Result<(T, BorrowedFd<'static>), Errno>
where
	F: Fn(BorrowedFd) -> rustix::io::Result<T>
{
	let stdin_fd: BorrowedFd<'static> = stdio::stdin();

	// try calling `f(stdin_fd)` until it doesn't return a non-fatal error
	match retry_on_nonfatal(|| f(stdin_fd)) {
		Ok(success) => return Ok((success, stdin_fd)),
		// the file descriptor was redirected to something other than a tty
		// so we try the other stdio file descriptors
		Err(Errno::NOTTY) | Err(Errno::BADF) => (),
		Err(other) => return Err(other),
	}

	let stdout_fd: BorrowedFd<'static> = stdio::stdout();

	// try calling `f(stdout_fd)` until it doesn't return a non-fatal error
	match retry_on_nonfatal(|| f(stdout_fd)) {
		Ok(success) => return Ok((success, stdout_fd)),
		// the file descriptor was redirected to something other than a tty
		// so we try the last stdio file descriptor: stderr
		Err(Errno::NOTTY) | Err(Errno::BADF) => (),
		Err(other) => return Err(other),
	}

	let stderr_fd = stdio::stderr();

	retry_on_nonfatal(|| f(stderr_fd)).map(|success| (success, stderr_fd))
}

/// Enables raw mode for the terminal.
///
/// This returns a [`RawTerminal`] struct whose `Drop` implementation disables
/// raw mode.
///
/// When raw mode is enabled:
/// - Input is not echoed back to the user.
/// - Input can be read byte-by-byte rather than only being sent when the
///   user presses enter.
/// - Prevents `Ctrl-C` from terminating the process and `Ctrl-Z` from
///   suspending it. These are instead sent to the process.
/// - Disables `Ctrl-S` which pauses transmission of data to the terminal
///   and `Ctrl-Q` which continues transmission of data to the terminal [^1].
///   These are instead sent to the program running.
/// - Prevents carriage returns from stdin from being translated into
///   newlines.
/// - Prevents newlines (`\n`) written into stdout from being translated
///   into a carriage return and a newline (`\r\n`).
/// - Maybe more minor things depending on the terminal/OS.
///
/// [^1]: https://viewsourcecode.org/snaptoken/kilo/02.enteringRawMode.html#disable-ctrl-s-and-ctrl-q
pub fn enable_raw_mode<'tty>() -> io::Result<RawTerminal<'tty>> {
	let (old_mode, fd) = try_with_stdio_fds::<_, Termios>(|fd| tcgetattr(fd))?;

	enable_raw_mode_with_fd_inner(old_mode, fd)
}

/// Enables raw mode with the given file descriptor.
pub fn enable_raw_mode_with_fd<'tty>(fd: BorrowedFd<'tty>) -> io::Result<RawTerminal<'tty>> {
	let old_mode = retry_on_nonfatal::<_, Termios>(|| tcgetattr(fd))?;

	enable_raw_mode_with_fd_inner(old_mode, fd)
}

fn enable_raw_mode_with_fd_inner<'tty>(old_mode: Termios, fd: BorrowedFd<'tty>) -> io::Result<RawTerminal<'tty>> {
	let mut raw_mode = old_mode.clone();
	raw_mode.make_raw();

	retry_on_nonfatal(|| tcsetattr(fd, OptionalActions::Now, &raw_mode))?;

	Ok(RawTerminal::new(old_mode, fd))
}

/// Disables raw mode early and returns to canonical ('cooked') mode, the
/// default mode.
///
/// In canonical mode:
/// - When the user types something, it is echoed back to the user
///   automatically.
/// - Input is only sent when the user presses enter.
/// - `Ctrl-C` terminates the process and `Ctrl-Z` pauses it.
/// - `Ctrl-S` pauses transmission of data to the terminal until `Ctrl-Q`
///   is pressed [^1].
/// - When the user enters a carriage return, it gets translated into a
///   newline.
/// - When you write a newline (`\n`) into stdout, it gets translated into
///   a CRLF (`\r\n`).
/// - Maybe other minor things depending on the terminal/OS.
///
/// [^1]: <https://viewsourcecode.org/snaptoken/kilo/02.enteringRawMode.html#disable-ctrl-s-and-ctrl-q>
pub fn disable_raw_mode(raw_terminal: RawTerminal) -> io::Result<()> {
	// wrap `raw_terminal` in a `ManuallyDrop` to prevent `raw_terminal`'s
	// destructor from running and disabling raw mode twice
	let raw_terminal = ManuallyDrop::new(raw_terminal);

	disable_raw_mode_inner(&raw_terminal)
}

fn disable_raw_mode_inner(raw_terminal: &RawTerminal) -> io::Result<()> {
	Ok(retry_on_nonfatal(||
		tcsetattr(
			raw_terminal.fd,
			OptionalActions::Flush,
			&raw_terminal.original_termios
		)
	)?)
}