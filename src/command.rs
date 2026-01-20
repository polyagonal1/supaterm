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

use std::io;

pub trait Command: Capability {
    fn size_hint(&self) -> Option<usize>;

    fn write_to(
        &self,
        database: &terminfo::Database,
        ctx: &mut terminfo::expand::Context,
        target: &mut dyn io::Write
    ) -> io::Result<()>;
}

pub trait Capability {
    fn is_supported(
        &self,
        database: &terminfo::Database,
    ) -> bool;
}