//! Friends module.

use super::user::ProfileDTO;

/// Enum representing the relationship between two friends.
pub type Relationship = RelationshipDTO;

/// Struct for a fractal connection
#[derive(Clone, Serialize, Deserialize)]
pub struct FriendRequestDTO {
    /// Where the connection originated.
    pub origin_id: u64,
    /// Who the user is trying to connect to.
    pub destination_id: u64,
    /// Message for the request.
    pub message: Option<String>,
    /// The particulars of there relationship.
    pub relationship: RelationshipDTO,
}

/// Enum representing the relationship between two friends.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Copy)]
pub enum RelationshipDTO {
    /// A stranger to the user
    Stranger,
    /// An Acquaintance to the uesr
    Acquaintance,
    /// A CoWorker to the user
    CoWorker,
    /// A friend to the uesr
    Friend,
    /// A Family member to the user
    Family,
}

/// Struct for friend requests
#[derive(Clone, Serialize, Deserialize)]
pub struct PendingFriendRequestDTO {
    /// Connection ID.
    pub connection_id: u64,
    /// Origin user.
    pub origin_user: ProfileDTO,
    /// Destination user.
    pub destination_user: ProfileDTO,
    /// Message.
    pub message: Option<String>,
}

/// Struct to a confirm pending connection
#[derive(Clone, Serialize, Deserialize)]
pub struct ConfirmFriendRequestDTO {
    /// The id of the connection
    pub request_id: u64,
    /// Where the connection originated
    pub origin: u64,
    /// The user confirming the connection
    pub destination: u64,
}
