//! Public module.

use DTO;

/// Struct for signup verification
#[derive(Clone, RustcEncodable, RustcDecodable)]
pub struct RegisterDTO {
    /// The users username
    pub username: String,
    /// The users password
    pub password: String,
    /// The users email
    pub email: String,
}

impl DTO for RegisterDTO {}

/// The login date type object
#[derive(RustcEncodable, RustcDecodable)]
pub struct LoginDTO {
    /// The users username or email
    pub user_email: String,
    /// The users password
    pub password: String,
    /// Extends the token time,
    pub remember_me: bool,
}

impl DTO for LoginDTO {}

/// Struct to reset the users password
#[derive(Clone, RustcEncodable, RustcDecodable)]
pub struct ResetPasswordDTO {
    /// The the username of the user
    pub username: String,
    /// Where the email of the user
    pub email: String,
}

impl DTO for ResetPasswordDTO {}

/// The new password data type object
#[derive(RustcEncodable, RustcDecodable)]
pub struct NewPasswordDTO {
    /// The new password
    pub new_password: String,
}

impl DTO for NewPasswordDTO {}
