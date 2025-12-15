pub mod command;

pub use command::Command;

use std::io;

pub struct Terminal<I: io::Read, O: io::Write> {
    reader: I,
    writer: O,
    info: terminfo::Database,
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
                    terminfo::Error::Parse => return Err(io::Error::new(io::ErrorKind::InvalidData, "error parsing the data in the database"))
                },
            },
        })
    }

    pub fn into_inner(self) -> (I, O) {
        (
            self.reader,
            self.writer,
        )
    }

    pub fn queue(&mut self, command: impl Command) -> io::Result<()> {
        command.write_to(&self.info, &mut self.writer)
    }

    pub fn queue_all<const N: usize>(&mut self, commands: [&dyn Command; N]) -> io::Result<()> {

        for cmd in commands {
            cmd.write_to(&self.info, self)?;
        }

        Ok(())
    }
}