/*
    supaterm – terminal manipulation library
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

#[cfg(any(feature = "window_size", feature = "raw_mode"))]
use rustix::{
	fd::BorrowedFd,
	stdio,
};
#[cfg(feature = "raw_mode")]
use crate::raw::RawModeEnabled;

/// Keeps track of terminal state
pub struct TerminalState<'tty> {
	#[cfg(feature = "raw_mode")]
	pub(crate) raw_mode_enabled: RawModeEnabled,

	#[cfg(any(feature = "window_size", feature = "raw_mode"))]
	pub(crate) tty_fd: BorrowedFd<'tty>,
	
	/// Marker to prevent a compile error about usage of the `'tty` lifetime 
	/// when neither `raw_mode` or `window_size` cargo features are enabled.
	#[allow(unused)]
	marker: &'tty (),
}

impl<'tty> Default for TerminalState<'tty> {
	fn default() -> Self {
		Self {
			#[cfg(feature = "raw_mode")]
			raw_mode_enabled: RawModeEnabled::Disabled,

			#[cfg(any(feature = "window_size", feature = "raw_mode"))]
			tty_fd: stdio::stdin(),

			marker: &(),
		}
	}
}