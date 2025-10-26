use std::io;

pub trait Command {

    /// Attempts to write a (sequence of) ANSI escape code(s) onto `target`
    ///
    /// `term_state` must be modified in a way that matches what gets written onto `target`
    fn queue(&self, target: &mut impl io::Write) -> io::Result<()>;
    
    /// Resets what `write_ansi()` would do for the terminal to go back to normal.
    /// 
    /// It is not an 'undo' button. It does not 'undo' what `write_ansi()` does, just makes it so
    /// that it was as if (for all future writes to the terminal) `write_ansi()` was never called.
    /// 
    /// Should return `None` if resetting the command is not possible or is unapplicable
    /// 
    /// # Examples
    /// ```rust
    /// ```
    fn reset(&self, target: &mut impl io::Write) -> Option<io::Result<()>>;
}