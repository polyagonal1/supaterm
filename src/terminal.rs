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

use std::{
    io,
    fmt,
};

use rustix::{
    termios,
    stdio,
    io::Errno
};

use crate::{
    termios::termios_mostly_equal,
    command::{Command},
};

pub enum SetAttrsError {
    Unsupported(termios::Termios),
    Other(Errno),
}

impl From<Errno> for SetAttrsError {
    fn from(err: Errno) -> SetAttrsError {
        SetAttrsError::Other(err)
    }
}

/// A handle to basically everything supaterm has to offer
///
/// # Examples
/// todo
pub struct Terminal<I: io::Read, O: io::Write> {
    stdin: I,
    stdout: O,
    original_termios: termios::Termios,
    pub commands_to_reset: Vec<Box<dyn Command>>,
}

impl<I: io::Read, O: io::Write> Drop for Terminal<I, O> {
    fn drop(&mut self) {

        for cmd in self.commands_to_reset.iter() {
            let _ = cmd.queue(&mut self.stdout);
        }

        let _ = self.stdout.flush();

        let stdin_fd = stdio::stdin();

        let _ = termios::tcdrain(stdin_fd);

        let _ = termios::tcsetattr(stdin_fd, termios::OptionalActions::Now, &self.original_termios);
    }
}

impl<I: io::Read, O: io::Write> io::Write for Terminal<I, O> {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.stdout.write(buf)
    }

    fn flush(&mut self) -> io::Result<()> {
        self.stdout.flush()
    }
}

impl<I: io::Read, O: io::Write> io::Read for Terminal<I, O> {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        self.stdin.read(buf)
    }
}

impl<'a> Terminal<io::StdinLock<'a>, io::StdoutLock<'a>> {
    
    /// Creates a new `Terminal` using `std::io::StdinLock` and `std::io::StdoutLock` for I/O
    pub fn new() -> io::Result<Self> {

        let stdin_fd = stdio::stdin();

        Ok(Self {
            stdin: io::stdin().lock(),
            stdout: io::stdout().lock(),
            original_termios: termios::tcgetattr(stdin_fd)?,
            commands_to_reset: Vec::new(),
        })
    }
}
impl<I: io::Read, O: io::Write> Terminal<I, O> {
    
    /// Creates a new `Terminal`, with custom input and output objects
    ///
    /// `stdin` must implement `io::Read` and `stdout` must implement `io::Write`
    pub fn new_custom(stdin: I, stdout: O) -> io::Result<Self> {
        
        let stdin_fd = stdio::stdin();
        
        Ok(Self {
            stdin,
            stdout,
            original_termios: termios::tcgetattr(stdin_fd)?,
            commands_to_reset: Vec::new(),
        })
    }

    /// Queue a command to be written to the terminal
    pub fn queue<C: Command>(&mut self, command: C) -> io::Result<()> {
        if let Some(reset_cmd) = command.reset_cmd() {

            self.commands_to_reset.push(reset_cmd);
        }

        command.queue(&mut self.stdout)
    }
    pub fn queue_all<const N: usize>(&mut self, commands: [&dyn Command; N]) -> io::Result<()> {

        let mut size: usize = 0;

        commands
            .iter()
            .for_each(|cmd| {
                size += cmd.size_hint().unwrap_or(64)
            });

        let mut bytes = Vec::with_capacity(size);

        commands
            .iter()
            .try_for_each(|cmd| {
                cmd.queue(&mut bytes)?;

                if let Some(reset_cmd) = cmd.reset_cmd() {
                    self.commands_to_reset.push(reset_cmd);
                }

                Ok::<(), io::Error>(())
            })?;

        Ok(self.stdout.write_all(&bytes)?)

    }
    
