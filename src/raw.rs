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

use std::io;

use rustix::termios::{Termios, OptionalActions, tcgetattr, tcsetattr};

use crate::Terminal;

pub(crate) enum RawModeEnabled {
	Enabled { original_termios: Termios },
	Disabled,
}

impl RawModeEnabled {
	fn is_enabled(&self) -> bool {
		match self {
			RawModeEnabled::Enabled { .. } => true,
			RawModeEnabled::Disabled => false,
		}
	}
}

pub trait RawMode<'tty> {
	/// Enables raw mode for the terminal. Also assigns a drop function to
	/// `Self` disabling raw mode on drop, making sure the terminal is back to
	/// [canonical mode][RawMode::disable_raw_mode] when the program exits.
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
	fn enable_raw_mode(&mut self) -> io::Result<()>;

	/// Disables raw mode early and returns to canonical ('cooked') mode, the
	/// default mode. Removes the drop function assigned by
	/// [`RawMode::enable_raw_mode`], if any
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
	fn disable_raw_mode(&mut self) -> io::Result<()>;

	fn is_raw_mode_enabled(&self) -> bool;
}

impl<'tty, I, O, R> RawMode<'tty> for Terminal<'tty, I, O, R> {
	fn enable_raw_mode(&mut self) -> io::Result<()> {
		let fd = self.state.tty_fd;
		
		let current_mode: Termios = tcgetattr(fd)?;
		let mut new_mode = current_mode.clone();

		self.state.raw_mode_enabled = RawModeEnabled::Enabled {
			original_termios: current_mode,
		};

		self.assign_drop_fn("disable_raw_mode", |term| {
			if let RawModeEnabled::Enabled { original_termios } = &term.state.raw_mode_enabled {
				for _ in 0..2 {
					match tcsetattr(term.state.tty_fd, OptionalActions::Flush, &original_termios) {
						Ok(_) => break,
						Err(_) => continue,
					}
				}
			}
		});

		new_mode.make_raw();

		tcsetattr(fd, OptionalActions::Flush, &new_mode)?;

		Ok(())
	}

	/// Disables raw mode early and removes the drop function that disables raw
	/// mode, if any.
	fn disable_raw_mode(&mut self) -> io::Result<()> {

		if let RawModeEnabled::Enabled { original_termios } = &self.state.raw_mode_enabled {
			tcsetattr(self.state.tty_fd, OptionalActions::Flush, &original_termios)?;

			self.remove_drop_fn("disable_raw_mode");

			self.state.raw_mode_enabled = RawModeEnabled::Disabled;
		}

		Ok(())
	}

	fn is_raw_mode_enabled(&self) -> bool {
		self.state.raw_mode_enabled.is_enabled()
	}
}