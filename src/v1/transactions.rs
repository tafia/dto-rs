//! Transaction module.

use chrono::DateTime;
use chrono::offset::Utc;
use utils::amount::Amount;
use utils::wallet_address::WalletAddress;
use user::ProfileDTO;

/// Struct used to generate a transaction.
#[derive(Clone, Serialize, Deserialize)]
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

/// Struct for tansactions.
#[derive(Clone, Serialize, Deserialize)]
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
    pub timestamp: DateTime<Utc>,
}

/// Struct for pending transactions.
#[derive(Clone, Serialize, Deserialize)]
pub struct PendingTransactionDTO {
    /// The pending transaction code.
    pub code: String,
}
