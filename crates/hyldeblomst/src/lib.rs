//! VM

#![warn(missing_docs)]
// Defined to disallow unchecked division in the VM crate
#![cfg_attr(not(test), deny(clippy::disallowed_methods))]
pub mod registers;
pub mod setnex;
