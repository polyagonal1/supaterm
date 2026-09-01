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

use std::io::{self, Write};

use super::{CSI, Writeable, write_all};

/// Trait that allows the cursor to be moved more expressively by the program
/// than it would be otherwise.
///
/// This trait is automatically implemented for any type implementing
/// [`Write`].
pub trait CursorControls {
	/// Moves the cursor to the top-left corner of the screen.
	fn go_to_home(&mut self) -> io::Result<()>;

	/// Moves the cursor to the specified line and column.
	fn go_to_pos(&mut self, line: u16, column: u16) -> io::Result<()>;

	/// Moves the cursor `n` lines up.
	fn move_up(&mut self, n: u16) -> io::Result<()>;

	/// Moves the cursor `n` lines down.
	fn move_down(&mut self, n: u16) -> io::Result<()>;

	/// Moves the cursor `n` columns right.
	fn move_right(&mut self, n: u16) -> io::Result<()>;

	/// Moves the cursor `n` columns left.
	fn move_left(&mut self, n: u16) -> io::Result<()>;

	/// Moves the cursor to the start of the `n`th line down from the cursor.
	fn go_to_next_line(&mut self, n: u16) -> io::Result<()>;

	/// Moves the cursor to the start of the `n`th line up from the cursor.
	fn go_to_prev_line(&mut self, n: u16) -> io::Result<()>;

	/// Moves the cursor to the specified column.
	fn go_to_column(&mut self, n: u16) -> io::Result<()>;
}

impl<T> CursorControls for T
where
	T: Write
{
	fn go_to_home(&mut self) -> io::Result<()> {
		self.write_all(b"\x1b[H")
	}

	fn go_to_pos(&mut self, line: u16, column: u16) -> io::Result<()> {
		// the actual escape code used for this is 1-based, not 0-based, so we
		// have to add 1 to the values provided
		write_all!(self,
			CSI,
			line + 1,
			b";",
			column + 1,
			b"H",
		);

		Ok(())
	}

	fn move_up(&mut self, n: u16) -> io::Result<()> {
		write_all!(self,
			CSI,
			n,
			b"A",
		);

		Ok(())
	}

	fn move_down(&mut self, n: u16) -> io::Result<()> {
		write_all!(self,
			CSI,
			n,
			b"B",
		);

		Ok(())
	}

	/// Moves the cursor right by `n` columns
	fn move_right(&mut self, n: u16) -> io::Result<()> {
		write_all!(self,
			CSI,
			n,
			b"C",
		);

		Ok(())
	}

	/// Moves the cursor left by `n` columns
	fn move_left(&mut self, n: u16) -> io::Result<()> {
		write_all!(self,
			CSI,
			n,
			b"D",
		);

		Ok(())
	}

	/// Moves the cursor to the start of the line, then moves it `n` lines down
	fn go_to_next_line(&mut self, n: u16) -> io::Result<()> {
		write_all!(self,
			CSI,
			n,
			b"E",
		);

		Ok(())
	}

	/// Moves the cursor to the start of the line, then moves it `n` lines up
	fn go_to_prev_line(&mut self, n: u16) -> io::Result<()> {
		write_all!(self,
			CSI,
			n,
			b"F"
		);

		Ok(())
	}

	/// Moves the cursor to the `n`th column
	fn go_to_column(&mut self, n: u16) -> io::Result<()> {
		write_all!(self,
			CSI,
			n,
			b"G"
		);

		Ok(())
	}

}