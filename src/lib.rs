use std::io;

pub mod command;
pub mod style;

pub use {
    command::Command,
    terminfo
};

/// A wrapper around a reader and writer that allows queueing of commands
pub struct Terminal<I: io::Read, O: io::Write> {
    reader: I,
    writer: O,
    info: terminfo::Database,
    terminfo_ctx: terminfo::expand::Context,
}

impl<'a, 'b> Default for Terminal<io::StdinLock<'a>, io::StdoutLock<'b>> {
    fn default() -> Self {
        Self::new(io::stdin().lock(), io::stdout().lock()).unwrap()
    }
}

impl<I: io::Read, O: io::Write> io::Write for Terminal<I, O> {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.writer.write(buf)
    }

    fn flush(&mut self) -> io::Result<()> {
        self.writer.flush()
    }
}

impl<I: io::Read, O: io::Write> io::Read for Terminal<I, O> {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        self.reader.read(buf)
    }
}

impl<I: io::Read, O: io::Write> Terminal<I, O> {
    #[inline]
    pub fn new(reader: I, writer: O) -> Result<Self, io::Error> {
        Ok(Self {
            reader,
            writer,
            info: match terminfo::Database::from_env() {
                Ok(info) => info,
                Err(error) => match error {
                    terminfo::Error::Io(io_err) => return Err(io_err),
                    terminfo::Error::Expand(_) => panic!("there should not be an expansion error when creating a database, right?"),
                    terminfo::Error::NotFound => panic!("if the database is not found, then this device is probably (currently) unsupported"),
                    terminfo::Error::Parse => return Err(io::Error::new(io::ErrorKind::InvalidData, "error parsing the data in the database, although, I didn't think any parsing would happen during database creation."))
                },
            },
            terminfo_ctx: terminfo::expand::Context::default()
        })
    }

    pub fn into_inner(self) -> (I, O) {
        (
            self.reader,
            self.writer,
        )
    }

    pub fn queue(&mut self, command: impl Command) -> io::Result<()> {
        command.write_to(&self.info, &mut self.terminfo_ctx, &mut self.writer)
    }

    pub fn queue_all<const N: usize>(&mut self, commands: [&dyn Command; N]) -> io::Result<()> {

        for cmd in commands {
            cmd.write_to(&self.info, &mut self.terminfo_ctx, &mut self.writer, )?;
        }

        Ok(())
    }
}

macro_rules! define {
    (default-no-args
        $(#[$attrs:meta])*
        definition: $visible:vis struct $typ:ident,
        capability: $capability:ty,
        size_hint: $size:expr,
        unsupported_msg: $unsupported_msg:literal $(,)?
        $(--add-command-implementation-errors-docs $($placeholder:tt)? )?
    ) => {
        define!(custom-impl
            $(#[$attrs])*
            definition: $visible struct $typ,
            capability: $capability,
            size_hint: $size,
            unsupported_msg: $unsupported_msg,
            implementation: |&self, _database, capability, ctx, target| {

                match capability.expand().with(ctx).to(target) {
                    // writing to `target` was successful
                    Ok(_) => (),
                    // writing to `target` was unsuccessful
                    Err(error) => return match error {
                        ::terminfo::Error::Parse => Err(::std::io::Error::new(
                            ::std::io::ErrorKind::InvalidData,
                            error
                        )),
                        ::terminfo::Error::NotFound => Err(::std::io::Error::new(
                            ::std::io::ErrorKind::NotFound,
                            error
                        )),
                        ::terminfo::Error::Expand(_) => Err(::std::io::Error::new(
                            ::std::io::ErrorKind::Other,
                            error
                        )),
                        ::terminfo::Error::Io(io_error) => Err(io_error)
                    }
                }
            }
            $(--add-command-implementation-errors-docs $($placeholder)?)?
        );
    };
    (custom-impl
        $(#[$attrs:meta])*
        definition: $visible:vis struct $typ:ident $(( $( $args:tt  )+ ))?,
        capability: $capability:ty,
        size_hint: $size_hint:expr,
        unsupported_msg: $unsupported_msg:literal,
        implementation: |&$self_var_name:ident, $database_var_name:ident, $cap_var_name:ident, $ctx_var_name:ident, $target_var_name:ident $(,)?| $implementation:block $(,)?
        $(--add-command-implementation-errors-docs $($placeholder:tt)? )?
    ) => {

        $(#[$attrs])*
        #[doc = concat!(
            $($($placeholder)?
                "# `Command` implementation errors\n",
                "Returns:\n",
                "- `Err(io::Error)` with an `ErrorKind` of `Unsupported` when the terminal does not support this command\n",
                "- `Err(io::Error)` with an `ErrorKind` of `InvalidData` when there was an error parsing the terminfo library\n",
                "- `Err(io::Error)` with an `ErrorKind` of `NotFound` when the terminfo entry for this terminal was not found\n",
                "- `Err(io::Error)` with an `ErrorKind` of `Other` when there was an error expanding a parameterised terminfo capability.\n",
                "May also return any other `io::Error`",
            )?
        )]
        #[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
        $visible struct $typ $(( $($args)+ ))?;

        impl crate::Command for $typ {
            fn size_hint(&self) -> Option<usize> {
                $size_hint
            }

            fn write_to(
                $self_var_name: &Self,
                $database_var_name: &::terminfo::Database,
                #[allow(unused)] $ctx_var_name: &mut ::terminfo::expand::Context,
                #[allow(unused)] $target_var_name: &mut dyn ::std::io::Write
            ) -> ::std::io::Result<()> {
                match $database_var_name.get::<$capability>() {
                    // this command is supported
                    Some($cap_var_name) => $implementation,
                    // this command is unsupported
                    None => return Err(::std::io::Error::new(
                        ::std::io::ErrorKind::Unsupported,
                        $unsupported_msg
                    ))
                }
                Ok(())
            }
        }
    };
}

pub(crate) use {
    define,
};