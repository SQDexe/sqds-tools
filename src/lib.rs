/*!
A collection of simple QoL tools.
Includes different types, functions, traits, macros, and so on.

# Disclaimer

This crate's purpose is to serve as – mostly – a personal tooling package.
Linking with it might lead to problems in the future,  
if some greater change were to happen to it.

Please be mindful, and proceed with caution when using this package.
*/

#![no_std]

#[cfg(test)]
extern crate std;

/* Modules declaration */
mod layout;
mod macros;
mod show;

/* Public re-exports */
pub use crate::layout::LayoutMetrics;
pub use crate::show::EmptyDisplay;
pub use crate::show::DisplayOption;
pub use crate::show::DisplayResult;
pub use crate::show::DisplaySlice;
pub use crate::show::ShowOption;
pub use crate::show::ShowResult;
pub use crate::show::ShowSlice;