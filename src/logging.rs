use std::{io, fmt};

pub use logging_options::*;

pub mod logging_options {

    /// Controls how an error is formatted in error logging functions
    pub struct ErrorLoggingOptions {
        /// Whether the `source()` of the error should be included. If it is, include options for
        /// how the source is formatted
        pub source: Option<Box<ErrorSourceLoggingOptions>>,
        /// Whether the name of the error type should be included
        pub include_type_name: bool,
        /// Whether the location of the caller should be included
        pub include_location: bool,
    }

    pub struct ErrorSourceLoggingOptions {
        pub include_msg: bool,
    }

    impl From<ErrorLoggingOptionsPresets> for ErrorLoggingOptions {
        fn from(preset: ErrorLoggingOptionsPresets) -> Self {
            match preset {
                ErrorLoggingOptionsPresets::Simple => ErrorLoggingOptions {
                    source: None,
                    include_type_name: false,
                    include_location: false,
                },
                ErrorLoggingOptionsPresets::Standard => ErrorLoggingOptions {
                    source: Some(Box::new(ErrorSourceLoggingOptions {
                        include_msg: true,
                    })),
                    include_type_name: true,
                    include_location: true,
                },
            }
        }
    }

    impl Default for ErrorLoggingOptions {
        fn default() -> Self {
            Self::from(ErrorLoggingOptionsPresets::Standard)
        }
    }

    /// Presets for `ErrorLoggingOptions`
    pub enum ErrorLoggingOptionsPresets {
        /// Corresponds to:
        /// ```rust
        /// # use terminal_rs::logging::logging_options::ErrorLoggingOptions;
        /// # fn main() {
        /// # let _ = {
        ///  ErrorLoggingOptionsPresets::Simple => ErrorLoggingOptions {
        ///     source: None,
        ///     include_type_name: false,
        ///     include_location: false,
        /// },
        /// # };
        /// # }
        /// ```
        Simple,
        /// Corresponds to:
        /// ```rust
        /// # use terminal_rs::logging::logging_options::ErrorLoggingOptions;
        /// # fn main() {
        /// # use terminal_rs::logging::logging_options::ErrorSourceLoggingOptions;
        /// # let _ = {
        /// ErrorLoggingOptions {
        ///     source: Some(Box::new(ErrorSourceLoggingOptions {
        ///         include_msg: true,
        ///     })),
        ///     include_type_name: true,
        ///     include_location: true
        /// };
        /// # }
        /// # }
        /// ```
        Standard,
    }
}

#[track_caller]
pub fn log(target: &mut impl io::Write, message: impl fmt::Display) -> io::Result<()> {
    let caller = std::panic::Location::caller();

    let message = format!(
        "[{}:{}:{}] {}\n",
        caller.file(),
        caller.line(),
        caller.column(),
        message,
    );

    target.write_all(message.as_bytes())
}

#[track_caller]
pub fn log_err<E: std::error::Error>(
    target: &mut impl io::Write,
    error: E,
    error_options: Option<ErrorLoggingOptions>,
) -> io::Result<()> {
    let caller = std::panic::Location::caller();

    let mut message = String::new();

    let options: ErrorLoggingOptions = error_options.unwrap_or_default();

    if options.include_location {
        message.push_str(format!(
            "[{}:{}:{}] ",
            caller.file(),
            caller.line(),
            caller.column(),
        ).as_str());
    }

    message.push_str("ERROR: ");

    if options.include_type_name {
        message.push_str(format!("{}: ", std::any::type_name::<E>()).as_str());
    }

    message.push_str(error.to_string().as_str());

    if let Some(source_options) = options.source
        && let Some(source) = error.source() {

        let cause_msg: String = if source_options.include_msg {
            format!(" (caused by {}: {})", std::any::type_name::<E>(), source)
        } else {
            format!(" (caused by {})", std::any::type_name::<E>())
        };

        message.push_str(cause_msg.as_str())
    }

    target.write_all(message.as_bytes())
}