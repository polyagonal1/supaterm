use std::io;

pub trait Command {

    /// Writes the command onto `target`
    fn queue(&self, target: &mut dyn io::Write) -> io::Result<()>;

    /// Returns an optional hint about the size of how much will be written to `target` in the
    /// `queue()` method for optimisation purposes.
    ///
    /// If this returns `None`, the size will be assumed to be 64 bytes
    fn size_hint(&self) -> Option<usize>;

    /// Returns the command that would reset this command.
    ///
    /// This does not 'undo' the command; but makes it that it was as if this command hadn't been
    /// called at all (so in some cases it would act like an 'undo', but that isn't it's sole
    /// purpose).
    ///
    /// Returns `None` when resetting the command is impossible or inapplicable or when calling this
    /// command with `queue()` resets (a part of) the terminal back to how it started.
    fn reset_cmd(&self) -> Option<Box<dyn Command>>;
}

impl<T: Command> Command for &T {
    fn queue(&self, target: &mut dyn io::Write) -> io::Result<()> {
        (*self).queue(target)
    }

    fn size_hint(&self) -> Option<usize> {
        (*self).size_hint()
    }

    fn reset_cmd(&self) -> Option<Box<dyn Command>> {
        (*self).reset_cmd()
    }
}

impl<T: Command> Command for Box<T> {
    fn queue(&self, target: &mut dyn io::Write) -> io::Result<()> {
        (**self).queue(target)
    }

    fn size_hint(&self) -> Option<usize> {
        (**self).size_hint()
    }

    fn reset_cmd(&self) -> Option<Box<dyn Command>> {
        (**self).reset_cmd()
    }
}