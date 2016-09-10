//! Transaction module.

use chrono::{DateTime, UTC};
use utils::{WalletAddress, Amount};

use DTO;

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

/// Struct for tansactions
#[derive(Clone, RustcEncodable, RustcDecodable)]
pub struct TransactionDTO {
    /// The id of the transaction
    pub id: u64,
    /// The origin of the transaction
    pub origin_user: u64,
    /// The destination of the transaction
    pub destination_user: u64,
    /// The destination address of the transaction
    pub destination_address: WalletAddress,
    /// The amount of the transaction
    pub amount: Amount,
    /// The timestamp of the transaction
    pub timestamp: DateTime<UTC>,
}

impl DTO for TransactionDTO {}
