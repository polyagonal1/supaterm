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

#[cfg(feature = "cursor_controls")]
mod cursor;
#[cfg(feature = "alternate_screen")]
mod alternate_screen;
#[cfg(feature = "erase_functions")]
mod erase;

#[cfg(feature = "cursor_controls")]
pub use cursor::CursorControls;
#[cfg(feature = "alternate_screen")]
pub use alternate_screen::{AlternateScreen, EnterAlternateScreen};

const CSI: &'static [u8] = b"\x1b[";

#[cfg(any(feature = "cursor_controls", feature = "alternate_screen", feature = "erase_functions"))]
use writable::*;

#[cfg(any(feature = "cursor_controls", feature = "alternate_screen", feature = "erase_functions"))]
mod writable {
	use std::io::{self, Write};
	use itoa::{Buffer, Integer};
	
	pub(super) trait Writeable {
		fn write_to<W: Write>(&self, writer: &mut W) -> io::Result<()>;
	}

	impl Writeable for str {
		#[inline]
		fn write_to<W: Write>(&self, writer: &mut W) -> io::Result<()> {
			writer.write_all(self.as_bytes())
		}
	}

	impl Writeable for [u8] {
		#[inline]
		fn write_to<W: Write>(&self, writer: &mut W) -> io::Result<()> {
			writer.write_all(self)
		}
	}

	impl<I: Integer> Writeable for I {
		#[inline]
		fn write_to<W: Write>(&self, writer: &mut W) -> io::Result<()> {
			writer.write_all(
				Buffer::new().format(*self).as_bytes()
			)
		}
	}

	macro_rules! write_all {
		(
			$writer:expr
			$(,
				$item:expr
			)* $(,)?
		) => {
			$(
				$item.write_to($writer)?;
			)*
		}
	}

	pub(super) use write_all;
}