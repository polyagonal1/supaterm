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

pub trait CursorControls {
	fn go_to_home(&mut self) -> io::Result<()>;

	fn go_to_pos(&mut self, line: u16, column: u16) -> io::Result<()>;

	fn move_up(&mut self, n: u16) -> io::Result<()>;

	fn move_down(&mut self, n: u16) -> io::Result<()>;

	fn move_right(&mut self, n: u16) -> io::Result<()>;

	fn move_left(&mut self, n: u16) -> io::Result<()>;

	fn go_to_start_and_down(&mut self, n: u16) -> io::Result<()>;

	fn go_to_start_and_up(&mut self, n: u16) -> io::Result<()>;

	fn go_to_column(&mut self, n: u16) -> io::Result<()>;
}

impl<'tty, I, O, R> CursorControls for crate::Terminal<'tty, I, O, R>
where
	Self: Write
{
	fn go_to_home(&mut self) -> io::Result<()> {
		self.write_all(b"\x1b[H")
	}

	/// Moves the cursor to the provided line and column
	fn go_to_pos(&mut self, line: u16, column: u16) -> io::Result<()> {
		write_all!(self,
			CSI,
			line,
			b";",
			column,
			"H",
		);

		Ok(())
	}

	/// Moves the cursor up by `n` lines
	fn move_up(&mut self, n: u16) -> io::Result<()> {
		write_all!(self,
			CSI,
			n,
			"A",
		);

		Ok(())
	}

	/// Moves the cursor down by `n` lines
	fn move_down(&mut self, n: u16) -> io::Result<()> {
		write_all!(self,
			CSI,
			n,
			"B",
		);

		Ok(())
	}

	/// Moves the cursor right by `n` columns
	fn move_right(&mut self, n: u16) -> io::Result<()> {
		write_all!(self,
			CSI,
			n,
			"C",
		);

		Ok(())
	}

	/// Moves the cursor left by `n` columns
	fn move_left(&mut self, n: u16) -> io::Result<()> {
		write_all!(self,
			CSI,
			n,
			"D",
		);

		Ok(())
	}

	/// Moves the cursor to the start of the line, then moves it `n` lines down
	fn go_to_start_and_down(&mut self, n: u16) -> io::Result<()> {
		write_all!(self,
			CSI,
			n,
			"E",
		);

		Ok(())
	}

	/// Moves the cursor to the start of the line, then moves it `n` lines up
	fn go_to_start_and_up(&mut self, n: u16) -> io::Result<()> {
		write_all!(self,
			CSI,
			n,
			"F"
		);

		Ok(())
	}

	/// Moves the cursor to the `n`th column
	fn go_to_column(&mut self, n: u16) -> io::Result<()> {
		write_all!(self,
			CSI,
			n,
			"G"
		);

		Ok(())
	}

}