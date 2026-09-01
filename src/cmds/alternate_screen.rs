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
	ops::{Deref, DerefMut},
	io::{self, Write},
};

/// Wrapper around a writer which shows that the alternate screen is enabled
/// whose `Drop` implementation disables the alternate screen.
///
/// This is created with [`EnterAlternateScreen::enter_alternate_screen`].
///
/// This struct also has implementations of `Deref<Target = W>` and `DerefMut`
/// so any methods available on the inner writer are available directly on this
/// struct as well.
#[non_exhaustive]
pub struct AlternateScreen<W: Write> {
	pub writer: W,
}

/// Allows for methods on the inner writer taking `&self` as a receiver to be
/// called directly on `AlternateScreen`: `self.method()`, rather than having
/// to access the field: `self.writer.method()`.
///
/// ```rust
/// use supaterm::{AlternateScreen, EnterAlternateScreen};
/// use std::{io, os::fd::AsFd};
///
/// # fn main() -> io::Result<()> {
/// let mut alternate_screen: AlternateScreen<io::Stdout> = io::stdout().enter_alternate_screen()?;
///
/// // AsFd is not implemented for `AlternateScreen` but it is for `StdoutLock`
/// let fd = alternate_screen.as_fd();
/// # Ok(())
/// # }
/// ```
impl<W: Write> Deref for AlternateScreen<W> {
	type Target = W;

	fn deref(&self) -> &Self::Target {
		&self.writer
	}
}

impl<W: Write> DerefMut for AlternateScreen<W> {
	fn deref_mut(&mut self) -> &mut Self::Target {
		&mut self.writer
	}
}

impl<W: Write> Drop for AlternateScreen<W> {
	fn drop(&mut self) {
		let _ = self.write_all(b"\x1b[?1049l");
	}
}

/// Trait enabling the use of the alternate screen. It is implemented on
/// any type implementing [`Write`].
pub trait EnterAlternateScreen: Write + Sized {
	/// Enables the alternate screen, consuming `self` and returning an 
	/// [`AlternateScreen`] struct containing `self` whose `Drop` 
	/// implementation disables the alternate screen.
	///
	/// You can call this 
	/// 
	/// # Examples
	/// 
	/// ```rust
	/// use std::io;
	/// use supaterm::{EnterAlternateScreen, CursorControls};
	/// 
	/// # fn main() -> io::Result<()> {
	/// 
	/// let mut stdout = io::stdout().lock().enter_alternate_screen()?;
	/// stdout.go_to_home()?;
	/// 
	/// // do stuff in the alternate screen here
	/// 
	/// drop(stdout); // this disables the alternate screen
	/// 
	/// println!("Hey it's back to normal now.");
	/// 
	/// # Ok(())
	/// # }
	/// ```
	fn enter_alternate_screen(self) -> io::Result<AlternateScreen<Self>>;
}

impl<W: Write> EnterAlternateScreen for W {
	fn enter_alternate_screen(mut self) -> io::Result<AlternateScreen<Self>> {
		self.write_all(b"\x1b[?1049h")?;
		
		Ok(AlternateScreen {
			writer: self
		})
	}
}