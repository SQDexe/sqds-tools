//! A collection of simple QoL tools.  
//! Includes diffrent types, functions, traits, macros, and so on.
//! 
//! # Disclaimer
//! 
//! This crate's purpose is to serve as - mostly - a personal tooling package.  
//! Linking with it might lead to problems in the future, in case some greater change were to happen with it.
//! 
//! Please be mindful, and proceed with caution when using this package.  

#![no_std]

#[cfg(test)]
extern crate std;

/* Modules declaration */
mod macros;
mod traits;

pub use traits::Size;