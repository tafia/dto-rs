//! Transaction module.

use chrono::{DateTime, UTC};
use utils::{WalletAddress, Amount};

use {DTO, ProfileDTO};

/// Struct used to generate a transaction.
#[derive(Clone, RustcEncodable, RustcDecodable)]
pub struct GenerateTransactionDTO {
    /// Where the transaction originated.
    pub origin_id: u64,
    /// The destination wallet address of the transaction.
    pub destination_address: WalletAddress,
    /// Who the transaction is going to.
    pub destination_id: u64,
    /// The amount of the transaction.
    pub amount: Amount,
}

impl DTO for GenerateTransactionDTO {}

/// Struct for tansactions.
#[derive(Clone, RustcEncodable, RustcDecodable)]
pub struct TransactionDTO {
    /// The id of the transaction.
    pub transaction_id: u64,
    /// The origin of the transaction.
    pub origin_user: ProfileDTO,
    /// The destination of the transaction.
    pub destination_user: ProfileDTO,
    /// The destination address of the transaction.
    pub destination_address: WalletAddress,
    /// The amount of the transaction.
    pub amount: Amount,
    /// The timestamp of the transaction.
    pub timestamp: DateTime<UTC>,
}

impl DTO for TransactionDTO {}

/// Struct for pending transactions.
#[derive(Clone, RustcEncodable, RustcDecodable)]
pub struct PendingTransactionDTO {
    /// The pending transaction code.
    pub code: String,
}

impl DTO for PendingTransactionDTO {}

/// Struct for Authenticating transactions.
#[derive(Clone, RustcEncodable, RustcDecodable)]
pub struct AuthenticateTransactionDTO {
    /// The pending transaction key.
    pub key: String,
    /// The 6 digit 2 factor authentication code.
    pub code: u32,
}

impl DTO for AuthenticateTransactionDTO {}
