use crate::macros::{impl_command, impl_display};

/// Moves the cursor onto the next vertical tab spot.
///
/// This is usually equivalent to a newline or is just ignored entirely in modern terminals
pub struct VerticalTab;

impl_command!(b"\x0B", VerticalTab);
impl_display!("\x0B", VerticalTab);