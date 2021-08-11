//! This module contains command line parsing utilities.
//!
//! The primary use is to enable using the *structopt* crate in context of implementing a
//! custom cargo command. The specific challenge here is to close the gap between what
//! cargo passes as command line arguments to the custom command executable and what
//! *structopt* expects:
//!
//! * Running `cargo <foo>` looks for an executable named *cargo-\<foo\>*, and when
//!   found calls *cargo-\<foo\>* passing *\<foo\>* as the second command line argument.
//! * *structopt* follows the established convention that the first command line argument
//!   designates the executable name, with arguments starting from the second being passed
//!   to that executable.
//!
//! The solution here is to implement an [`Iterator`] feeding *structopt* that addresses
//! both issues:
//!
//! * Convert the first command line argument *cargo-\<foo\>* into something that is
//!   displayed as `cargo foo`.
//! * Skip over the second command line argument, that's always equal to `foo` when
//!   invoked from cargo.
//!
use std::{env::args_os, ffi::OsString};

/// A structure holding command line arguments.
///
/// This structure isn't useful in itself. It merely provides an [`Iterator`]
/// implementation to be used by *structopt*'s `from_iter()` or `from_iter_safe()`
/// methods.
///
pub(crate) struct CargoExtensionCliParser {
    args: Vec<OsString>,
    current_index: usize,
}

impl CargoExtensionCliParser {
    /// Constructs a new [`CargoExtensionCliParser`] sourced from the OS-level command
    /// line arguments.
    ///
    pub(crate) fn new() -> Self {
        Self {
            args: args_os().into_iter().collect(),
            current_index: 0,
        }
    }
}

impl Iterator for CargoExtensionCliParser {
    type Item = OsString;

    /// Produces the next command line argument in a *structopt*-compatible fashion.
    ///
    /// It replaces the first argument (the actual executable name) with the constant
    /// string `"cargo uwp"`, skips over the second argument, and procedes to produce the
    /// remaining arguments, if any.
    ///

    // Note: This produces the desired behavior when invoked as `cargo uwp ...`, but fails
    //       when running this executable (cargo-uwp.exe) directly, in most spectacular
    //       ways.
    //       This isn't entirely satisfactory, but I'm leaving it as-is for now, since
    //       calling the executable rather than invoking `cargo uwp` is currently
    //       considered out-of-spec.
    //
    fn next(&mut self) -> Option<Self::Item> {
        self.current_index += 1;
        if self.current_index == 1 {
            Some("cargo uwp".into())
        } else {
            self.args.get(self.current_index).cloned()
        }
    }
}
