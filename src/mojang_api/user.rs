use crate::mojang_api::client::get;
use crate::mojang_api::error::{ApiError, UsernameError};

use serde::{Deserialize, Serialize};

/// Basic user information.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct User {
    /// The UUID of the user.
    pub id: String,
    /// The username of the user.
    pub name: String,
}

impl User {
    fn fetch(username: &str) -> Result<Self, ApiError> {
        let url = format!(
            "https://api.mojang.com/users/profiles/minecraft/{}",
            username
        );
        Ok(get(url)?.json()?)
    }
}

/// Gets the UUID of the username
pub fn get_username_uuid(username: &str) -> Result<String, ApiError> {
    User::fetch(username).map(|p| p.id)
}

/// Checks if a username is a valid username that the api may return.
///
/// This does not check if a username is currently available, or if a username is currently valid.
/// Notably this does not check if the length of a username is less than 3, as some very old minecraft accounts *do* have unique usernames that are shorter than 3.
///
/// ## Example
/// ```rust
/// # use minecraft_utils::mojang_api::error::UsernameError;
/// # use minecraft_utils::mojang_api::user::validate_username;
/// assert_eq!(validate_username("brecert"), Ok(()));
/// assert_eq!(validate_username("12345678901234567"), Err(UsernameError::TooLong));
/// assert_eq!(
///     validate_username("ブリー"),
///     Err(UsernameError::InvalidCharacter('ブ'))
/// );
/// assert_eq!(validate_username(""), Err(UsernameError::Empty));
/// ```
pub fn validate_username(username: &str) -> Result<(), UsernameError> {
    if username.len() == 0 {
        return Err(UsernameError::Empty);
    }

    if username.len() > 16 {
        return Err(UsernameError::TooLong);
    }

    username
        .chars()
        .find(|&ch| !ch.is_ascii_alphanumeric() && ch != '_')
        .map_or(Ok(()), |ch| Err(UsernameError::InvalidCharacter(ch)))
}
