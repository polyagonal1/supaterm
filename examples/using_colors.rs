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
    self as st,
    style::{
        Color, Colors, StandardColor, BrightColor,
        ResetStyle, SetForegroundColor, SetBackgroundColor
    },
};

use std::io::Write;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut term = st::Terminal::new()?;
    
    // standard colors
    
    if !term.is_capability_supported(Colors::Standard) {
        term.write_all(b"Colors aren't supported?! What age is this terminal from?")?;
        return Ok(());
    }
    
    term.queue(SetForegroundColor(Color::Standard(StandardColor::Red)))?;
    term.write_all(b"This text is red.\n")?;
    
    term.queue(ResetStyle)?;
    term.write_all(b"This text is not red.\n")?;
    
    // bright colors
    
    if !term.is_capability_supported(Colors::Bright) {
        term.write_all(b"Bright colors are not supported. How unfortunate.\n")?;
        return Ok(())
    }
    
    term.queue(SetBackgroundColor(Color::Bright(BrightColor::BrightYellow)))?;
    term.write_all(b"This text is bright yellow.")?;
    term.queue(ResetStyle)?;
    term.write_all(b"\n")?;
    
    // 256-color mode
    
    if !term.is_capability_supported(Colors::From256ColorPalette) {
        term.write_all(b"256 color mode is unsupported.\n")?;
        return Ok(())
    }
    
    for i in 16..=255u8 {
        if i == 16 {
            term.write_all(b"\n6x6x6 color cube\n")?;
        }
        if i == 232 {
            term.write_all(b"\nGreyscale in 24 steps\n")?;
        }

        term.queue(SetForegroundColor(Color::From256ColorPalette(i)))?;

        write!(term, "{i:<4}")?;

        term.queue(ResetStyle)?;

        // formatting to insert a newline every
        if i == 51 || i == 87 || i == 123 || i == 159 || i == 195 {
            term.write_all(b"\n")?;
        }
    }

    term.write_all(b"\n")?;

    Ok(())
}
