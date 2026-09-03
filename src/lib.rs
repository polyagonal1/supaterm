/*
    chromoterm – terminal manipulation library
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

//! # Supaterm
//! 
//! This is a low-level terminal manipulation library that aims to be simple to 
//! use and ergonomic and fast. ('Low-level' meaning that it does not provide 
//! any sort of TUI, just the commands used to make the TUI.)
//! 
//! This crate does not currently support Windows, but I would like to add 
//! support at some point in the future.

mod cmds;
#[cfg(feature = "raw_mode")]
pub mod terminal_mode;
#[cfg(feature = "window_size")]
pub mod winsize;

pub use cmds::*;
#[cfg(feature = "raw_mode")]
pub use terminal_mode::raw;
