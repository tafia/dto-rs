extern crate rustc_serialize;
extern crate chrono;
extern crate fractal_utils as utils;

use std::fmt;
use std::str::FromStr;
use std::error::Error;
use rustc_serialize::{Encodable, Decodable};

use chrono::NaiveDate;
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

impl FromStr for ScopeDTO {
     type Err = FromDTOError;
     fn from_str(s: &str) -> Result<ScopeDTO, FromDTOError> {
         match s {
             "Admin" => Ok(ScopeDTO::Admin),
             "Public" => Ok(ScopeDTO::Public),
             "Developer" => Ok(ScopeDTO::Developer),
             s => match s.rfind("User:") {
                 Some(i) => Ok(ScopeDTO::User(match s[i..].parse() {
                     Ok(id) => id,
                     _ => return Err(FromDTOError::new("Invalid Scope")),
                 })),
                 _ => Err(FromDTOError::new("Invalid Scope")),
             },
         }
     }
 }


#[derive(RustcEncodable, RustcDecodable)]
pub struct LoginDTO {
    pub username: String,
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
    pub origin_id: u64,
    pub destination_id: u64,
    pub relationship: u8,
}

impl DTO for FractalConnectionDTO {}

/// Struct for for fractal developer client
#[derive(Clone, RustcEncodable, RustcDecodable)]
pub struct DeveloperClientDTO {
    pub name: String,
    pub scopes: Vec<ScopeDTO>,
}

impl DTO for DeveloperClientDTO {}

/// Struct to a confirm pending connection
#[derive(Clone, RustcEncodable, RustcDecodable)]
pub struct ConfirmPendingConnectionDTO {
    pub id: u64,
    pub origin: u64,
    pub destination: u64,
}

impl DTO for ConfirmPendingConnectionDTO {}

/// Struct used to update user information
#[derive(Clone, RustcEncodable, RustcDecodable)]
pub struct UpdateUserDTO {
    pub new_username: Option<String>,
    pub old_password: Option<String>,
    pub new_password: Option<String>,
    pub new_first: Option<String>,
    pub new_last: Option<String>,
    pub new_address: Option<Address>,
    pub new_birthday: Option<NaiveDate>,
    pub new_phone: Option<String>,
    pub new_email: Option<String>,
    pub new_image: Option<String>,
}

impl DTO for UpdateUserDTO {}

/// Struct used to generate a transaction
#[derive(Clone, RustcEncodable, RustcDecodable)]
pub struct GenerateTransactionDTO {
    pub origin_id: u64,
    pub destination_address: WalletAddress,
    pub destination_id: u64,
    pub amount: Amount,
}

impl DTO for GenerateTransactionDTO { }

/// Struct for for signup verification
#[derive(Clone, RustcEncodable, RustcDecodable)]
pub struct RegisterDTO {
    pub username: String,
    pub password: String,
    pub email: String,
}

impl DTO for RegisterDTO { }

/// AccessToken Data type object
#[derive(Debug, Clone, RustcDecodable, RustcEncodable)]
pub struct AccessTokenDTO {
    pub app_id: String,
    pub scopes: String,
    pub access_token: String,
    pub token_type: TokenTypeDTO,
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
    fn new<S: AsRef<str>>(error: S) -> FromDTOError {
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
