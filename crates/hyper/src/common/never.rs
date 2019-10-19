//! An uninhabitable type meaning it can never happen.
//!
//! To be replaced with `!` once it is stable.

use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub enum Never {}

#[cfg_attr(test, ::mutagen::mutate)] impl fmt::Display for Never {
    fn fmt(&self, _: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {}
    }
}

#[cfg_attr(test, ::mutagen::mutate)] impl Error for Never {
    fn description(&self) -> &str {
        match *self {}
    }
}

