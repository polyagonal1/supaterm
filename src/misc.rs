/*
    supaterm – terminal manipulation library allowing use of colored text and other functionality is planned
    Copyright (C) 2026  @polyagonal1

    This program is free software: you can redistribute it and/or modify
    it under the terms of the GNU General Public License as published by
    the Free Software Foundation, either version 3 of the License, or
    (at your option) any later version.

    This program is distributed in the hope that it will be useful,
    but WITHOUT ANY WARRANTY; without even the implied warranty of
    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
    GNU General Public License for more details.

    You should have received a copy of the GNU General Public License
    along with this program.  If not, see <https://www.gnu.org/licenses/>
*/

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