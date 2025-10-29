/*
    supaterm – a terminal manipulation library
    Copyright (C) 2025  polyagonal1

    This program is free software: you can redistribute it and/or modify
    it under the terms of the GNU General Public License as published by
    the Free Software Foundation, either version 3 of the License, or
    (at your option) any later version.

    This program is distributed in the hope that it will be useful,
    but WITHOUT ANY WARRANTY; without even the implied warranty of
    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
    GNU General Public License for more details.

    You should have received a copy of the GNU General Public License
    along with this program.  If not, see <https://www.gnu.org/licenses/>.
*/

use std::{fmt, io};
use crate::Command;

/// Equivalent to pressing the backspace key on a keyboard
pub struct Backspace;

impl Command for Backspace {
    fn queue(&self, target: &mut dyn io::Write) -> io::Result<()> {
        target.write_all(b"\x08")
    }

    fn size_hint(&self) -> Option<usize> {
        Some(1)
    }

    fn reset_cmd(&self) -> Option<Box<dyn Command>> {
        None
    }
}

impl fmt::Display for Backspace {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("\x08")
    }
}

/// Equivalent to pressing the delete key on a keyboard (not to be confused with backspace)
pub struct DeleteChar;

impl Command for DeleteChar {
    fn queue(&self, target: &mut dyn io::Write) -> io::Result<()> {
        target.write_all(b"\x7F")
    }

    fn size_hint(&self) -> Option<usize> {
        Some(1)
    }

    fn reset_cmd(&self) -> Option<Box<dyn Command>> {
        None
    }
}

impl fmt::Display for DeleteChar {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("\x7F")
    }
}

/// Clears everything from the cursor to the end of the screen
pub struct ClearCursorToEndOfScreen;

impl Command for ClearCursorToEndOfScreen {
    fn queue(&self, target: &mut dyn io::Write) -> io::Result<()> {
        target.write_all(b"\x1b[0J")
    }

    fn size_hint(&self) -> Option<usize> {
       Some(4)
    }

    fn reset_cmd(&self) -> Option<Box<dyn Command>> {
        None
    }
}

impl fmt::Display for ClearCursorToEndOfScreen {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("\x1b[0J")
    }
}

/// Clears everything from the start of the screen to the cursor
pub struct ClearStartOfScreenToCursor;

impl Command for ClearStartOfScreenToCursor {
    fn queue(&self, target: &mut dyn io::Write) -> io::Result<()> {
        target.write_all(b"\x1b[1J")
    }

    fn size_hint(&self) -> Option<usize> {
        Some(4)
    }

    fn reset_cmd(&self) -> Option<Box<dyn Command>> {
        None
    }
}

impl fmt::Display for ClearStartOfScreenToCursor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("\x1b[1J")
    }
}

/// Clears the entire screen
///
/// Best used when the alternate screen is enabled, as to not disrupt the other contents of the main
/// screen of the terminal
///
/// On some terminals, this command also moves the cursor to the home position (0, 0), so this
/// command is best used just before manually moving the cursor the home position
pub struct ClearScreen;

impl Command for ClearScreen {
    fn queue(&self, target: &mut dyn io::Write) -> io::Result<()> {
        target.write_all(b"\x1b[2J")
    }

    fn size_hint(&self) -> Option<usize> {
        Some(4)
    }

    fn reset_cmd(&self) -> Option<Box<dyn Command>> {
        None
    }
}

impl fmt::Display for ClearScreen {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("\x1b[2J")
    }
}

/// Clears everything from the cursor to the end of the current line
pub struct ClearCursorToEndOfLine;

impl Command for ClearCursorToEndOfLine {
    fn queue(&self, target: &mut dyn io::Write) -> io::Result<()> {
        target.write_all(b"\x1b[0K")
    }

    fn size_hint(&self) -> Option<usize> {
        Some(4)
    }

    fn reset_cmd(&self) -> Option<Box<dyn Command>> {
        None
    }
}

impl fmt::Display for ClearCursorToEndOfLine {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("\x1b[0K")
    }
}

/// Clears everything from the start of the current line to the cursor
pub struct ClearStartOfLineToCursor;

impl Command for ClearStartOfLineToCursor {
    fn queue(&self, target: &mut dyn io::Write) -> io::Result<()> {
        target.write_all(b"\x1b[1K")
    }

    fn size_hint(&self) -> Option<usize> {
        Some(4)
    }

    fn reset_cmd(&self) -> Option<Box<dyn Command>> {
        None
    }
}

impl fmt::Display for ClearStartOfLineToCursor {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("\x1b[1K")
    }
}

/// Clears the contents of the current line
pub struct ClearLine;

impl Command for ClearLine {
    fn queue(&self, target: &mut dyn io::Write) -> io::Result<()> {
        target.write_all(b"\x1b[2K")
    }

    fn size_hint(&self) -> Option<usize> {
        Some(4)
    }

    fn reset_cmd(&self) -> Option<Box<dyn Command>> {
        None
    }
}

impl fmt::Display for ClearLine {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("\x1b[2K")
    }
}
