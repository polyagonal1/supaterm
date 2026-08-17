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

use crate::Terminal;

/// Trait enabling the use of the alternate screen. It is implemented on
/// [`Terminal`]. For an alternate screen trait that is implemented on any
/// type implementing [`Write`].
pub trait AlternateScreen: Write {
	/// Enables the alternate screen.
	/// 
	/// If using a [`Terminal`], it is recommended to assign a drop function 
	/// to the terminal disabling the alternate screen on drop like so:
	/// ```rust
	/// use std::io;
	/// use supaterm::{Terminal, AlternateScreen};
	/// 
	/// # fn main() -> io::Result<()> {
	/// 
	/// let mut term = Terminal::new();
	/// term.enter_alternate_screen()?;
	/// 
	/// // makes `term` disable the alternate screen on drop
	/// term.assign_drop_fn("alternate_screen", |term| { term.disable_alternate_screen(); });
	/// 
	/// 
	/// // do stuff in the alternate screen here
	/// 
	/// 
	/// drop(term); // this calls `AlternateScreen::disable_alternate_screen`
	/// 
	/// println!("Hey it's back to normal now.");
	/// 
	/// # Ok(())
	/// # }
	/// ```
	fn enter_alternate_screen(&mut self) -> io::Result<()>;
	
	/// Disables the alternate screen and removes the drop function to disable 
	/// the alternate screen on `Self`'s `Drop` implementation.
	fn disable_alternate_screen(&mut self) -> io::Result<()>;
}

impl<'tty, I, O, R> AlternateScreen for Terminal<'tty, I, O, R>
where
	Self: Write
{
	fn enter_alternate_screen(&mut self) -> io::Result<()> {
		self.write_all(b"\x1b[?1049h")
	}
	
	fn disable_alternate_screen(&mut self) -> io::Result<()> {
		self.write_all(b"\x1b[1049l")
	}
}
