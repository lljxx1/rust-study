//!
//! # An mixed(mem&disk) fundb implementation
//!

#![deny(warnings)]
#![deny(missing_docs)]
#![allow(clippy::upper_case_acronyms)]

pub mod helper;
pub mod mapx;
mod serde;
pub mod vecx;

pub use mapx::Mapx;
pub use vecx::Vecx;
