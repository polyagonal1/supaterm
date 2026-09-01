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

/// Cursor movement.
///
/// This trait has methods which allow the cursor to be moved more expressively
/// by the program than it would be otherwise. It is automatically implemented
/// for any type implementing [`Write`].
pub trait CursorControls {
	/// Moves the cursor to the top-left corner of the screen.
	fn go_to_home(&mut self) -> io::Result<()>;

	/// Moves the cursor to the specified line and column.
	///
	/// The `line` and `column` arguments are 0-based. This means a position of
	/// (0, 0) is the top-left corner of the terminal.
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

/// Implement cursor movements for any writer
///
/// Internally, this implementation writes [ANSI escape codes] to the writer.
/// Because of this, this will not actually move any cursor for non-tty
/// writers, like [`Vec<T>`], just write the escape code.
///
/// Note: This implementation of `CursorControls` does not attempt to do any
/// buffering so if you use these methods directly on a [`File`] for example,
/// there will be at least one (but possibly more) system calls for each method
/// call.
///
/// [ANSI escape codes]: <https://gist.github.com/fnky/458719343aabd01cfb17a3a4f7296797>
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

#[cfg(test)]
mod cursor_controls_tests {
	use super::CursorControls;

	use std::{
		ascii::EscapeDefault,
		slice::Iter,
		io::{self, Write},
	};

	struct Escaper<'a> {
		main_iter: Iter<'a, u8>,
		byte_iter: Option<EscapeDefault>,
	}

	impl<'a> Iterator for Escaper<'a> {
		type Item = char;

		fn next(&mut self) -> Option<Self::Item> {
			match &mut self.byte_iter {
				Some(escape_default) => match escape_default.next() {
					Some(byte) => return Some(byte as char),
					// byte_iter is Some(empty_iter)
					None => (),
				},
				// byte_iter is None
				None => ()
			}

			match self.main_iter.next() {
				Some(unescaped_byte) => {
					self.byte_iter = Some(unescaped_byte.escape_ascii());

					self.next()
				},
				None => None,
			}
		}
	}

	fn escape_buf(buf: &[u8]) -> String {
		let escaper = Escaper {
			main_iter: buf.iter(),
			byte_iter: None,
		};

		escaper.collect()
	}

	#[test]
	fn test_go_to_home() -> io::Result<()> {
		let mut buf: Vec<u8> = Vec::new();

		buf.go_to_home()?;

		assert_eq!(buf, b"\x1b[H");

		Ok(())
	}

	#[test]
	fn test_go_to_pos() -> io::Result<()> {
		let mut buf: Vec<u8> = Vec::new();

		buf.go_to_pos(0, 0)?;

		buf.write_all(b"hello")?;

		buf.go_to_pos(5, 2)?;

		buf.write_all(b"world")?;

		eprintln!("buf = {}", escape_buf(&*buf));
		assert_eq!(buf, b"\x1b[1;1Hhello\x1b[6;3Hworld");

		Ok(())
	}
}