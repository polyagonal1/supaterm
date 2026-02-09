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
use crate::{Capability, Command, define};

use std::{io, env};

use infoterm::{
    entry::Entry,
    expand,
    index::{Boolean, Integer, String},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Writeln<'a>(pub &'a [u8]);

impl<'a> Command for Writeln<'a> {
    fn size_hint(&self) -> Option<usize> {
        // add 2 because `write_to()` appends a CRLF
        Some(self.0.len() + 2)
    }

    fn write_to(&self, _database: &Entry, target: &mut dyn io::Write) -> io::Result<()> {
        target.write_all(self.0)?;

        target.write_all(b"\r\n")?;

        Ok(())
    }
}

impl<'a> Capability for Writeln<'a> {
    type IsSupportedType = bool;

    fn is_supported(&self, _: &Entry) -> bool {
        // every terminal supports writing to it, right?
        true
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Write<'a>(pub &'a [u8]);

impl<'a> Command for Write<'a> {
    fn size_hint(&self) -> Option<usize> {
        Some(self.0.len())
    }

    fn write_to(&self, _database: &Entry, target: &mut dyn io::Write) -> io::Result<()> {
        target.write_all(self.0)?;

        Ok(())
    }
}

impl<'a> Capability for Write<'a> {
    type IsSupportedType = bool;

