use bitflags::bitflags;
use minreq::Method;
use serde::{Deserialize, Serialize};

use crate::mojang_api::{client::fetch, error::ApiError};

bitflags! {
    /// Flags for different metrics on Mojang's games.
    pub struct Metrics: u32 {
        #[allow(missing_docs)] const MINECRAFT_ITEMS_SOLD             = 0b00000001;
        #[allow(missing_docs)] const MINECRAFT_PREPAID_CARDS_REDEEMED = 0b00000010;
        #[allow(missing_docs)] const COBALT_ITEMS_SOLD                = 0b00000100;
        #[allow(missing_docs)] const COBALT_PREPAID_CARDS_REDEEMED    = 0b00001000;
        #[allow(missing_docs)] const SCROLLS_ITEMS_SOLD               = 0b00010000;
        #[allow(missing_docs)] const DUNGEONS_ITEM_SOLD               = 0b00100000;
    }
}

#[doc(hidden)]
pub mod keys {
    pub const MINECRAFT_ITEMS_SOLD: &'static str = "item_sold_minecraft";
    pub const MINECRAFT_PREPAID_CARDS_REDEEMED: &'static str = "prepaid_card_redeemed_minecraft";
    pub const COBALT_ITEMS_SOLD: &'static str = "item_sold_cobalt";
    pub const COBALT_PREPAID_CARDS_REDEEMED: &'static str = "prepaid_card_redeemed_cobalt";
    pub const SCROLLS_ITEMS_SOLD: &'static str = "item_sold_scrolls";
    pub const DUNGEONS_ITEM_SOLD: &'static str = "item_sold_dungeons";
}

macro_rules! add_keys {
    ($self:ident, $vec:ident, [$($name:ident),*]) => {
        $(if $self.contains(Self::$name) {
            $vec.push(keys::$name)
        })*
    };
}

impl Metrics {
    /// Combined total of minecraft items sold, and cards redeemed
    pub fn minecraft() -> Self {
        Self::MINECRAFT_ITEMS_SOLD | Self::MINECRAFT_PREPAID_CARDS_REDEEMED
    }

    /// Combined total of cobalt items sold, and cards redeemed
    pub fn cobalt() -> Self {
        Self::COBALT_ITEMS_SOLD | Self::COBALT_PREPAID_CARDS_REDEEMED
    }

    /// The amount of scrolls items sold
    pub fn scrolls() -> Self {
        Self::SCROLLS_ITEMS_SOLD
    }

    /// The amount of dungeon items sold
    pub fn dungeons() -> Self {
        Self::DUNGEONS_ITEM_SOLD
    }
}

impl Into<Vec<&'static str>> for Metrics {
    fn into(self) -> Vec<&'static str> {
        let mut vec = vec![];
        add_keys!(
            self,
            vec,
            [
                MINECRAFT_ITEMS_SOLD,
                MINECRAFT_PREPAID_CARDS_REDEEMED,
                COBALT_ITEMS_SOLD,
                COBALT_PREPAID_CARDS_REDEEMED,
                SCROLLS_ITEMS_SOLD,
                DUNGEONS_ITEM_SOLD
            ]
        );
        vec
    }
}

/// Statistics on the sales of Mojang's games.
///
/// ## Example
/// ```rust
/// use minecraft_utils::mojang_api::stats::{ Stats, Metrics };
///
/// let stats = Stats::fetch(Metrics::minecraft()).unwrap();
///
/// assert!(stats.total > 1000);
/// ```
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Stats {
    /// The total amount sold.
    pub total: u64,
    /// The total amount sold in the last 24 hours.
    pub last24h: u64,
    /// The average amount of sales per second.
    #[serde(rename = "saleVelocityPerSeconds")]
    pub sale_velocity_per_seconds: f32,
}

#[doc(hidden)]
#[derive(Serialize, Debug)]
pub struct Payload {
    #[serde(rename = "metricKeys")]
    metric_keys: Vec<&'static str>,
}

impl Stats {
    /// Get statistics on the sales of Mojang's games.
    ///
    /// ## Example
    /// ```rust
    /// # use minecraft_utils::mojang_api::stats::{ Stats, Metrics };
    /// let stats = Stats::fetch(Metrics::minecraft()).unwrap();
    ///
    /// assert!(stats.total > 1000);
    /// ```
    pub fn fetch(keys: Metrics) -> Result<Self, ApiError> {
        let body = Payload {
            metric_keys: keys.into(),
        };

        let res = fetch(Method::Post, "https://api.mojang.com/orders/statistics")
            .with_json(&body)?
            .send()?;

        if res.status_code == 200 {
            Ok(res.json()?)
        } else {
            Err(ApiError::Request {
                status: res.status_code,
                reason: res.reason_phrase,
            })
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_metric_keys_conversion() {
        let keys: Vec<_> = Metrics::all().into();
        assert_eq!(
            keys,
            vec![
                "item_sold_minecraft",
                "prepaid_card_redeemed_minecraft",
                "item_sold_cobalt",
                "prepaid_card_redeemed_cobalt",
                "item_sold_scrolls",
                "item_sold_dungeons",
            ]
        );

        let keys: Vec<_> =
            (Metrics::MINECRAFT_ITEMS_SOLD | Metrics::MINECRAFT_PREPAID_CARDS_REDEEMED).into();

        assert_eq!(
            keys,
            vec!["item_sold_minecraft", "prepaid_card_redeemed_minecraft",]
        );
    }
}
