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

use rustix::{termios};

pub use termios::{Termios, OptionalActions};

pub fn termios_mostly_equal(a: &Termios, b: &Termios) -> bool {
    a.input_modes == b.input_modes
        && a.output_modes == b.output_modes
        && a.control_modes == b.control_modes
        && a.local_modes == b.local_modes
        && a.control_modes == b.control_modes
}
