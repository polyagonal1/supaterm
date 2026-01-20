use {
    crate::{
        define,
        Command,
        Capability
    },

    std::io,

    terminfo::{capability as cap, Database},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Writeln<'a>(pub &'a [u8]);

impl<'a> Command for Writeln<'a> {
    fn size_hint(&self) -> Option<usize> {
        // add 1 because `write_to()` appends a newline
        Some(self.0.len() + 1)
    }

    fn write_to(&self, _database: &Database, _ctx: &mut terminfo::expand::Context, target: &mut dyn io::Write) -> io::Result<()> {

        target.write_all(self.0)?;

        target.write_all(b"\r\n")?;

        Ok(())
    }
}

impl<'a> Capability for Writeln<'a> {
    fn is_supported(&self, _: &Database) -> bool {
        true
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Write<'a>(pub &'a [u8]);

impl<'a> Command for Write<'a> {
    fn size_hint(&self) -> Option<usize> {
        Some(self.0.len())
    }

    fn write_to(
        &self,
        _database: &Database,
        _ctx: &mut terminfo::expand::Context,
        target: &mut dyn io::Write
    ) -> io::Result<()> {

        target.write_all(self.0)?;

        Ok(())
    }
}

impl<'a> Capability for Write<'a> {
    fn is_supported(&self, _: &Database) -> bool {
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
    capability: cap::ExitAttributeMode,
    size_hint: Some(8),
    unsupported_msg: "Resetting the current style (terminfo cap-name 'sgr0' in terminfo) is unsupported on this terminal",
    --add-command-implementation-errors-docs
);

define!(default-no-args
    /// Enables bold mode
    definition: pub struct SetBold,
    capability: cap::EnterBoldMode,
    size_hint: Some(8),
    unsupported_msg: "Bold mode (terminfo cap-name 'bold') is unsupported in this terminal",
    --add-command-implementation-errors-docs
);

define!(default-no-args
    /// Makes text that is written after this command underlined
    definition: pub struct SetUnderline,
    capability: cap::EnterUnderlineMode,
    size_hint: Some(8),
    unsupported_msg: "Underline mode (terminfo cap-name 'smul') is unsupported in this terminal",
    --add-command-implementation-errors-docs
);

define!(default-no-args
    /// Disables underlined mode (see [EnterUnderline])
    definition: pub struct ResetUnderline,
    capability: cap::ExitUnderlineMode,
    size_hint: Some(8),
    unsupported_msg: "Resetting underline mode explicitly (terminfo cap-name 'rmul') is unsupported in this terminal",
    --add-command-implementation-errors-docs
);

define!(default-no-args
    /// Switches the foreground and background colours for text written after this command has been executed
    definition: pub struct SetReverseMode,
    capability: cap::EnterReverseMode,
    size_hint: Some(8),
    unsupported_msg: "Reverse video mode (terminfo cap-name 'rev') is unsupported in this terminal",
    --add-command-implementation-errors-docs
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
    capability: cap::EnterStandoutMode,
    size_hint: Some(20),
    unsupported_msg: "Standout mode (terminfo cap-name 'smso') is unsupported in this terminal",
    --add-command-implementation-errors-docs
);

define!(default-no-args
    /// Disables standout mode (see [SetStandoutMode])
    definition: pub struct ResetStandoutMode,
    capability: cap::ExitStandoutMode,
    size_hint: Some(20),
    unsupported_msg: "Resetting standout mode explicitly (terminfo cap-name 'rmso') is unsupported in this terminal",
    --add-command-implementation-errors-docs
);

define!(default-no-args
    /// Makes text written after this command appear dimmed
    definition: pub struct SetDim,
    capability: cap::EnterDimMode,
    size_hint: Some(8),
    unsupported_msg: "Dim mode (terminfo cap-name 'dim') is unsupported in this terminal",
    --add-command-implementation-errors-docs
);

define!(default-no-args
    /// Makes text written after this command blink
    definition: pub struct SetBlinking,
    capability: cap::EnterBlinkMode,
    size_hint: Some(8),
    unsupported_msg: "Blinking mode (terminfo cap-name 'blink' is unsupported in this terminal",
    --add-command-implementation-errors-docs
);

define!(default-no-args
    /// Makes text written after this command invisible
    definition: pub struct SetInvisible,
    capability: cap::EnterSecureMode,
    size_hint: Some(8),
    unsupported_msg: "Invisible mode (terminfo cap-name 'invis') is unsupported in this terminal",
    --add-command-implementation-errors-docs
);

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
    capability: cap::SetAForeground,
    size_hint: Some(16),
    unsupported_msg: "Setting the foreground color separately to the background color and/or setting any colours is unsupported in this terminal",
    write_to_impl: |self, database, capability, ctx, target| {

        // how many colors are supported and is assumed to be the maximum color value you can have
        let colors = database.get::<cap::MaxColors>()
            .ok_or(io::Error::new(
                io::ErrorKind::Unsupported,
                "No colors! What age is this terminal from?"
            ))?.0;

        let requested_color = self.0.as_u8();

        // if the color requested is supported
        //
        // colors - 1 is used because `colors` is the number of distinct colors available. For example,
        // if `colors == 8`, the values `0..=7` are the values that can be used in `Expansion::color()`
        if (0..colors).contains(&(requested_color as i32)) {
            // the terminal supports the color
            match capability.expand().color(requested_color).to(target) {
                Ok(_) => (),
                Err(error) => return match error {
                    terminfo::Error::Io(io_error) => Err(io_error),
                    terminfo::Error::NotFound => Err(io::Error::new(io::ErrorKind::NotFound, error)),
                    terminfo::Error::Parse => Err(io::Error::new(io::ErrorKind::InvalidData, error)),
                    terminfo::Error::Expand(_) => Err(io::Error::new(io::ErrorKind::Other, error)),
                }
            }
        } else {
            // the terminal doesn't support the requested color
            return Err(io::Error::new(
                io::ErrorKind::Unsupported,
                format!("The terminal only supports {colors} colors which is less than the requested color: {:?}", self.0).as_str()
            ))
        }
    },
    is_supported_impl: |self, database, capability| {
        // how many colors are supported and is assumed to be the maximum color value you can have
        let colors = match database.get::<cap::MaxColors>() {
            Some(max_colors) => max_colors.0,
            None => return false,
        };

        let requested_color = self.0.as_u8();

        (0..colors).contains(&(requested_color as i32))
    }
);

define!(custom-impl
    definition: pub struct SetBackgroundColor(pub Color),
    capability: cap::SetABackground,
    size_hint: Some(16),
    unsupported_msg: "Setting the background color separately to the foreground color and/or setting any colours is unsupported in this terminal",
    write_to_impl: |self, database, capability, ctx, target| {

        // how many colors are supported and is assumed to be the maximum color value you can have
        let colors = database.get::<cap::MaxColors>()
            .ok_or(io::Error::new(
                io::ErrorKind::Unsupported,
                "No colors! What age is this terminal from?"
            ))?.0;

        let requested_color = self.0.as_u8();

        // if the color requested is supported
        //
        // colors - 1 is used because `colors` is the number of distinct colors available. For example,
        // if `colors == 8`, the values `0..=7` are the values that can be used in `Expansion::color()`
        if (0..colors).contains(&(requested_color as i32)) {
            // the terminal supports the color
            match capability.expand().color(requested_color).to(target) {
                Ok(_) => (),
                Err(error) => return match error {
                    terminfo::Error::Io(io_error) => Err(io_error),
                    terminfo::Error::NotFound => Err(io::Error::new(io::ErrorKind::NotFound, error)),
                    terminfo::Error::Parse => Err(io::Error::new(io::ErrorKind::InvalidData, error)),
                    terminfo::Error::Expand(_) => Err(io::Error::new(io::ErrorKind::Other, error)),
                }
            }
        } else {
            // the terminal doesn't support the requested color
            return Err(io::Error::new(
                io::ErrorKind::Unsupported,
                format!("The terminal only supports {colors} colors which is less than the requested color: {:?}", self.0).as_str()
            ))
        }
    },
    // if your IDE reports the error "Missing lifetime specifier" here when you explicitly set the type, it is wrong
    // and is a limitation of rust-analyzer. If you compile the code directly with cargo or rustc, you won't get this
    // error.
    is_supported_impl: |self, database, capability: cap::SetABackground| {
        // how many colors are supported and is assumed to be the maximum color value you can have
        let colors = match database.get::<cap::MaxColors>() {
            Some(max_colors) => max_colors.0,
            None => return false,
        };

        let requested_color = self.0.as_u8();

        (0..colors).contains(&(requested_color as i32))
    }
);

/// This is documentation here
///
/// Note: More variants are planned to be added
#[non_exhaustive]
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum Color {
    // standard colors
    Black,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    White,

    // standard bright colors
    BrightBlack,
    BrightRed,
    BrightGreen,
    BrightYellow,
    BrightBlue,
    BrightMagenta,
    BrightCyan,
    BrightWhite,

    /// A color described by an id
    ///
    /// All values of a `u8` (0-255 inclusive) all represent valid colors
    ///
    /// Here is a table describing the mapping between the IDs and their colors:
    ///
    /// TODO: Add the table to github
    ///
    /// ![testtest](https://github.com/polyagonal1/supaterm/raw/refs/heads/master/images/256-color-mode.png)
    ColorById(u8)
}

impl Color {
    #[inline]
    pub(super) const fn as_u8(&self) -> u8 {
        match self {
            Color::Black => 0,
            Color::Red => 1,
            Color::Green => 2,
            Color::Yellow => 3,
            Color::Blue => 4,
            Color::Magenta => 5,
            Color::Cyan => 6,
            Color::White => 7,

            Color::BrightBlack => 8,
            Color::BrightRed => 9,
            Color::BrightGreen => 10,
            Color::BrightYellow => 11,
            Color::BrightBlue => 12,
            Color::BrightMagenta => 13,
            Color::BrightCyan => 14,
            Color::BrightWhite => 15,

            Color::ColorById(id) => *id
        }
    }
}