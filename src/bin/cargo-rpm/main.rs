//! Main entry point for `cargo rpm`

#![deny(missing_docs, trivial_casts, unused_qualifications)]
#![forbid(unsafe_code)]
#![allow(unused_imports)]

use cargo_rpm::application::APPLICATION;

/// Boot `cargo-rpm`
fn main() {
    abscissa_core::boot(&APPLICATION);
}
