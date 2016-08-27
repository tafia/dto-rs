extern crate rustc_serialize;
extern crate chrono;
extern crate fractal_utils as utils;

use std::collections::{HashMap, BTreeSet};
use std::fmt;
use std::error::Error;
use rustc_serialize::{Encodable, Decodable};

use chrono::{NaiveDate, DateTime, UTC};
use utils::{Amount, WalletAddress, Address};

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
    /// The users email
    pub email: String,
    //// Whether the email has been confirmed or not
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
    // Whether the users phone ## has been confirmed
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
    pub last_activty: DateTime<UTC>,
    /// Whether the user is banned and until when
    pub banned: Option<DateTime<UTC>>,
}

impl DTO for UserDTO {}

/// The login date type object
#[derive(RustcEncodable, RustcDecodable)]
pub struct LoginDTO {
    /// The users username
    pub username: String,
    /// The users password
    pub password: String,
}

impl DTO for LoginDTO {}

/// Holds both public and signing keys encoded in base64
#[derive(RustcEncodable, RustcDecodable)]
pub struct PublicKeysDTO {
    pub public_sign_key: String,
    pub public_encrypt_key: String,
}

impl DTO for PublicKeysDTO {}

/// Struct for for fractal connection
#[derive(Clone, RustcEncodable, RustcDecodable)]
pub struct FractalConnectionDTO {
    /// Where the connection originated
    pub origin_id: u64,
    /// Who the user is trying to connect to
    pub destination_id: u64,
    /// The particulars of there relationship
    pub relationship: u8,
}

impl DTO for FractalConnectionDTO {}

/// Struct for for fractal developer client
#[derive(Clone, RustcEncodable, RustcDecodable)]
pub struct DeveloperClientDTO {
    /// The name of the client
    pub name: String,
    /// The permissions the client has
    pub scopes: Vec<ScopeDTO>,
}

impl DTO for DeveloperClientDTO {}

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

impl DTO for GenerateTransactionDTO { }

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

impl DTO for RegisterDTO { }

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
    Bearer,
}

impl fmt::Display for TokenTypeDTO {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl DTO for TokenTypeDTO {}

pub trait DTO: Encodable + Decodable {}

pub trait FromDTO<D: DTO>: Sized {
    fn from_dto(dto: D) -> Result<Self, FromDTOError>;
}

#[derive(Debug)]
pub struct FromDTOError {
    error: String,
}

impl FromDTOError {
    /// Creates a new FromDTOError
    pub fn new<S: AsRef<str>>(error: S) -> FromDTOError {
        FromDTOError {
            error: String::from(error.as_ref()),
        }
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
