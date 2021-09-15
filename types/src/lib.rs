#![cfg_attr(feature = "no-std", no_std)]

#[cfg(feature = "no-std")]
extern crate alloc;

pub mod common;
pub mod segment;
pub mod cartesian;
