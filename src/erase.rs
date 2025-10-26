use crate::macros::{impl_command, impl_display};

/// Equivalent to hitting the backspace key on a keyboard
pub struct Backspace;

impl_command!(b"\x08", Backspace);
impl_display!("\x08", Backspace);

/// Like pressing the delete key on a keyboard (not to be confused with backspace)
pub struct DeleteChar;

impl_command!(b"\x7F", DeleteChar);
impl_display!("\x7F", DeleteChar);