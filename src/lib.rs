//! This crate contains the data type objects for Fractal Global. It contains the objects
//! sent to the fractal api.
//!
//!
//! Using it is as simple as including this in the crate:
//!
//! ```
//! extern crate fractal_dto;
//! ```
#![forbid(missing_docs, warnings)]
#![deny(deprecated, improper_ctypes, non_shorthand_field_patterns, overflowing_literals,
    plugin_as_library, private_no_mangle_fns, private_no_mangle_statics, stable_features,
    unconditional_recursion, unknown_lints, unused, unused_allocation, unused_attributes,
    unused_comparisons, unused_features, unused_parens, while_true)]
#![warn(trivial_casts, trivial_numeric_casts, unused, unused_extern_crates, unused_import_braces,
    unused_qualifications, unused_results, variant_size_differences)]
extern crate rustc_serialize;
extern crate chrono;
extern crate fractal_utils as utils;

use rustc_serialize::{Encodable, Decodable};

pub mod error;
pub use error::FromDTOError;

pub mod v1;
pub use v1::*;

/// The dto trate to make it Encodeable and Decodable into fractal objects
pub trait DTO: Encodable + Decodable {}

/// creates an object from a dto
pub trait FromDTO<D: DTO>: Sized {
    /// the from dto wrapper
    fn from_dto(dto: D) -> Result<Self, FromDTOError>;
}

// /// The new password data type object
// #[derive(RustcEncodable, RustcDecodable)]
// pub struct NewPasswordDTO {
//     /// The new password
//     pub new_password: String,
// }
//
// impl DTO for NewPasswordDTO {}
//
// /// Holds both public and signing keys encoded in base64
// #[derive(RustcEncodable, RustcDecodable)]
// pub struct PublicKeysDTO {
//     /// the public ntrumls key in base 64
//     pub public_sign_key: String,
//     /// the public ntru key in base 64
//     pub public_encrypt_key: String,
// }
//
// impl DTO for PublicKeysDTO {}
