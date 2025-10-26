use std::{fmt, io};

use rustix::{termios, stdio, io::Errno};
use crate::{
    termios::termios_mostly_equal,
    command::Command,
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
}

impl<I: io::Read, O: io::Write> Drop for Terminal<I, O> {
    fn drop(&mut self) {

        let stdin_fd = stdio::stdin();

        let _ = termios::tcflush(stdin_fd, termios::QueueSelector::IOFlush);

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
        })
    }

    /// Queue a command to be written to the terminal
    pub fn queue<C: crate::command::Command>(&mut self, command: C) -> io::Result<C::Output> {
        command.queue(&mut self.stdout)
    }

    /// Calls the `reset()` function on `command`, passing a mutable reference to `self.stdout` to it
    pub fn reset_cmd<C: crate::command::Command>(&mut self, command: C) -> Option<io::Result<C::ResetOutput>> {
        command.reset(&mut self.stdout)
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

// pub struct EnterAlternateScreen;

// impl crate::Command for EnterAlternateScreen {
//
//     type Output = ();
//     type ResetOutput = ();
//
//     fn queue(&self, target: &mut impl io::Write) -> io::Result<Self::Output> {
//
//     }
//
//     fn reset(&self, target: &mut impl io::Write) -> Option<io::Result<Self::ResetOutput>> {
//         None
//     }
// }