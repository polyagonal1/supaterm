pub mod terminal;
pub mod termios;
pub mod command;
pub mod logging;
pub mod style;
pub mod erase;
pub mod cursor;

pub use command::Command;
pub use terminal::Terminal;

pub(crate) mod macros {
    macro_rules! impl_command {
        ($val:expr, $type:ty) => {
            impl $crate::Command for $type {
                
                fn queue(&self, target: &mut impl ::std::io::Write) -> ::std::io::Result<()> {
                    target.write_all($val)
                }
                
                fn reset(&self, _target: &mut impl ::std::io::Write) -> ::std::option::Option<std::io::Result<()>> {
                    None
                }
            }
        };
        ($val:expr, $reset:expr, $type:ty) => {
            impl $crate::Command for $type {
                
                fn queue(&self, target: &mut impl ::std::io::Write) -> ::std::io::Result<()> {
                    target.write_all($val)
                }
                
                fn reset(&self, target: &mut impl ::std::io::Write) -> ::std::option::Option<std::io::Result<()>> {
                    target.write_all($reset)
                }
            }
        }
    }
    
    macro_rules! impl_display {
        ($val:expr, $type:ty) => {
            impl ::std::fmt::Display for $type {
                fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                    f.write_str($val)
                }
            }
        }
    }
    
    pub(crate) use impl_command;
    pub(crate) use impl_display;
}