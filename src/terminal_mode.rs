/*
    chromoterm – terminal manipulation library
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

//! Lower-level API than [`crate::raw`] which uses this API internally.
//! 
//! Assuming a POSIX-like OS, a TTY (terminal) has *attributes*. These define 
//! how the terminal behaves. There are also different categories of terminal 
//! attributes. One category is output modes and defines how standard output 
//! (stdout) behaves. One of the attributes (flags) in output modes is `OCRNL` 
//! which controls if the terminal maps carriage returns to newlines or not. 
//! 
//! There are other categories and many more attributes. (See 
//! [this section of the POSIX spec](posix-termios) and [this man page](c-termios) 
//! for more info.)
//! 
//! Certain combinations of these attributes make a terminal in 'raw mode'. The 
//! affects of raw mode is documented in [`crate::raw`]. You can see 
//! the exact flags setting raw mode sets and removes [here](cfmakeraw-impl).
//! 
//! Currently, this API does not allow for manually tuning these flags, but it 
//! may be considered in the future.
//! 
//! [posix-termios]: https://pubs.opengroup.org/onlinepubs/9799919799/
//! [c-termios]: https://man7.org/linux/man-pages/man3/termios.3.html
//! [cfmakeraw-impl]: https://man7.org/linux/man-pages/man3/termios.3.html#:~:text=termios_p-,CS8;

use std::{
	fmt,
	io,
	error,
	os::fd::AsFd,
};
use rustix::{
	io::Errno,
	fd::BorrowedFd,
	termios::{Termios, tcgetattr, tcsetattr, OptionalActions},
};

use crate::sys::{
	retry_on_nonfatal,
	try_get_tty_fd
};

/// Terminal attributes. See [the module documentation][`self`] for more info.
#[derive(Debug, Clone)]
pub struct TerminalMode {
	termios: Termios,
}

/// Wrapper around [`io::Error`] encoding the extra error condition: could not 
/// find the file descriptor of the TTY
#[derive(Debug)]
#[non_exhaustive]
pub enum AccessModeError {
	/// Stdin, stdout, and stderr were all not pointing to a TTY able to 
	/// get/set attributes of.
	CouldNotFindTty,
	/// Other IO error.
	Other(io::Error),
}

impl fmt::Display for AccessModeError {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		match self {
			AccessModeError::CouldNotFindTty => write!(f, "could not find the tty"),
			AccessModeError::Other(io_error) => fmt::Display::fmt(io_error, f),
		}
	}
}

impl error::Error for AccessModeError {
	fn source(&self) -> Option<&(dyn error::Error + 'static)> {
		match self {
			AccessModeError::Other(io_error) => Some(io_error),
			AccessModeError::CouldNotFindTty => None,
		}
	}
}

impl From<AccessModeError> for io::Error {
	fn from(error: AccessModeError) -> Self {
		match error {
			AccessModeError::Other(io_error) => io_error,
			AccessModeError::CouldNotFindTty => Errno::NOTTY.into()
		}
	}
}

impl From<io::Error> for AccessModeError {
	fn from(io_error: io::Error) -> Self {
		AccessModeError::Other(io_error)
	}
}

impl From<Errno> for AccessModeError {
	fn from(errno: Errno) -> Self {
		match errno {
			Errno::NOTTY | Errno::BADF => Self::CouldNotFindTty,
			other => Self::Other(other.into()),
		}
	}
}

impl TerminalMode {
	/// Attempts to find the file descriptor of the tty (terminal) and get its 
	/// current attributes.
	/// 
	/// This first tries stdin, then stdout, then stderr in that order and if 
	/// setting terminal mode on all of them fails with `ENOTTY` or `EBADF`, 
	/// it returns [`AccessModeError::CouldNotFindTty`]
	pub fn get_current<'a>() -> Result<(Self, BorrowedFd<'a>), AccessModeError> {
		let tty_fd = try_get_tty_fd().ok_or(AccessModeError::CouldNotFindTty)?;

		Ok((Self::get_mode_of_fd(tty_fd)?, tty_fd))
	}
	
	/// Gets the terminal attributes of the specified file descriptor. 
	///
	/// The TTY can be referred to using the file descriptors of stdin, stdout, 
	/// or stderr normally; but if, for example, stdout was redirected to a 
	/// file, attempting to set terminal mode on stdout would fail with the OS 
	/// error code of `ENOTTY` (the fd was not a terminal). 
	/// 
	/// Since [`TerminalMode::get_current`] tries stdin, stdout, and stderr, this 
	/// won't normally be an issue but if all of those are redirected to 
	/// something other than a terminal and you have a different fd referencing 
	/// a terminal that you want to use instead, or you want direct control 
	/// over the order stdin, stdout, and stderr are tried, this function can 
	/// be useful.
	pub fn get_mode_of_fd<Fd: AsFd>(tty_fd: Fd) -> io::Result<Self> {
		let fd = tty_fd.as_fd();
		
		Ok(Self {
			termios: retry_on_nonfatal(|| tcgetattr(fd))?,
		})
	}
	
	/// Makes `Self` represent raw mode.
	pub fn make_raw(&mut self) {
		self.termios.make_raw()
	}
}

/// Attempts to find the fd of the terminal, then set its attributes to 
/// `terminal_mode`.
pub fn set_terminal_mode(terminal_mode: &TerminalMode) -> Result<(), AccessModeError> {
	let tty_fd = try_get_tty_fd().ok_or(AccessModeError::CouldNotFindTty)?;
	
	Ok(set_terminal_mode_of_fd(tty_fd, terminal_mode)?)
}

/// Sets the terminal attributes of `fd` to `mode`.
pub fn set_terminal_mode_of_fd<Fd: AsFd>(fd: Fd, mode: &TerminalMode) -> io::Result<()> {
	let fd = fd.as_fd();
	Ok(retry_on_nonfatal(
		|| tcsetattr(fd, OptionalActions::Now, &mode.termios)
	)?)
}