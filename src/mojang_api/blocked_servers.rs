use std::borrow::Cow;

use sha1::{Digest, Sha1};

use crate::mojang_api::client::get;
use crate::mojang_api::error::ApiError;

/// A list of hashes corresponding to blocked server patterns.
///
/// ## Example
/// ```rust
/// use minecraft_utils::mojang_api::BlockedServers;
///
/// // Use our own blocked servers list for demonstration purposes.
/// let blocked = BlockedServers {
///     hashes: vec![
///         // *.example.com
///         String::from("8c7122d652cb7be22d1986f1f30b07fd5108d9c0"),
///         // 192.0.*
///         String::from("8c15fb642b3e8f58480df51798382f1016e748eb"),
///         // 127.0.0.1
///         String::from("4b84b15bff6ee5796152495a230e45e3d7e947d9"),
///     ],
/// };
///
/// // Check if server is blocked
/// assert!(blocked.is_blocked("127.0.0.1"));
/// ```
#[derive(Debug, Clone)]
pub struct BlockedServers {
    /// Hashes of the block patterns
    pub hashes: Vec<String>,
}

impl BlockedServers {
    /// Fetch current Blocked Servers List
    ///
    /// ## Example
    /// ```rust
    /// # use minecraft_utils::mojang_api::BlockedServers;
    /// let blocked = BlockedServers::fetch().unwrap();
    ///
    /// // Check if server is blocked
    /// assert!(blocked.is_blocked("mc.playmc.mx"));
    /// ```
    pub fn fetch() -> Result<Self, ApiError> {
        let res = get("https://sessionserver.mojang.com/blockedservers")?;
        let txt = res.as_str()?;
        let lines = txt.lines().map(String::from).collect();
        Ok(BlockedServers { hashes: lines })
    }

    /// Check if the supplied address is in the blocklist, and if it is then return the matching pattern.
    ///
    /// ## Example
    /// ```rust
    /// # use minecraft_utils::mojang_api::BlockedServers;
    /// use std::borrow::Cow;
    /// # // Use our own blocked servers list for demonstration purposes.
    /// # let blocked = BlockedServers {
    /// #     hashes: vec![
    /// #         // *.example.com
    /// #         String::from("8c7122d652cb7be22d1986f1f30b07fd5108d9c0"),
    /// #         // 192.0.*
    /// #         String::from("8c15fb642b3e8f58480df51798382f1016e748eb"),
    /// #         // 127.0.0.1
    /// #         String::from("4b84b15bff6ee5796152495a230e45e3d7e947d9"),
    /// #     ],
    /// # };
    ///
    /// // Using the blocked servers list from the struct example find the matching pattern
    /// assert_eq!(blocked.find_blocked_pattern("mc.example.com"), Some(Cow::from("*.example.com")));
    /// assert_eq!(blocked.find_blocked_pattern("192.0.2.235"), Some(Cow::from("192.0.*")));
    /// assert_eq!(blocked.find_blocked_pattern("127.0.0.1"), Some(Cow::from("127.0.0.1")));
    /// assert_eq!(blocked.find_blocked_pattern("127.0.0.2"), None);
    /// ```
    pub fn find_blocked_pattern<'a>(&self, address: &'a str) -> Option<Cow<'a, str>> {
        let address_parts: Vec<&str> = address.split('.').collect();

        if self.is_pattern_blocked(&address) {
            return Some(Cow::Borrowed(address));
        }

        if is_ipv4(&address_parts) {
            (1..address_parts.len())
                .rev()
                .map(|i| format!("{}.*", address_parts[..i].join(".")))
                .find(|pattern| self.is_pattern_blocked(&pattern))
                .map(Cow::Owned)
        } else {
            (1..address_parts.len())
                .map(|i| format!("*.{}", address_parts[i..].join(".")))
                .find(|pattern| self.is_pattern_blocked(&pattern))
                .map(Cow::Owned)
        }
    }

    /// Check if the supplied address is in the blocklist.
    ///
    /// ## Example
    /// ```rust
    /// # use minecraft_utils::mojang_api::BlockedServers;
    /// let blocked = BlockedServers::fetch().unwrap();
    ///
    /// // Check if server is blocked
    /// assert!(blocked.is_blocked("mc.playmc.mx"));
    /// ```
    pub fn is_blocked(&self, address: &str) -> bool {
        self.find_blocked_pattern(address).is_some()
    }

    /// Check if a pattern is in the hashed pattern list.
    ///
    /// ## Example
    /// ```rust
    /// # use minecraft_utils::mojang_api::BlockedServers;
    /// # let blocked = BlockedServers {
    /// #     hashes: vec![
    /// #         // *.example.com
    /// #         String::from("8c7122d652cb7be22d1986f1f30b07fd5108d9c0"),
    /// #         // 192.0.*
    /// #         String::from("8c15fb642b3e8f58480df51798382f1016e748eb"),
    /// #         // 127.0.0.1
    /// #         String::from("4b84b15bff6ee5796152495a230e45e3d7e947d9"),
    /// #     ],
    /// # };
    /// // Using the blocked servers list from the struct example determine if the pattern is in the blocklist or not.
    /// assert!(blocked.is_pattern_blocked("*.example.com"));
    /// assert!(!blocked.is_pattern_blocked("example.com"));
    /// ```
    pub fn is_pattern_blocked(&self, pattern: &str) -> bool {
        let hash = format!("{:#02X}", Sha1::digest(pattern.as_bytes())).to_lowercase();
        self.hashes.contains(&Cow::Owned(hash))
    }
}

#[doc(hidden)]
/// Test if an address is ipv4 naively to better match how mojang determines if an address is ipv4 or not.
///
/// ## Example
/// ```rust
/// # use minecraft_utils::mojang_api::blocked_servers::is_ipv4;
/// assert!(!is_ipv4(&["mc", "example", "com"]));
/// assert!(is_ipv4(&["192", "0", "2", "235"]));
/// ```
pub fn is_ipv4(ip: &[&str]) -> bool {
    // If thare are too many sections, and each octet is a valid u8
    ip.len() == 4 && ip.iter().all(|x| x.parse::<u8>().is_ok())
}
