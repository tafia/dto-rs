//! Public module.

/// Struct for signup verification
#[derive(Clone, Serialize, Deserialize)]
pub struct RegisterDTO {
    /// The users username
    pub username: String,
    /// The users password
    pub password: String,
    /// The users email
    pub email: String,
    /// Posible referer
    pub referer: Option<String>,
}

/// The login date type object
#[derive(Serialize, Deserialize)]
pub struct LoginDTO {
    /// The users username or email
    pub user_email: String,
    /// The users password
    pub password: String,
    /// Extends the token time,
    pub remember_me: bool,
}

/// Struct to reset the users password
#[derive(Clone, Serialize, Deserialize)]
pub struct ResetPasswordDTO {
    /// The the username of the user
    pub username: String,
    /// Where the email of the user
    pub email: String,
}

/// The new password data type object
#[derive(Serialize, Deserialize)]
pub struct NewPasswordDTO {
    /// The new password
    pub new_password: String,
}
