use thiserror::Error;

/// Errors which can occur when fetching fails from the api.
#[derive(Error, Debug)]
pub enum ApiError {
    /// When the response isn't a status code of `200`.
    #[error("[{}] API Request failed: {}", .status, .reason)]
    Request {
        /// The status code of the response
        status: i32,
        /// The reason given for the status code
        reason: String,
    },

    /// When the request fails to resolve.
    #[error("Fetching failed: {}", .0)]
    Fetch(#[from] minreq::Error),
}

/// Errors which can occur when validating a username fails.
#[derive(Error, Debug, PartialEq)]
pub enum UsernameError {
    /// The username was empty
    #[error("username was empty")]
    Empty,

    /// The username was longer than 16 characters.
    #[error("username was too long")]
    TooLong,

    /// The username contained an invalid character.
    #[error("username contained invalid character '{}'", .0)]
    InvalidCharacter(char),
}
