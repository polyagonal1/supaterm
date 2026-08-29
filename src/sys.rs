#[cfg(feature = "raw_mode")]
mod terminal_mode {
	use rustix::{
		stdio,
		io::Errno,
		fd::BorrowedFd,
		termios::isatty,
	};
	
	pub const STDIN_FD: BorrowedFd<'static> = stdio::stdin();
	pub const STDOUT_FD: BorrowedFd<'static> = stdio::stdout();
	pub const STDERR_FD: BorrowedFd<'static> = stdio::stderr();

	pub fn try_get_tty_fd() -> Option<BorrowedFd<'static>> {
		if isatty(STDIN_FD) {
			return Some(STDIN_FD)
		}

		if isatty(STDOUT_FD) {
			return Some(STDOUT_FD)
		}

		if isatty(STDERR_FD) {
			return Some(STDERR_FD)
		}

		None
	}

	pub fn retry_on_nonfatal<F, T>(f: F) -> rustix::io::Result<T>
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
}

#[cfg(feature = "raw_mode")]
pub use terminal_mode::*;