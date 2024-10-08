//! reaper_core: A Rust library for quantitative finance tools.
//!
//! Any contributions are greatly appreciated. Make a PR or open an issue !
//!
//! I'm particularly interested in hearing from people with strong experience
//! in implementing quantitative software in a professional setting.
//!
//! # Installation
//!
//! In your Rust project's root directory, simply run:
//!
//! ```bash
//! cargo add reaper_core
//! ```
//!
//! This will add the latest version to your project.
//!
//! If you require a specific version, add the following to your Cargo.toml file:
//!
//! ```toml
//! [dependencies]
//! reaper_core = "*"
//! ```
//!
//! replacing `"*"` with the version number you require, such as `"0.0.17"`.

#![doc = include_str!("../README.md")]
#![deny(rust_2018_idioms, unsafe_code)]
#![allow(clippy::needless_collect)] // the implementation of that rule is way too eager, it rejects necessary collects
#![allow(clippy::derive_partial_eq_without_eq)]
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// GLOBAL SETTINGS
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

// Strictly enforce documentation.
// #![forbid(missing_docs)]
//
// When writing mathematical equations in documentation, Clippy suggests to
// put backticks inside the LaTeX block. This suppresses that behavior.
#![allow(clippy::doc_markdown)]
//
// Allow snake case.
// This is because much of this library is based on mathematics, so I
// want to adhere to the standard mathematical notation.
#![allow(non_snake_case)]
//
// Strictly enforce SAFETY comments.
// There is no unsafe code currently, but for anyone to add any, it must be
// documented with a SAFETY comment.
#![forbid(clippy::undocumented_unsafe_blocks)]

#[macro_use]
pub mod macros;
pub mod cashflow;
pub mod error;
pub mod instruments;
pub mod iso;
pub mod time;
pub mod trading;

#[cfg(test)]
mod tests {
  #[test]
  fn it_works() {
    let result = 2 + 2;
    assert_eq!(result, 4);
  }
}
