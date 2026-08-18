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
	error,
	io::Write,
};

use supaterm::{
	Terminal,
	raw::enable_raw_mode,
};

fn main() -> Result<(), Box<dyn error::Error>> {
	{
		let mut term = Terminal::new();

		// this should output 'World!' directly below 'Hello'
		write!(term, "Hello\nWorld!\n\n")?;

		let _raw_terminal = enable_raw_mode()?;

		write!(term, "Raw mode is enabled now.\r\n\r\n")?;

		// this should output 'World!' diagonally below and to the right of
		//  'Hello' because newlines are not translated into CRLFs
		write!(term, "Hello\nWorld\r\n\r\n")?;
	}

	println!("And back to canonical mode.");

	Ok(())
}