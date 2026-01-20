use crate::define;

use terminfo::capability as cap;

define!(default-no-args
    definition: pub struct EnterAlternateScreen,
    capability: cap::EnterCaMode,
    size_hint: Some(20),
    unsupported_msg: "The alternate screen is unsupported on this terminal",
);

define!(default-no-args
    definition: pub struct ExitAlternateScreen,
    capability: cap::ExitCaMode,
    size_hint: Some(20),
    unsupported_msg: "The alternate screen is unsupported on this terminal",
);