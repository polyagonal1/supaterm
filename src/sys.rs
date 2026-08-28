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

	/// Calls the function provided with each stdio file descriptor until it
	/// doesn't return a non-fatal error, returning the file descriptor that worked.
	pub fn try_with_stdio_fds<F, T>(f: F) -> Result<(T, BorrowedFd<'static>), Errno>
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
}

#[cfg(feature = "raw_mode")]
pub use terminal_mode::*;