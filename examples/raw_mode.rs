/*
    chromoterm – terminal manipulation library
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
	io::{self, Write},
};

use chromoterm::raw::enable_raw_mode;

fn main() -> Result<(), Box<dyn error::Error>> {
	{
		let mut stdout = io::stdout().lock();

		// this should output 'World!' directly below 'Hello'
		write!(stdout, "Hello\nWorld!\n\n")?;

		let _raw_terminal = enable_raw_mode()?;

		write!(stdout, "Raw mode is enabled now.\r\n\r\n")?;

		// this should output 'World!' diagonally below and to the right of
		//  'Hello' because newlines are not translated into CRLFs
		write!(stdout, "Hello\nWorld\r\n\r\n")?;

	} // `_raw_terminal` gets dropped here and raw mode is disabled

	println!("And back to canonical mode.");

	Ok(())
}