    /// Sets the terminal's attributes to raw mode.
    /// 
    /// In raw mode, input is available a byte at a time, echoing is disabled, and special terminal
    /// input and output codes are disabled.
    pub fn enable_raw_mode(&self, optional_actions: termios::OptionalActions) -> io::Result<()> {

        let stdin_fd = stdio::stdin();

        let mut attrs = termios::tcgetattr(stdin_fd)?;
        attrs.make_raw();

        termios::tcsetattr(stdin_fd, optional_actions, &attrs)?;

        Ok(())
    }
    
    /// Sets the terminal's attributes back to how they were at the start (not raw mode)
    pub fn disable_raw_mode(&self, optional_actions: termios::OptionalActions) -> io::Result<()> {

        let stdin_fd = stdio::stdin();

        termios::tcsetattr(stdin_fd, optional_actions, &self.original_termios)?;

        Ok(())
    }
    
    /// Query's if raw mode is currently enabled
    pub fn query_raw_mode(&self) -> io::Result<bool> {

        let stdin_fd = stdio::stdin();

        let current_attrs = termios::tcgetattr(stdin_fd)?;

        Ok(!termios_mostly_equal(&current_attrs, &self.original_termios))
    }
    
    /// Sets terminal attributes to a specific `Termios` value
    pub fn set_term_attrs(&self, attrs: termios::Termios, optional_actions: termios::OptionalActions) -> Result<(), SetAttrsError> {

        let stdin_fd = stdio::stdin();

        let original_attrs = termios::tcgetattr(stdin_fd)?;

        termios::tcsetattr(stdin_fd, optional_actions, &attrs)?;

        let attrs_after_being_set = termios::tcgetattr(stdin_fd)?;

        if !termios_mostly_equal(&original_attrs, &attrs_after_being_set) {
            Err(SetAttrsError::Unsupported(attrs_after_being_set))
        } else {
            Ok(())
        }
    }
    
    /// Gets the current terminal attributes
    pub fn get_term_attrs(&self) -> io::Result<termios::Termios> {
        
        let stdin_fd = stdio::stdin();
        
        Ok(termios::tcgetattr(stdin_fd)?)
    }
}

/// Fires the terminal bell. Usually produces a sound or flashes the title bar to indicate an error.
///
/// More information about the terminal bell can be found [here][more-info]
///
/// [more-info]: https://rosettacode.org/wiki/Terminal_control/Ringing_the_terminal_bell
pub struct TerminalBell;

impl Command for TerminalBell {
    fn queue(&self, target: &mut dyn io::Write) -> io::Result<()> {

        target.write_all(b"\x07")
    }

    fn size_hint(&self) -> Option<usize> {
        Some(1)
    }

    fn reset_cmd(&self) -> Option<Box<dyn Command>> {
        None
    }
}

impl fmt::Display for TerminalBell {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("\x07")
    }
}

/// Enables the alternate terminal buffer
///
/// When activated, the terminal will hide the contents of the main screen and show the alternate
/// screen. When deactivated, the terminal will clear the contents of the alternate screen and switch
/// back to the main screen
///
/// This struct implements `Command::reset_cmd()`, so when `Drop` on `self` is called, the alternate
/// screen will be exited
pub struct EnterAlternateScreen;
impl Command for EnterAlternateScreen {

    fn queue(&self, target: &mut dyn io::Write) -> io::Result<()> {
        target.write_all(b"\x1b[?1049h")
    }

    fn size_hint(&self) -> Option<usize> {
        Some(8)
    }

    fn reset_cmd(&self) -> Option<Box<dyn Command>> {
        Some(Box::new(LeaveAlternateScreen))
    }
}

impl fmt::Display for EnterAlternateScreen {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("\x1b[?1049h")
    }
}

pub struct LeaveAlternateScreen;

impl Command for LeaveAlternateScreen {

    fn queue(&self, target: &mut dyn io::Write) -> io::Result<()> {
        target.write_all(b"\x1b[?1049l")
    }

    fn size_hint(&self) -> Option<usize> {
        Some(8)
    }

    fn reset_cmd(&self) -> Option<Box<dyn Command>> {
        None
    }
}

impl fmt::Display for LeaveAlternateScreen {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("\x1b[?1049l")
    }
}
