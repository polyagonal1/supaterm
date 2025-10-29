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

use std::{fmt, io};
    
use crate::command::Command;
pub struct Print<T: fmt::Display>(pub T);

impl<T: fmt::Display> Command for Print<T> {

    fn queue(&self, target: &mut dyn io::Write) -> io::Result<()> {
        write!(target, "{}\r\n", self.0)
    }
    
    fn size_hint(&self) -> Option<usize> {
        Some(self.0.to_string().len() + 2)
    }

    fn reset_cmd(&self) -> Option<Box<dyn Command>> {
        None
    }
}
