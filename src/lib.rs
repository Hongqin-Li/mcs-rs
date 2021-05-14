#![cfg_attr(
    feature = "unstable",
    feature(asm, generic_param_attrs, dropck_eyepatch)
)]
#![no_std]

#[cfg(test)]
#[macro_use]
extern crate lazy_static;
#[cfg(test)]
extern crate std;

mod mutex;
mod pause;

pub use mutex::{Guard, Mutex, Slot};
