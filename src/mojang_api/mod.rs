#![doc = include_str!("./README.md")]

/// Utilities for finding and checking servers blocked by Mojang.
pub mod blocked_servers;

/// Errors used throughout this library.
pub mod error;

/// Fetching the profile/textures, or username history of a user.
pub mod profile;

/// Utilities for fetching basic user data, such as resolving a username to a UUID.
pub mod user;

#[doc(hidden)]
pub mod client;

pub use blocked_servers::BlockedServers;
pub use profile::Profile;
pub use user::get_username_uuid;
