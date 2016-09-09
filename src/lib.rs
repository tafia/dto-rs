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

use std::collections::{HashMap, BTreeSet};
use std::fmt;
use std::error::Error;
use rustc_serialize::{Encodable, Decodable};

use chrono::{NaiveDate, DateTime, UTC};
use utils::{Amount, WalletAddress, Address, Relationship};

/// Enum that represents
#[derive(Debug, PartialEq, Eq, Copy, Clone, RustcDecodable, RustcEncodable)]
pub enum ScopeDTO {
    /// Administration scope
    ///
    /// This scope is used for administration purposes, and will not be enabled for public
    /// development accounts.
    Admin,
    /// User scope
    ///
    /// This scope will provide access to user functionality, such as creating transactions and
    /// editing user information. It contains the user ID for which the token is valid.
    User(u64),
    /// Public scope
    ///
    /// This scope is the public scope. Every client will have access to everything provided in the
    /// admin scope.
    Public,
    /// Developer scope
    ///
    /// This scope is used for administration purposes, and will not be enabled for public
    /// development accounts.
    Developer,
}

impl fmt::Display for ScopeDTO {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}



/// The user date type object
#[derive(RustcEncodable, RustcDecodable)]
pub struct UserDTO {
    /// The unique ID of the user
    pub id: u64,
    /// The unique username of the user
    pub username: String,
    /// The display name of the user
    pub displayname: String,
    /// The users email
    pub email: String,
    /// / Whether the email has been confirmed or not
    pub email_confirmed: bool,
    /// The users first name
    pub first: Option<String>,
    /// Whether the first name has been confirmed or not
    pub first_confirmed: bool,
    /// The users last name
    pub last: Option<String>,
    /// Whether the last name has been confirmed
    pub last_confirmed: bool,
    /// The amount of devices the user has
    pub device_count: u8,
    /// The users wallet addresses
    pub wallet_addresses: BTreeSet<WalletAddress>,
    /// The users pending balance
    pub pending_balance: Amount,
    /// The users checking balance
    pub checking_balance: Amount,
    /// The users cold balance
    pub cold_balance: Amount,
    /// The users bonds and when he purchased them
    pub bonds: HashMap<DateTime<UTC>, u64>,
    /// The users birthday
    pub birthday: Option<NaiveDate>,
    /// Whether the birthday has been confirmed
    pub birthday_confirmed: bool,
    /// The users phone #
    pub phone: Option<String>,
    /// Whether the users phone # has been confirmed
    pub phone_confirmed: bool,
    /// The users profile picture
    pub image: Option<String>,
    /// The users address
    pub address: Option<Address>,
    /// Whether the address has been confirmed
    pub address_confirmed: bool,
    /// The users sybil score
    pub sybil_score: i8,
    /// The users trust score
    pub trust_score: i8,
    /// Whether the users account id disabled
    pub enabled: bool,
    /// When the user registered
    pub registered: DateTime<UTC>,
    /// The users last activity time
    pub last_activity: DateTime<UTC>,
    /// Whether the user is banned and until when
    pub banned: Option<DateTime<UTC>>,
}

impl DTO for UserDTO {}

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

/// The new password data type object
#[derive(RustcEncodable, RustcDecodable)]
pub struct NewPasswordDTO {
    /// The new password
    pub new_password: String,
}

impl DTO for NewPasswordDTO {}

/// Holds both public and signing keys encoded in base64
#[derive(RustcEncodable, RustcDecodable)]
pub struct PublicKeysDTO {
    /// the public ntrumls key in base 64
    pub public_sign_key: String,
    /// the public ntru key in base 64
    pub public_encrypt_key: String,
}

impl DTO for PublicKeysDTO {}

/// Struct for a fractal connection
#[derive(Clone, RustcEncodable, RustcDecodable)]
pub struct FractalConnectionDTO {
    /// Where the connection originated
    pub origin_id: u64,
    /// Who the user is trying to connect to
    pub destination_id: u64,
    /// The particulars of there relationship
    pub relationship: Relationship,
}

impl DTO for FractalConnectionDTO {}

