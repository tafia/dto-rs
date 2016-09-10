//! DTOs for the first version of the API.

pub mod oauth;
pub mod public;
pub mod user;
pub mod friends;
pub mod transactions;

pub use oauth::*;
pub use public::*;
pub use user::*;
pub use friends::*;
pub use transactions::*;

use DTO;

/// Response Data type object
#[derive(Debug, Clone, RustcDecodable, RustcEncodable)]
pub struct ResponseDTO {
    /// The message
    pub message: String,
}

impl ResponseDTO {
    /// creates a new response dto
    pub fn new<S: AsRef<str>>(message: S) -> ResponseDTO {
        ResponseDTO { message: String::from(message.as_ref()) }
    }
    /// Sets the message of the response
    pub fn set_message<S: AsRef<str>>(&mut self, message: S) {
        self.message = String::from(message.as_ref());
    }
}

impl DTO for ResponseDTO {}
