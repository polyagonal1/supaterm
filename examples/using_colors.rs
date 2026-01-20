/*
    supaterm – terminal manipulation library allowing use of colored text and other functionality is planned
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