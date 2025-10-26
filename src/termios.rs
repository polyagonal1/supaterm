use rustix::{termios};

use termios::{Termios, OptionalActions};

pub fn termios_mostly_equal(a: &Termios, b: &Termios) -> bool {
    a.input_modes == b.input_modes
        && a.output_modes == b.output_modes
        && a.control_modes == b.control_modes
        && a.local_modes == b.local_modes
        && a.control_modes == b.control_modes
}