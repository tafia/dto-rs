//! One time password module.

/// The 6 digit authentication code struct
#[derive(Clone, Serialize, Deserialize)]
pub struct AuthenticationCodeDTO {
    /// The 6 digit authentication code
    pub code: u32,
}
