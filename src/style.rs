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