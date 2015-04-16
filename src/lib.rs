#![feature(no_std)]
#![feature(asm, core)]
#![feature(libc)]
#![no_std]

#[macro_use]
extern crate core;

#[cfg(test)]
#[macro_use]
extern crate std;

pub use context::Context;
pub use stack::{Stack, StackSource};

#[cfg(not(test))]
mod std { pub use core::*; }

pub mod context;
pub mod stack;

mod debug;

#[cfg(target_arch = "x86_64")]
mod arch;

#[cfg(feature = "os")]
pub mod os;