/// Struct for creating a fractal developer
#[derive(Clone, RustcEncodable, RustcDecodable)]
pub struct CreateClientDTO {
    /// The name of the client
    pub name: String,
    /// The permissions the client has
    pub scopes: Vec<ScopeDTO>,
}

impl DTO for CreateClientDTO {}

/// Struct with the developer client information
#[derive(Clone, RustcEncodable, RustcDecodable)]
pub struct ClientInfoDTO {
    /// The name of the client
    pub id: String,
    /// The permissions the client has
    pub secret: String,
}

impl DTO for ClientInfoDTO {}

/// Struct to a confirm pending connection
#[derive(Clone, RustcEncodable, RustcDecodable)]
pub struct ConfirmPendingConnectionDTO {
    /// The id of the connection
    pub id: u64,
    /// Where the connection originated
    pub origin: u64,
    /// The user confirming the connection
    pub destination: u64,
}

impl DTO for ConfirmPendingConnectionDTO {}

/// Struct to reset the users password
#[derive(Clone, RustcEncodable, RustcDecodable)]
pub struct ResetPasswordDTO {
    /// The the username of the user
    pub username: String,
    /// Where the email of the user
    pub email: String,
}

impl DTO for ResetPasswordDTO {}


/// Struct used to update user information
#[derive(Clone, RustcEncodable, RustcDecodable)]
pub struct UpdateUserDTO {
    /// The users new username
    pub new_username: Option<String>,
    /// The users old password
    pub old_password: Option<String>,
    /// The users new password
    pub new_password: Option<String>,
    /// The users new first name
    pub new_first: Option<String>,
    /// The users new last name
    pub new_last: Option<String>,
    /// The users new address
    pub new_address: Option<Address>,
    /// The users new birthday
    pub new_birthday: Option<NaiveDate>,
    /// The users new phone #
    pub new_phone: Option<String>,
    /// The users new email
    pub new_email: Option<String>,
    /// The users new profile picture
    pub new_image: Option<String>,
}

impl DTO for UpdateUserDTO {}

/// Struct used to generate a transaction
#[derive(Clone, RustcEncodable, RustcDecodable)]
pub struct GenerateTransactionDTO {
    /// Where the transaction originated
    pub origin_id: u64,
    /// The destination wallet address of the transaction
    pub destination_address: WalletAddress,
    /// Who the transaction is going to
    pub destination_id: u64,
    /// The amount of the transaction
    pub amount: Amount,
}

impl DTO for GenerateTransactionDTO {}

/// Struct for for signup verification
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

/// AccessToken Data type object
#[derive(Debug, Clone, RustcDecodable, RustcEncodable)]
pub struct AccessTokenDTO {
    /// The app id
    pub app_id: String,
    /// The permissions of the access token
    pub scopes: String,
    /// The access token
    pub access_token: String,
    /// The access tokken type (currently only configured for bearer)
    pub token_type: TokenTypeDTO,
    /// The expiration time of the token
    pub expiration: i64,
}

impl DTO for AccessTokenDTO {}

/// Token type data type object (currently only using bearer)
#[derive(Debug, PartialEq, Eq, Copy, Clone, RustcDecodable, RustcEncodable)]
pub enum TokenTypeDTO {
    /// oAuth Bearer token type
    Bearer,
}

impl fmt::Display for TokenTypeDTO {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl DTO for TokenTypeDTO {}

/// The dto trate to make it Encodeable and Decodable into fractal objects
pub trait DTO: Encodable + Decodable {}

/// creates an object from a dto
pub trait FromDTO<D: DTO>: Sized {
    /// the from dto wrapper
    fn from_dto(dto: D) -> Result<Self, FromDTOError>;
}

/// From DTO Error
#[derive(Debug)]
pub struct FromDTOError {
    error: String,
}

impl FromDTOError {
    /// Creates a new FromDTOError
    pub fn new<S: AsRef<str>>(error: S) -> FromDTOError {
        FromDTOError { error: String::from(error.as_ref()) }
    }
}

impl fmt::Display for FromDTOError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.error)
    }
}

impl Error for FromDTOError {
    fn description(&self) -> &str {
        &self.error
    }

    fn cause(&self) -> Option<&Error> {
        None
    }
}
