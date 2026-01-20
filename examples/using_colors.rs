use supaterm::{
    style::{SetForegroundColor, Color, ResetStyle, Colors},
    self as st
};
    
use std::io::{self, Write};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    
    let mut term = st::Terminal::new(
        io::stdin().lock(),
        io::stdout().lock()
    )?;

    if !term.is_capability_supported(Colors) {
        term.write_all(b"Colors aren't supported?! What age is this terminal from?")?;
        return Ok(())
    }

    // the standard colors are almost always going to be supported if colors are supported at all
    term.queue(SetForegroundColor(Color::Red))?;
    term.write_all(b"This text is red.")?;
    
    for i in 0..=255u8 {

        if i == 0 {
            term.write_all(b"\nStandard colors\n")?;
        }
        if i == 8 {
            term.write_all(b"\nStandard bright colors\n")?;
        }
        if i == 16 {
            term.write_all(b"\n6x6x6 color cube\n")?;
        }
        if i == 232 {
            term.write_all(b"\nGreyscale in 24 steps\n")?;
        }

        match term.is_capability_supported(SetForegroundColor(Color::ColorById(i))) {
            true => term.queue(SetForegroundColor(Color::ColorById(i)))?,
            false => {
                write!(term, "The terminal does not support colors with an id higher than {}", i-1)?;
                break;
            }
        }


        write!(term, "{i:<4}")?;

        term.queue(ResetStyle)?;

        if i == 51 || i == 87 || i == 123 || i == 159 || i == 195 {
            term.write_all(b"\n")?;
        }
    }

    term.write_all(b"\n")?;
    
    Ok(())
}