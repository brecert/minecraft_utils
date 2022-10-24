use serde::{de, Deserialize, Deserializer, Serialize};

use crate::mojang_api::{client::get, error::ApiError};

fn deserialize_textures_entry<'de, D>(ty: D) -> Result<TexturesEntry, D::Error>
where
    D: Deserializer<'de>,
{
    // should be fine
    let mut buf = [0u8; 768];
    let str = String::deserialize(ty)?;
    let len =
        base64::decode_config_slice(&str, base64::STANDARD, &mut buf).map_err(de::Error::custom)?;
    serde_json::from_slice(&buf[..len]).map_err(de::Error::custom)
}

/// More complex user information
///
/// ## Example
/// ```rust
/// use minecraft_utils::mojang_api::Profile;
///
/// let profile = Profile::fetch("7a8084cd1f444a159bb1eef8d5b535a1").unwrap();
///
/// assert_eq!(
///     profile.textures().skin.url,
///     "http://textures.minecraft.net/texture/b8130282b80cc08872bfc858975350ab3f3fcd4b1d18717bfb5b7b838fce4eaa"
/// );
/// ```
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Profile {
    /// The UUID of the user.
    pub id: String,

    /// The username of the user.
    pub name: String,

    /// Properties associated with the user.
    pub properties: [ProfileProperty; 1],

    /// If the account is a legacy account or not.
    #[serde(default)]
    pub legacy: bool,
}

impl Profile {
    /// Fetches the user profile.
    pub fn fetch(uuid: &str) -> Result<Self, ApiError> {
        let url = format!(
            "https://sessionserver.mojang.com/session/minecraft/profile/{}",
            uuid
        );
        Ok(get(url)?.json()?)
    }

    /// Returns texture information of the user.
    pub fn textures(&self) -> &Textures {
        &self.properties[0].value.textures
    }

    /// Returns if the model of the user is slim or not.
    pub fn slim_model(&self) -> bool {
        let is_slim = self
            .textures()
            .skin
            .metadata
            .as_ref()
            .map(|m| m.model == "slim");

        matches!(is_slim, Some(true))
    }
}

/// A property associated with the user, currently only supports textures.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct ProfileProperty {
    /// Name of the property.
    pub name: String,

    /// The value property.
    #[serde(deserialize_with = "deserialize_textures_entry")]
    pub value: TexturesEntry,
}

/// A texture entry in the properties.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct TexturesEntry {
    /// When the entry was served.
    pub timestamp: i64,

    /// UUID of the user.
    #[serde(rename = "profileId")]
    pub profile_id: String,

    /// username of the user.
    #[serde(rename = "profileName")]
    pub profile_name: String,

    /// Texture information for the user.
    pub textures: Textures,
}

/// Texture information for the user.
///
/// If the user does not have a cape texture then it will be [None].
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Textures {
    /// Information on the skin, such as the texture url, and model the skin uses.
    #[serde(rename = "SKIN")]
    pub skin: SkinData,

    /// Information on the user's cape, will be [None] if the user does not have a cape.
    #[serde(rename = "CAPE")]
    pub cape: Option<CapeData>,
}

/// Information relating to the skin of a user.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct SkinData {
    /// The url of the skin texture.
    pub url: String,

    /// Metadata relating to the skin, such as the model used for the skin.
    pub metadata: Option<SkinMetadata>,
}

/// Information relating to the cape of a user.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct CapeData {
    /// The url of the cape texture.
    pub url: String,
}

/// Metadata relating to the skin, such as the model used for the skin
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct SkinMetadata {
    /// The model used for the skin.
    pub model: String,
}

/// A username change entry
///
/// if `changed_to_at` is [None] then it is the original name as the name was never changed to from a previous one.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct UsernameEntry {
    /// The username
    pub name: String,

    /// A unix timestamp (in ms) representing when the username was changed to the current entry.
    #[serde(rename = "changedToAt")]
    pub changed_to_at: Option<u64>,
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse() {
        let json = r#"{
            "id" : "7a8084cd1f444a159bb1eef8d5b535a1",
            "name" : "brecert",
            "properties" : [ {
              "name" : "textures",
              "value" : "ewogICJ0aW1lc3RhbXAiIDogMTY0MDMyNjE1MTg1OSwKICAicHJvZmlsZUlkIiA6ICI3YTgwODRjZDFmNDQ0YTE1OWJiMWVlZjhkNWI1MzVhMSIsCiAgInByb2ZpbGVOYW1lIiA6ICJicmVjZXJ0IiwKICAidGV4dHVyZXMiIDogewogICAgIlNLSU4iIDogewogICAgICAidXJsIiA6ICJodHRwOi8vdGV4dHVyZXMubWluZWNyYWZ0Lm5ldC90ZXh0dXJlL2I4MTMwMjgyYjgwY2MwODg3MmJmYzg1ODk3NTM1MGFiM2YzZmNkNGIxZDE4NzE3YmZiNWI3YjgzOGZjZTRlYWEiLAogICAgICAibWV0YWRhdGEiIDogewogICAgICAgICJtb2RlbCIgOiAic2xpbSIKICAgICAgfQogICAgfQogIH0KfQ=="
            } ]
          }"#;

        let profile = serde_json::from_str::<Profile>(&json).unwrap();

        assert_eq!(profile.textures().skin.url, "http://textures.minecraft.net/texture/b8130282b80cc08872bfc858975350ab3f3fcd4b1d18717bfb5b7b838fce4eaa")
    }
}
