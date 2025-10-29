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
