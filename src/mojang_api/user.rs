use crate::mojang_api::client::{get, post};
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
/// Gets a list of [User]s from a list of usernames in a single request
///
/// Invalid usernames will be skipped in the result, and will not error
///
/// Limited to 10 per request
pub fn get_uuids_from_usernames(usernames: &[&str]) -> Result<Vec<User>, ApiError> {
    let url = "https://api.mojang.com/profiles/minecraft";
    Ok(post(url, &usernames)?.json()?)
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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn get_uuids() {
        let uuids = get_uuids_from_usernames(&["brecert", "MHF_Present1", "MHF_Present2"]).unwrap();
        assert_eq!(
            uuids,
            vec![
                User {
                    id: "7a8084cd1f444a159bb1eef8d5b535a1".into(),
                    name: "brecert".into(),
                },
                User {
                    id: "156b251b12e04829a130a61b53ba7720".into(),
                    name: "MHF_Present1".into()
                },
                User {
                    id: "f1eb7cade2c04e9e8aad1eae21d5fd95".into(),
                    name: "MHF_Present2".into()
                }
            ]
        )
    }
}
