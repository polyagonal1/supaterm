use std::{
	time,
	thread,
	io::Write,
};

use supaterm::{
	Terminal,
	misc::{EnterAlternateScreen, ExitAlternateScreen}
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
	{
		let mut term = Terminal::new()?;

		if !term.is_capability_supported(EnterAlternateScreen) {
			term.write_all(b"The alternate screen is unsupported.")?;
			return Err(Box::new("The alternate screen is unsupported"))
		}

		term.on_drop(ExitAlternateScreen);
		term.queue(EnterAlternateScreen)?;

		term.write_all(b"We are in the alternate screen. The terminal should have turned into a fullscreen terminal now.\n")?;

		thread::sleep(time::Duration::new(5, 0));

	} // `term` gets dropped here and the `ExitAlternateScreen` command gets run

	println!("We should be out of the alternate screen now.");

	Ok(())
}
