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
	time::Duration,
	thread::sleep,
	io::{self, Write},
	error,
};

use supaterm::{
	EnterAlternateScreen,
	CursorControls,
};

fn main() -> Result<(), Box<dyn error::Error>> {
	println!("We are about to enter the alternate screen.");

	sleep(Duration::from_secs(2));

	{
		let mut stdout = io::stdout().lock().enter_alternate_screen()?;
		// entering the alternate screen does not neccessarily place the cursor
		// in the home position, so we move it there
		stdout.go_to_home()?;

		writeln!(stdout, "We are now in the alternate screen.")?;
		writeln!(stdout, "You should not be able to see anything other than these 2 sentences in the terminal.")?;

		sleep(Duration::from_secs(4));
	}

	println!("We have now exited the alternate screen");

	Ok(())
}