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
mod state;

use std::{
	io::{self, Write, Read, StdinLock, StdoutLock},
	fmt,
};

#[cfg(feature = "drop_handling")]
use indexmap::IndexMap;

pub use state::TerminalState;

/// Helper struct encapsulating standard I/O and terminal state
/// 
/// The type used for standard input and standard output defaults to 
/// [`StdinLock`] and [`StdoutLock`] but is completely customisable.
pub struct Terminal<'tty, I = StdinLock<'tty>, O = StdoutLock<'tty>, R = ()> {
	/// Standard input
	pub stdin: I,
	/// Standard output
	pub stdout: O,
	
	/// Optional user-defined resources that can be used in user-assigned drop 
	/// functions, e.g. cursor position
	pub resources: R,
	
	/// The current state of the terminal
	pub state: TerminalState<'tty>,
	#[cfg(feature = "drop_handling")]
	drop_fns: IndexMap<&'static str, fn(&mut Self)>,
}

#[cfg(feature = "drop_handling")]
impl<'tty, I, O, R> Drop for Terminal<'tty, I, O, R> {
	fn drop(&mut self) {
		for func in self.drop_fns.clone().values() {
			func(self)
		}
	}
}

/// Implements [`Read`] for [`Terminal`] whenever the user-chosen standard 
/// input implements `Read`. 
/// 
/// Users of this crate could set the standard input to be `()` by modifying 
/// the generic arguments of `Terminal<I, O, R>` (where `I` and `O` mean 
/// standard I/O) to `Terminal<(), _, _>`. Since `()` does not implement 
/// [`Read`], `Terminal<(), _, _>` will therefore not implement [`Read`] and 
/// won't be able to process user input itself.
impl<'tty, I, O, R> Read for Terminal<'tty, I, O, R>
where
	I: Read
{
	fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
		self.stdin.read(buf)
	}

	fn read_vectored(&mut self, bufs: &mut [io::IoSliceMut<'_>]) -> io::Result<usize> {
		self.stdin.read_vectored(bufs)
	}

	fn read_to_end(&mut self, buf: &mut Vec<u8>) -> io::Result<usize> {
		self.stdin.read_to_end(buf)
	}

	fn read_to_string(&mut self, buf: &mut String) -> io::Result<usize> {
		self.stdin.read_to_string(buf)
	}

	fn read_exact(&mut self, buf: &mut [u8]) -> io::Result<()> {
		self.stdin.read_exact(buf)
	}
}

impl<'tty, I, O, R> Write for Terminal<'tty, I, O, R>
where
	O: Write
{
	fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
		self.stdout.write(buf)
	}

	fn write_vectored(&mut self, bufs: &[io::IoSlice<'_>]) -> io::Result<usize> {
		self.stdout.write_vectored(bufs)
	}

	fn flush(&mut self) -> io::Result<()> {
		self.stdout.flush()
	}

	fn write_all(&mut self, buf: &[u8]) -> io::Result<()> {
		self.stdout.write_all(buf)
	}

	fn write_fmt(&mut self, args: fmt::Arguments<'_>) -> io::Result<()> {
		self.stdout.write_fmt(args)
	}
}

impl<'tty, 'i, 'o> Terminal<'tty, StdinLock<'i>, StdoutLock<'o>, ()> {
	pub fn new() -> Self {
		Self::with_resources(())
	}
}

impl<'tty, 'i, 'o, R> Terminal<'tty, StdinLock<'i>, StdoutLock<'o>, R> {
	pub fn with_resources(resources: R) -> Self {
		Self::from_parts(
			io::stdin().lock(),
			io::stdout().lock(),
			resources,
		)
	}
}

impl<'tty, I: Read, O: Write, R> Terminal<'tty, I, O, R> {
	pub fn from_parts(
		stdin: I,
		stdout: O,
		resources: R,
	) -> Self {
		Self {
			stdout,
			stdin,
			resources,
			state: TerminalState::default(),
			#[cfg(feature = "drop_handling")]
			drop_fns: IndexMap::new(),
		}
	}
}

impl<'tty, I, O, R> Terminal<'tty, I, O, R> {
	/// Assigns a new function with the given label to be called on `Self`'s
	/// `Drop` implementation
	#[cfg(feature = "drop_handling")]
	pub fn assign_drop_fn(&mut self, label: &'static str, func: fn(&mut Self)) {
		self.drop_fns.insert(label, func);
	}

	/// Removes an existing drop function with the given label, returning
	/// whether the function with the given label existed and was removed
	#[cfg(feature = "drop_handling")]
	pub fn remove_drop_fn(&mut self, label: &'static str) -> bool {
		self.drop_fns.shift_remove(label).is_some()
	}

	/// Checks if a drop function with the given label exists
	#[cfg(feature = "drop_handling")]
	pub fn drop_fn_exists(&self, label: &'static str) -> bool {
		self.drop_fns.contains_key(label)
	}
}