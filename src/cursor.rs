use std::{io, fmt};
use crate::Command;

/// Moves the cursor onto the next vertical tab spot.
///
/// This is usually equivalent to a newline or is just ignored entirely in modern terminals
pub struct VerticalTab;

impl Command for VerticalTab {
    fn queue(&self, target: &mut dyn io::Write) -> io::Result<()> {
        target.write_all(b"\x0B")
    }

    fn size_hint(&self) -> Option<usize> {
        Some(1)
    }

    fn reset_cmd(&self) -> Option<Box<dyn Command>> {
        None
    }
}

impl fmt::Display for VerticalTab {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("\x0B")
    }
}

pub struct GoHome;

impl Command for GoHome {
    fn queue(&self, target: &mut dyn io::Write) -> io::Result<()> {
        target.write_all(b"\x1b[H")
    }

    fn size_hint(&self) -> Option<usize> {
        Some(3)
    }

    fn reset_cmd(&self) -> Option<Box<dyn Command>> {
        None
    }
}

/// Moves the cursor to the top-left cell (0, 0)
impl fmt::Display for GoHome {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("\x1b[H")
    }
}

/// Moves the cursor to (line, column)
pub struct GoTo(pub u16, pub u16);

impl Command for GoTo {
    fn queue(&self, target: &mut dyn io::Write) -> io::Result<()> {
        write!(target, "\x1b[{};{}f", self.0, self.1)
    }

    fn size_hint(&self) -> Option<usize> {
        Some(10)
    }

    fn reset_cmd(&self) -> Option<Box<dyn Command>> {
        None
    }
}

impl fmt::Display for GoTo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "\x1b[{};{}f", self.0, self.1)
    }
}

/// Moves the cursor up by n lines
pub struct GoUpBy(pub u16);

impl Command for GoUpBy {
    fn queue(&self, target: &mut dyn io::Write) -> io::Result<()> {
        write!(target, "\x1b[{}A", self.0)
    }

    fn size_hint(&self) -> Option<usize> {
        Some(6)
    }

    fn reset_cmd(&self) -> Option<Box<dyn Command>> {
        None
    }
}

impl fmt::Display for GoUpBy {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "\x1b[{}A", self.0)
    }
}

/// Moves the cursor down by n lines
pub struct GoDownBy(pub u16);

impl Command for GoDownBy {
    fn queue(&self, target: &mut dyn io::Write) -> io::Result<()> {
        write!(target, "\x1b[{}B", self.0,)
    }

    fn size_hint(&self) -> Option<usize> {
        Some(6)
    }

    fn reset_cmd(&self) -> Option<Box<dyn Command>> {
        None
    }
}

impl fmt::Display for GoDownBy {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "\x1b[{}B", self.0)
    }
}

pub struct GoRightBy(pub u16);

/// Moves the cursor right by n columns
impl Command for GoRightBy {
    fn queue(&self, target: &mut dyn io::Write) -> io::Result<()> {
        write!(target, "\x1b[{}C", self.0,)
    }

    fn size_hint(&self) -> Option<usize> {
        Some(6)
    }

    fn reset_cmd(&self) -> Option<Box<dyn Command>> {
        None
    }
}

impl fmt::Display for GoRightBy {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "\x1b[{}C", self.0)
    }
}

/// Moves the cursor left by n columns
pub struct GoLeftBy(pub u16);

impl Command for GoLeftBy {
    fn queue(&self, target: &mut dyn io::Write) -> io::Result<()> {
        write!(target, "\x1b[{}D", self.0,)
    }

    fn size_hint(&self) -> Option<usize> {
        Some(6)
    }

    fn reset_cmd(&self) -> Option<Box<dyn Command>> {
        None
    }
}

impl fmt::Display for GoLeftBy {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "\x1b[{}D", self.0)
    }
}

/// Moves the cursor to the beginning of the current line, then n lines down
pub struct GoToBeginningAndDown(pub u16);

impl Command for GoToBeginningAndDown {
    fn queue(&self, target: &mut dyn io::Write) -> io::Result<()> {
        write!(target, "\x1b[{}E", self.0)
    }

    fn size_hint(&self) -> Option<usize> {
        Some(6)
    }

    fn reset_cmd(&self) -> Option<Box<dyn Command>> {
        None
    }
}

/// Moves the cursor to the beginning of the current line, then n lines down
impl fmt::Display for GoToBeginningAndDown {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "\x1b[{}E", self.0)
    }
}

pub struct GoToBeginningAndUp(pub u16);

