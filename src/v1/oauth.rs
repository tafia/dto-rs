//! OAuth module.

use std::fmt;

use DTO;

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

/// Enum representing token type.
///
/// Currently only using bearer.
#[derive(Debug, Clone, RustcDecodable, RustcEncodable, PartialEq, Eq, Copy)]
pub enum TokenTypeDTO {
    /// OAuth Bearer token type
    Bearer,
}

impl fmt::Display for TokenTypeDTO {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl DTO for TokenTypeDTO {}

/// Enum that represents a scope.
#[derive(Debug, Clone, RustcDecodable, RustcEncodable, PartialEq, Eq, Copy)]
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
    /// public scope.
    Public,
}

impl fmt::Display for ScopeDTO {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl DTO for ScopeDTO {}

/// Struct for creating a fractal developer
#[derive(Clone, RustcEncodable, RustcDecodable)]
pub struct CreateClientDTO {
    /// The name of the client
    pub name: String,
    /// The permissions the client has
    pub scopes: Vec<ScopeDTO>,
    /// Number of requests per hour that the client will be able to do
    pub request_limit: usize,
}

impl DTO for CreateClientDTO {}

/// Struct with the developer client information
#[derive(Clone, RustcEncodable, RustcDecodable)]
pub struct ClientInfoDTO {
    /// The ID of the client
    pub id: String,
    /// The name of the client
    pub name: String,
    /// The secret of the client
    pub secret: String,
    /// The scopes of the client
    pub scopes: Vec<ScopeDTO>,
    /// The request limit of the client
    pub request_limit: usize,
}

impl DTO for ClientInfoDTO {}
