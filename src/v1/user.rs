//! Users module.

use std::collections::BTreeSet;

use chrono::{NaiveDate, DateTime};
use chrono::offset::Utc;
use utils::location::Address;
use utils::amount::Amount;
use utils::wallet_address::WalletAddress;

/// The user date type object
#[derive(Serialize, Deserialize)]
pub struct UserDTO {
    /// The unique ID of the user
    pub user_id: u64,
    /// The unique username of the user
    pub username: String,
    /// The display name of the user
    pub display_name: String,
    /// The users email
    pub email: String,
    /// / Whether the email has been confirmed or not
    pub email_confirmed: bool,
    /// The users first name
    pub first_name: Option<String>,
    /// Whether the first name has been confirmed or not
    pub first_name_confirmed: bool,
    /// The users last name
    pub last_name: Option<String>,
    /// Whether the last name has been confirmed
    pub last_name_confirmed: bool,
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
    /// The users birthday
    pub birthday: Option<NaiveDate>,
    /// Whether the birthday has been confirmed
    pub birthday_confirmed: bool,
    /// The users phone #
    pub phone: Option<String>,
    /// Whether the users phone # has been confirmed
    pub phone_confirmed: bool,
    /// The users profile picture
    pub image_url: Option<String>,
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
    pub registration_time: DateTime<Utc>,
    /// The users last activity time
    pub last_activity: DateTime<Utc>,
    /// Whether the user is banned and until when
    pub banned: Option<DateTime<Utc>>,
}

/// Struct for profiles
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProfileDTO {
    /// User's ID.
    pub user_id: u64,
    /// Display name of the user.
    pub display_name: String,
    /// First name of the user.
    pub first_name: Option<String>,
    /// Last name of the user.
    pub last_name: Option<String>,
    /// Link to the user's profile image.
    pub image_url: Option<String>,
    /// Age of the user.
    pub age: Option<u8>,
    /// Address of the user.
    pub address: Option<String>,
    /// Trust score of the user.
    pub trust_score: i8,
}

/// Struct used to update user information
#[derive(Clone, Serialize, Deserialize)]
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

/// Struct used to search a user.
#[derive(Clone, Serialize, Deserialize)]
pub struct SearchUserDTO {
    /// Random data.
    pub random: Option<String>,
    /// Username.
    pub username: Option<String>,
    /// Email of the user.
    pub email: Option<String>,
    /// First name of the user.
    pub first_name: Option<String>,
    /// Last name of the user.
    pub last_name: Option<String>,
    /// Age of the user.
    pub age: Option<u8>,
    /// Country of the user.
    pub country: Option<String>,
    /// State of the user.
    pub state: Option<String>,
    /// City of the user.
    pub city: Option<String>,
    /// Phone of the user.
    pub phone: Option<String>,
    /// Check for all or any?
    pub all: bool,
    /// Include the current user, if logged in.
    pub include_me: bool,
    /// Include friends of the current user, if logged in.
    pub include_friends: bool,
}