impl Command for GoToBeginningAndUp {
    fn queue(&self, target: &mut dyn io::Write) -> io::Result<()> {
        write!(target, "\x1b[{}F", self.0)
    }

    fn size_hint(&self) -> Option<usize> {
        Some(6)
    }

    fn reset_cmd(&self) -> Option<Box<dyn Command>> {
        None
    }
}

impl fmt::Display for GoToBeginningAndUp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "\x1b[{}F", self.0)
    }
}

/// Moves the cursor to the specified column
pub struct GoToColumn(pub u16);

impl Command for GoToColumn {
    fn queue(&self, target: &mut dyn io::Write) -> io::Result<()> {
        write!(target, "\x1b[{}G", self.0)
    }

    fn size_hint(&self) -> Option<usize> {
        Some(6)
    }

    fn reset_cmd(&self) -> Option<Box<dyn Command>> {
        None
    }
}

impl fmt::Display for GoToColumn {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "\x1b[{}G", self.0)
    }
}

/// Requests the cursor position
///
/// When this is sent, the terminal will reply via stdin the cursor position in the format:
/// `\x1b[{row};{column}R`
pub struct RequestCursorPosition;

impl Command for RequestCursorPosition {
    fn queue(&self, target: &mut dyn io::Write) -> io::Result<()> {
        target.write_all(b"\x1b[6n")
    }

    fn size_hint(&self) -> Option<usize> {
        Some(6)
    }

    fn reset_cmd(&self) -> Option<Box<dyn Command>> {
        None
    }
}

impl fmt::Display for RequestCursorPosition {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("\x1b[6n")
    }
}

/// Moves the cursor up by one line, keeping the column the same
pub struct GoUpByOne;

impl Command for GoUpByOne {
    fn queue(&self, target: &mut dyn io::Write) -> io::Result<()> {
        target.write_all(b"\x1bM")
    }

    fn size_hint(&self) -> Option<usize> {
        Some(6)
    }

    fn reset_cmd(&self) -> Option<Box<dyn Command>> {
        None
    }
}

impl fmt::Display for GoUpByOne {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("\x1bM")
    }
}

/// Moves the cursor down by one line, keeping the column the same
pub struct GoDownByOne;

impl Command for GoDownByOne {
    fn queue(&self, target: &mut dyn io::Write) -> io::Result<()> {
        target.write_all(b"\x1bD")
    }

    fn size_hint(&self) -> Option<usize> {
        Some(6)
    }

    fn reset_cmd(&self) -> Option<Box<dyn Command>> {
        None
    }
}

impl fmt::Display for GoDownByOne {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("\x1bD")
    }
}

/// Moves the cursor left by one column, keeping the line the same
pub struct GoLeftByOne;

impl Command for GoLeftByOne {
    fn queue(&self, target: &mut dyn io::Write) -> io::Result<()> {
        target.write_all(b"\x1b[D")
    }

    fn size_hint(&self) -> Option<usize> {
        Some(6)
    }

    fn reset_cmd(&self) -> Option<Box<dyn Command>> {
        None
    }
}

impl fmt::Display for GoLeftByOne {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("\x1b[D")
    }
}

/// Moves the cursor right by one column, keeping the line the same
pub struct GoRightByOne;

impl Command for GoRightByOne {
    fn queue(&self, target: &mut dyn io::Write) -> io::Result<()> {
        target.write_all(b"\x1b[C")
    }

    fn size_hint(&self) -> Option<usize> {
        Some(6)
    }

    fn reset_cmd(&self) -> Option<Box<dyn Command>> {
        None
    }
}

impl fmt::Display for GoRightByOne {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("\x1b[C")
    }
}

/// Saves the current cursor position
pub struct SavePosition;

impl Command for SavePosition {
    fn queue(&self, target: &mut dyn io::Write) -> io::Result<()> {
        target.write_all(b"\x1b7")
    }

    fn size_hint(&self) -> Option<usize> {
        Some(6)
    }

    fn reset_cmd(&self) -> Option<Box<dyn Command>> {
        None
    }
}

/// Restores the last saved cursor position
impl fmt::Display for SavePosition {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("\x1b7")
    }
}

pub struct RestorePosition;

impl Command for RestorePosition {
    fn queue(&self, target: &mut dyn io::Write) -> io::Result<()> {
        target.write_all(b"\x1b8")
    }

    fn size_hint(&self) -> Option<usize> {
        Some(6)
    }

    fn reset_cmd(&self) -> Option<Box<dyn Command>> {
        None
    }
}

impl fmt::Display for RestorePosition {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("\x1b8")
    }
}