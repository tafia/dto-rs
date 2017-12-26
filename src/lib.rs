#![doc(html_logo_url = "http://test.fractal.global/img/logo.svg",
       html_favicon_url = "http://test.fractal.global/img/favicon32.png",
       html_root_url = "http://fractal.global/dto-rs/")]
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
extern crate chrono;
extern crate fractal_utils as utils;

#[macro_use]
extern crate serde_derive;

pub mod error;
pub use error::FromDTOError;

pub mod v1;
pub use v1::*;

// /// The dto trate to make it Encodeable and Decodable into fractal objects
// pub trait DTO: Encodable + Decodable {}
// 
// /// creates an object from a dto
// pub trait FromDTO<D: DTO>: Sized {
//     /// the from dto wrapper
//     fn from_dto(dto: D) -> Result<Self, FromDTOError>;
// }
