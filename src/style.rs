pub use print::*;

mod print {
    
    use std::{fmt, io};
    
    use crate::command::Command;
    pub struct Print<T: fmt::Display>(pub T);

    impl<T: fmt::Display> Command for Print<T> {

        fn queue(&self, target: &mut impl io::Write) -> io::Result<()> {
            write!(target, "{}\r\n", self.0)
        }
        
        fn reset(&self, target: &mut impl io::Write) -> Option<io::Result<()>> {
            None
        }
    }

    impl<T: fmt::Display, const N: usize> Command for [Print<T>; N] {
    
        fn queue(&self, target: &mut impl io::Write) -> io::Result<()> {
            self.iter().try_for_each(|print_cmd| {
                write!(target, "{}", print_cmd.0)
            })
        }
        
        fn reset(&self, target: &mut impl io::Write) -> Option<io::Result<()>> {
            None
        }
    }
}