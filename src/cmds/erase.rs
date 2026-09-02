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

use std::io;

/// Clear the display.
///
/// This trait has functions which allow the terminal screen to be cleared 
/// (erased). This is mainly useful when you're in the [alternate screen]. 
/// 
/// For the purposes of this trait, 'display' means the terminal lines which 
/// are currently being rendered, separate from the scrollback, which is what 
/// is not currently being rendered. 
///
/// For erase functions only in the current line, see the [ClearInLine] trait.
///
/// [alternate screen]: crate::EnterAlternateScreen
#[cfg(feature = "erase_in_display")]
pub trait ClearInDisplay {
	/// Clears the whole display. 
	/// 
	/// This does not usually clear scrollback when in the main screen.
	fn clear_screen(&mut self) -> io::Result<()>;
	
	/// Clears from the cursor to the end of the display. 
	/// 
	/// This does not normally clear scrollback when in the main screen.
	#[cfg(feature = "erase_in_display_ext")]
	fn clear_from_cursor_to_end(&mut self) -> io::Result<()>;
	
	/// Clears from the start of the display to the cursor. 
	/// 
	/// Like [ClearInDisplay::clear_from_cursor_to_end], this does not normally 
	/// clear scrollback when in the main screen.
	#[cfg(feature = "erase_in_display_ext")]
	fn clear_from_start_to_cursor(&mut self) -> io::Result<()>;
}

/// Clear the line.
/// 
/// This allows you to clear (erase) in the current line. This would be used in 
/// things like loading bars when in the main screen when you don't want to 
/// clear everything, just re-render a specific line.
#[cfg(feature = "erase_in_line")]
pub trait ClearInLine {
	/// Clears the line the cursor is currently on.
	fn clear_line(&mut self) -> io::Result<()>;

	/// Clears from the cursor to the end of the line the cursor is on.
	#[cfg(feature = "erase_in_line_ext")]
	fn clear_from_cursor_to_line_end(&mut self) -> io::Result<()>;

	/// Clears from the start of the line the cursor is on to the cursor.
	#[cfg(feature = "erase_in_line_ext")]
	fn clear_from_line_start_to_cursor(&mut self) -> io::Result<()>;
}