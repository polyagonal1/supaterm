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

use std::env;
use std::fs;
use std::io;

pub mod command;
pub mod misc;
pub mod style;

mod define_macro;

pub use {
    command::{Capability, Command},
    infoterm,
};

use infoterm::{entry::Entry, search::Search};

pub(crate) use define_macro::*;

/// A wrapper around stdin and stdout that allows queueing of commands
pub struct Terminal<'t> {
    stdin: io::StdinLock<'t>,
    stdout: io::StdoutLock<'t>,
    info: Entry,
    on_drop: Vec<Box<dyn Command + 't>>,
}

impl Drop for Terminal<'_> {
    fn drop(&mut self) {
        for cmd in self.on_drop.iter() {
            let _ = cmd.write_to(&self.info, &mut self.stdout);
        }
    }
}

impl Default for Terminal<'_> {
    #[inline]
    fn default() -> Self {
        Self::new().unwrap()
    }
}

impl io::Write for Terminal<'_> {
    #[inline]
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.stdout.write(buf)
    }
    
    #[inline]
    fn flush(&mut self) -> io::Result<()> {
        self.stdout.flush()
    }
}

impl io::Read for Terminal<'_> {
    #[inline]
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        self.stdin.read(buf)
    }
}

impl<'t> Terminal<'t> {
    /// Creates a new `Terminal` instance which can be used to queue commands
    pub fn new() -> io::Result<Self> {
        Ok(Self {
            stdin: io::stdin().lock(),
            stdout: io::stdout().lock(),
            info: {
                let searcher = Search::standard();

                let entry_name = match env::var("TERM") {
                    Ok(entry_name) => entry_name,
                    Err(error) => match error {
                        env::VarError::NotUnicode(_) => {
                            return Err(io::ErrorKind::InvalidData.into());
                        }
                        env::VarError::NotPresent => return Err(io::ErrorKind::NotFound.into()),
                    },
                };

                let path = searcher
                    .find(entry_name)
                    .ok_or::<io::Error>(io::ErrorKind::NotFound.into())?;

                match Entry::parse(fs::read(path)?.as_slice()) {
                    Ok(entry) => entry,
                    Err(_) => return Err(io::ErrorKind::InvalidData.into()),
                }
            },
            on_drop: Vec::new(),
        })
    }

    /// Checks if some command `cmd` is supported and can be used on this terminal
    pub fn is_capability_supported<C: Capability>(&self, cmd: C) -> bool {
        cmd.is_supported(&self.info)
    }

    /// Queues a command if it is supported (if `cmd.is_supported()` returns true)
    ///
    /// If this function returns `None`, the command is unsupported and did not queue any commands
    /// If this function returns `Some(r)`, the command was supported and the command was queued and
    /// the result has been returned
    ///
    /// This is the same as checking `term.is_cmd_supported(cmd)`, then `queue()`ing the command
    /// based on that.
    #[inline]
    pub fn queue_if_supported(&mut self, cmd: impl Command) -> Option<io::Result<()>> {
        match cmd.is_supported(&self.info) {
            true => Some(cmd.write_to(&self.info, &mut self.stdout)),
            false => None,
        }
    }
    
    #[inline]
    pub fn on_drop<C>(&mut self, cmd: C)
    where
        C: Command + 't
    {
        self.on_drop.push(Box::new(cmd))
    }
    
    // #[inline]
    // pub fn on_drop_multiple<C: IntoIterator<Item = &dyn Command>>(&mut self, cmds: C) {
    //     self.on_drop.extend(cmds)
    // }

    /// Queues a command for execution
    ///
    /// This function may not immediately execute the command. Call `flush()` after to execute all
    /// queued commands
    #[inline]
    pub fn queue(&mut self, command: impl Command) -> io::Result<()> {
        command.write_to(&self.info, &mut self.stdout)
    }
    
    #[inline]
    pub fn queue_all<const N: usize>(&mut self, commands: [&dyn Command; N]) -> io::Result<()> {
        for cmd in commands {
            cmd.write_to(&self.info, &mut self.stdout)?;
        }

        Ok(())
    }
}