    fn is_supported(&self, _: &Entry) -> bool {
        true
    }
}

define!(default-no-args
    /// Resets the current terminal style
    ///
    /// This command uses the 'sgr0' ('exit_attribute_mode') capability in terminfo which is
    /// described by the [linux man page](terminfo_docs) as 'turn off all attributes'. This is
    /// interpreted differently across terminal implementations so this may reset some other aspects
    /// of the terminal depending on the terminal. This may not always reset colours as well.
    ///
    /// [terminfo_docs]: https://man7.org/linux/man-pages/man5/terminfo.5.html
    definition: pub struct ResetStyle,
    capability: String::ExitAttributeMode,
    capability_getter: Entry::get_string,
    size_hint: Some(8),
    unsupported_msg: "Resetting the current style (terminfo cap-name 'sgr0' in terminfo) is unsupported on this terminal",
);

define!(default-no-args
    /// Enables bold mode
    definition: pub struct SetBold,
    capability: String::EnterBoldMode,
    capability_getter: Entry::get_string,
    size_hint: Some(8),
    unsupported_msg: "Bold mode (terminfo cap-name 'bold') is unsupported in this terminal",
);

define!(default-no-args
    /// Makes text that is written after this command underlined
    definition: pub struct SetUnderline,
    capability: String::EnterUnderlineMode,
    capability_getter: Entry::get_string,
    size_hint: Some(8),
    unsupported_msg: "Underline mode (terminfo cap-name 'smul') is unsupported in this terminal",
);

define!(default-no-args
    /// Disables underlined mode (see [EnterUnderline])
    definition: pub struct ResetUnderline,
    capability: String::ExitUnderlineMode,
    capability_getter: Entry::get_string,
    size_hint: Some(8),
    unsupported_msg: "Resetting underline mode explicitly (terminfo cap-name 'rmul') is unsupported in this terminal",
);

define!(default-no-args
    /// Switches the foreground and background colours for text written after this command has been executed
    definition: pub struct SetReverseMode,
    capability: String::EnterReverseMode,
    capability_getter: Entry::get_string,
    size_hint: Some(8),
    unsupported_msg: "Reverse video mode (terminfo cap-name 'rev') is unsupported in this terminal",
);

define!(default-no-args
    /// Makes text written after this command stand out. This usually implies something like [reverse
    /// mode], and/or [bold mode], and/or [underline mode] depending on the terminal and/or user
    /// configuration for the terminal
    ///
    /// [reverse mode]: SetReverseMode
    /// [bold mode]: SetBold
    /// [underline mode]: SetUnderline
    definition: pub struct SetStandoutMode,
    capability: String::EnterStandoutMode,
    capability_getter: Entry::get_string,
    size_hint: Some(20),
    unsupported_msg: "Standout mode (terminfo cap-name 'smso') is unsupported in this terminal",
);

define!(default-no-args
    /// Disables standout mode (see [SetStandoutMode])
    definition: pub struct ResetStandoutMode,
    capability: String::ExitStandoutMode,
    capability_getter: Entry::get_string,
    size_hint: Some(20),
    unsupported_msg: "Resetting standout mode explicitly (terminfo cap-name 'rmso') is unsupported in this terminal",
);

define!(default-no-args
    /// Makes text written after this command appear dimmed
    definition: pub struct SetDim,
    capability: String::EnterDimMode,
    capability_getter: Entry::get_string,
    size_hint: Some(8),
    unsupported_msg: "Dim mode (terminfo cap-name 'dim') is unsupported in this terminal",
);

define!(default-no-args
    /// Makes text written after this command blink
    definition: pub struct SetBlinking,
    capability: String::EnterBlinkMode,
    capability_getter: Entry::get_string,
    size_hint: Some(8),
    unsupported_msg: "Blinking mode (terminfo cap-name 'blink' is unsupported in this terminal",
);

define!(default-no-args
    /// Makes text written after this command invisible
    definition: pub struct SetInvisible,
    capability: String::EnterSecureMode,
    capability_getter: Entry::get_string,
    size_hint: Some(8),
    unsupported_msg: "Invisible mode (terminfo cap-name 'invis') is unsupported in this terminal",
);

// FIXME: Once `!` is stabilised, change `std::convert::Infallible` to `!`
#[inline]
fn unsupported_io_error() -> io::Result<()> {
    Err(io::Error::from(io::ErrorKind::Unsupported))
}

define!(custom-impl
    /// Sets the foreground color (the color of the text) to `self.0` if it is supported for text
    /// written after this command has been executed
    ///
    /// # `Command` implementation errors
    ///
    /// The `write_to()` method of `Self`'s `Command` implementation will return `Err(err)` in these
    /// cases:
    ///
    /// - `io::Error` with an `ErrorKind` of `Unsupported` when:
    ///     - The terminal doesn't support this command at all
    ///     - The terminal does support this command but not the requested color
    /// - `io::Error` with an `ErrorKind` of `NotFound` when the terminfo database was not found
    /// - `io::Error` with an `ErrorKind` of `InvalidData` when there was an error parsing the
    /// terminfo database
    /// - `io::Error` with an `ErrorKind` of `Other` when there was an error expanding the terminfo
    /// capability with the requested color
    definition: pub struct SetForegroundColor(pub Color),
    capability: String::SetAForeground,
    capability_getter: Entry::get_string,
    size_hint: Some(16),
    unsupported_msg: "Setting the foreground color separately to the background color and/or setting any colours is unsupported in this terminal",
    write_to_impl: |self, entry, capability_data, target| {
        let n_colors = match entry.get_integer(Integer::MaxColors.into()) {
            Some(n) => n,
            None => return unsupported_io_error()
        };

        match self.0 {
            Color::Standard(standard_color) => if n_colors == 8 || n_colors == 16 || n_colors == 256 {
                target.write_all(
                    match expand::expand(capability_data, &[expand::Value::Int(standard_color as i32)]) {
                        Ok(bytes) => bytes,
                        Err(_) => return Err(io::ErrorKind::InvalidData.into()),
                    }.as_slice()
                )?
            } else {
                return unsupported_io_error()
            },
            Color::Bright(bright_color) => if n_colors == 16 || n_colors == 256 {
                target.write_all(
                    match expand::expand(capability_data, &[expand::Value::Int(bright_color as i32)]) {
                        Ok(bytes) => bytes,
                        Err(_) => return Err(io::ErrorKind::InvalidData.into())
                    }.as_slice()
                )?
            } else {
                return unsupported_io_error()
            },
            Color::From256ColorPalette(id) => if n_colors == 256 {
                target.write_all(
                    match expand::expand(capability_data, &[expand::Value::Int(id as i32)]) {
                        Ok(bytes) => bytes,
                        Err(_) => return Err(io::ErrorKind::InvalidData.into())
                    }.as_slice()
                )?
            } else {
                return unsupported_io_error()
            },
            Color::Truecolor(r, g, b) => {
                let truecolor_supported = match entry.get_user_string("Tc") {
                    Some(_) => true,
                    None => match entry.get_user_string("RGB") {
                        Some(_) => true,
                        None => match env::var("COLORTERM") {
                            Ok(colorterm_var) => {
                                if colorterm_var == "truecolor" || colorterm_var == "24bit" {
                                    true
                                } else {
                                    false
                                }
                            },
                            Err(_) => false,
                        }
                    }
                };

                if truecolor_supported {
                    target.write_all(b"\x1b[38;2;")?;

                    let mut itoa_buf = itoa::Buffer::new();

                    target.write_all(itoa_buf.format(r).as_bytes())?;

                    target.write_all(b";")?;

                    target.write_all(itoa_buf.format(g).as_bytes())?;

                    target.write_all(b";")?;

                    target.write_all(itoa_buf.format(b).as_bytes())?;

                    target.write_all(b"m")?;
                } else {
                    return unsupported_io_error()
                }
            }
        }
    },
    is_supported_impl: |self, entry, capability| {
        Colors::from(self.0).is_supported(entry)
    }
);

define!(custom-impl
    definition: pub struct SetBackgroundColor(pub Color),
    capability: String::SetABackground,
    capability_getter: Entry::get_string,
    size_hint: Some(16),
    unsupported_msg: "Setting the background color separately to the foreground color and/or setting any colours is unsupported in this terminal",
    write_to_impl: |self, entry, capability_data, target| {

        let n_colors = match entry.get_integer(Integer::MaxColors.into()) {
            Some(n) => n,
            None => return unsupported_io_error()
        };

        match self.0 {
            Color::Standard(standard_color) => if n_colors == 8 || n_colors == 16 || n_colors == 256 {
                target.write_all(
                    match expand::expand(capability_data, &[expand::Value::Int(standard_color as i32)]) {
                        Ok(bytes) => bytes,
                        Err(_) => return Err(io::ErrorKind::InvalidData.into()),
                    }.as_slice()
                )?
            } else {
                return unsupported_io_error()
            },
            Color::Bright(bright_color) => if n_colors == 16 || n_colors == 256 {
                target.write_all(
                    match expand::expand(capability_data, &[expand::Value::Int(bright_color as i32)]) {
                        Ok(bytes) => bytes,
                        Err(_) => return Err(io::ErrorKind::InvalidData.into())
                    }.as_slice()
                )?
            } else {
                return unsupported_io_error()
            },
            Color::From256ColorPalette(id) => if n_colors == 256 {
                target.write_all(
                    match expand::expand(capability_data, &[expand::Value::Int(id as i32)]) {
                        Ok(bytes) => bytes,
                        Err(_) => return Err(io::ErrorKind::InvalidData.into())
                    }.as_slice()
                )?
            } else {
                return unsupported_io_error()
            },
            Color::Truecolor(r, g, b) => {
                let truecolor_supported = match entry.get_user_string("Tc") {
                    Some(_) => true,
                    None => match entry.get_user_string("RGB") {
                        Some(_) => true,
                        None => match env::var("COLORTERM") {
                            Ok(colorterm_var) => {
                                if colorterm_var == "truecolor" || colorterm_var == "24bit" {
                                    true
                                } else {
                                    false
                                }
                            },
                            Err(_) => false,
                        }
                    }
                };

                if truecolor_supported {
                    target.write_all(b"\x1b[48;2;")?;

                    let mut itoa_buf = itoa::Buffer::new();

                    target.write_all(itoa_buf.format(r).as_bytes())?;

                    target.write_all(b";")?;

                    target.write_all(itoa_buf.format(g).as_bytes())?;

                    target.write_all(b";")?;

                    target.write_all(itoa_buf.format(b).as_bytes())?;

                    target.write_all(b"m")?;
                } else {
                    return unsupported_io_error()
                }
            }
        }
    },
    is_supported_impl: |self, entry, capability_data| {
        Colors::from(self.0).is_supported(entry)
    }
);

impl From<Color> for Colors {
    fn from(value: Color) -> Self {
        match value {
            Color::Standard(_) => Self::Standard,
            Color::Bright(_) => Self::Bright,
            Color::From256ColorPalette(_) => Self::From256ColorPalette,
            Color::Truecolor(_, _, _) => Self::Truecolor,
        }
    }
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum Color {
    /// See docs for [`StandardColor`]
    Standard(StandardColor),
    /// See docs for [`BrightColor`]
    Bright(BrightColor),
    /// Colors that are identified by an id.
    ///
    /// All values of a `u8` are valid id's although some ids are equivalent to the other enum
    /// variants and those enum variants should be preferred:
    /// - [`Self::Standard`] should be preferred to ids 0-7 inclusive
    /// - [`Self::Bright`] should be preferred to ids 8-15 inclusive
    ///
    /// Table with what ids correspond for which color:
    /// ![Table showing what ids in `supaterm::Color::_256ColorPalette` correspond to which colors](https://raw.githubusercontent.com/polyagonal1/supaterm/refs/heads/master/images/256-color-mode-usage.png)
    From256ColorPalette(u8),
    /// An RGB color
    ///
    /// RGB is not supported on all terminals but it is on some terminals
    Truecolor(u8, u8, u8),
}

/// 2x2x2 color cube
///
/// These colors are supported by almost all terminals
#[repr(u8)]
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum StandardColor {
    Black = 0,
    Red = 1,
    Green = 2,
    Yellow = 3,
    Blue = 4,
    Magenta = 5,
    Cyan = 6,
    White = 7,
}

/// Bright versions of the colors in `StandardColor`
#[repr(u8)]
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum BrightColor {
    BrightBlack = 8,
    BrightRed = 9,
    BrightGreen = 10,
    BrightYellow = 11,
    BrightBlue = 12,
    BrightMagenta = 13,
    BrightCyan = 14,
    BrightWhite = 15,
}

pub enum Colors {
    Standard,
    Bright,
    From256ColorPalette,
    Truecolor,
}

impl Capability for Colors {
    fn is_supported(&self, entry: &Entry) -> Self::IsSupportedType {
        let n_colors = match entry.get_integer(Integer::MaxColors.into()) {
            Some(n) => n,
            None => return false,
        };

        match self {
            Self::Standard => if n_colors == 8 || n_colors == 16 || n_colors == 256 {
                true
            } else {
                false
            },
            Self::Bright => if n_colors == 16 || n_colors == 256 {
                true
            } else {
                false
            },
            Self::From256ColorPalette => if n_colors == 256 {
                true
            } else {
                false
            },
            Self::Truecolor => match entry.get_user_string("Tc") {
                Some(_) => true,
                None => match entry.get_user_string("RGB") {
                    Some(_) => true,
                    None => match env::var("COLORTERM") {
                        Ok(colorterm_var) => {
                            if colorterm_var == "truecolor" || colorterm_var == "24bit" {
                                true
                            } else {
                                false
                            }
                        },
                        Err(_) => false,
                    }
                }
            }
        }
    }

    type IsSupportedType = bool;
}