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

//! Module that allows the switching between the alternate screen (also called
//! the alternate buffer) and the main screen.
//!
//! By default, when you open a terminal, you are in the main screen.
//!
//! You can switch between the alternate screen and the main screen by using
//! the [`ScreenController`] trait. See that trait's docs for more info.
//!
//! You can make sure the alternate screen is always disabled when the program
//! exits using the [`ScreenControllerGuard`] struct.
//!
//! # What is the alternate screen?
//!
//! The alternate screen has a completetly different buffer to the main screen
//! (an 'alternate' buffer). When you enter the alternate screen, you will
//! switch to viewing that alternate buffer, rather than the main buffer.
//!
//! The alternate buffer will almost always be empty when you switch to it
//! which means all the previous shell output will disappear. The user cannot
//! scroll up or down in the alternate screen like you would in the main
//! screen. It does mean however that the program has access to all the
//! terminal's cells. This makes it easier to write full-screen terminal
//! applications such as text editors.

use std::{
	ops::{Deref, DerefMut},
	io::{self, Write},
};

/// Trait allowing the switching between the alternate screen and the main
/// screen. It is automatically implemented for any type implementing
/// [`Write`].
///
/// This trait defines two functions:
/// - [`enter_alternate_screen`][ScreenController::enter_alternate_screen] –
///   switch to the alternate screen.
/// - [`leave_alternate_screen`][ScreenController::leave_alternate_screen] –
///   switch back to the main screen.
///
///
///
/// For more info about the difference between the main screen and the
/// alternate screen, see the [module-level docs][self]
pub trait ScreenController {
	/// Switches to the alternate screen.
	///
	/// The alternate screen is a buffer the terminal has seperate from the
	/// main screen. When in the alternate screen, you have access to all the
	/// terminal's cells which makes it easier to writer full-screen terminal
	/// applications. See the [module-level docs][self] for more info.
	fn enter_alternate_screen(&mut self) -> io::Result<()>;

	/// Switches back to the main screen from the alternate screen.
	fn leave_alternate_screen(&mut self) -> io::Result<()>;
}

impl<W: Write> ScreenController for W {
	fn enter_alternate_screen(&mut self) -> io::Result<()> {
		self.write_all(b"\x1b[?1049h")
	}

	fn leave_alternate_screen(&mut self) -> io::Result<()> {
		self.write_all(b"\x1b[?1049l")
	}
}

/// Wrapper around a [`ScreenController`] which disables the alternate screen
/// on `Self`'s `Drop` implementation.
///
/// This helps to ensure correctness in a program making use of the alternate
/// screen and make sure that the alternate screen is disabled even when a
/// panic occurs.
pub struct ScreenControllerGuard<S: ScreenController> {
	inner: S,
	alternate_screen_enabled: bool,
}

impl<S: ScreenController> Drop for ScreenControllerGuard<S> {
	fn drop(&mut self) {
		if self.alternate_screen_enabled {
			let _ = self.inner.leave_alternate_screen();
		}
	}
}

impl<S: ScreenController> ScreenController for ScreenControllerGuard<S> {
	fn enter_alternate_screen(&mut self) -> io::Result<()> {
		self.inner.enter_alternate_screen()?;
		self.alternate_screen_enabled = true;
		Ok(())
	}

	fn leave_alternate_screen(&mut self) -> io::Result<()> {
		self.inner.leave_alternate_screen()?;
		self.alternate_screen_enabled = false;
		Ok(())
	}
}

impl<S: ScreenController> Deref for ScreenControllerGuard<S> {
	type Target = S;
	
	#[inline]
	fn deref(&self) -> &Self::Target {
		&self.inner
	}
}

impl<S: ScreenController> DerefMut for ScreenControllerGuard<S> {
	#[inline]
	fn deref_mut(&mut self) -> &mut Self::Target {
		&mut self.inner
	}
}

impl<S: ScreenController> ScreenControllerGuard<S> {
	/// Creates a new [`ScreenControllerGuard`] from an inner
	/// [`ScreenController`]
	///
	/// This function assumes we are in the main screen.
	/// 
	/// # Examples
	/// ```rust
	/// # use chromoterm::screen::ScreenControllerGuard;
	/// use std::io;
	/// 
	/// let alternate_screen_guard = ScreenControllerGuard::new(io::stdout());
	/// 
	/// let alternate_screen_enabled = alternate_screen_guard.is_alternate_screen_enabled();
	/// 
	/// assert_eq!(alternate_screen_enabled, false);
	/// ```
	#[inline]
	pub fn new(inner: S) -> Self {
		Self {
			inner,
			alternate_screen_enabled: false
		}
	}

	/// Creates a new [`ScreenControllerGuard`], assuming that we are already
	/// in the alternate screen. 
	/// 
	/// # Examples
	/// ```rust
	/// # use chromoterm::screen::{ScreenControllerGuard, ScreenController};
	/// use std::io;
	/// 
	/// let mut stdout = io::stdout().lock();
	/// stdout.enter_alternate_screen()?;
	/// 
	/// let guard = ScreenControllerGuard::with_alternate_screen_enabled(stdout);
	/// 
	/// assert!(guard.is_alternate_screen_enabled());
	/// ```
	#[inline]
	pub fn with_alternate_screen_enabled(inner: S) -> Self {
		Self {
			inner,
			alternate_screen_enabled: true
		}
	}
	
	/// Returns whether the alternate screen is currently enabled or not.
	pub fn is_alternate_screen_enabled(&self) -> bool {
		self.alternate_screen_enabled
	}

	/// Returns an immutable shared reference to the inner
	/// [`ScreenController`].
	/// 
	/// Usually, you shouldn't need to access this function as 
	/// [`ScreenControllerGuard`] implements [`Deref<Target = S>`][Deref] and 
	/// [`DerefMut`], so any methods on the inner [`ScreenController`]
	///
	/// This function's mutable counterpart is [`Self::inner_mut`].
	#[inline]
	pub fn inner(&self) -> &S {
		&self.inner
	}
}