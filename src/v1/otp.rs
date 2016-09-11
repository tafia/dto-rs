//! One time password module.

use DTO;

/// The 6 digit authentication code struct
#[derive(Clone, RustcEncodable, RustcDecodable)]
pub struct AuthenticationCodeDTO {
    /// The 6 digit authentication code
    pub code: u32,
}

impl DTO for AuthenticationCodeDTO {}
