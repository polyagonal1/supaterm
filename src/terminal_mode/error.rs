use std::{error, fmt, io};

use rustix::io::Errno;

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