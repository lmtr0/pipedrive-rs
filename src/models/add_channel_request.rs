/*
 * Pipedrive API v1
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 1.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AddChannelRequest {
    /// The name of the channel
    #[serde(rename = "name")]
    pub name: String,
    /// The channel ID
    #[serde(rename = "provider_channel_id")]
    pub provider_channel_id: String,
    /// The URL for an icon that represents your channel
    #[serde(rename = "avatar_url", skip_serializing_if = "Option::is_none")]
    pub avatar_url: Option<String>,
    /// If true, enables templates logic on UI. Requires getTemplates endpoint implemented. Find out more [here](https://pipedrive.readme.io/docs/implementing-messaging-app-extension).
    #[serde(rename = "template_support", skip_serializing_if = "Option::is_none")]
    pub template_support: Option<bool>,
    /// It controls the icons (like the icon next to the conversation)
    #[serde(rename = "provider_type", skip_serializing_if = "Option::is_none")]
    pub provider_type: Option<ProviderType>,
}

impl AddChannelRequest {
    pub fn new(name: String, provider_channel_id: String) -> AddChannelRequest {
        AddChannelRequest {
            name,
            provider_channel_id,
            avatar_url: None,
            template_support: None,
            provider_type: None,
        }
    }
}

/// It controls the icons (like the icon next to the conversation)
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum ProviderType {
    #[serde(rename = "facebook")]
    Facebook,
    #[serde(rename = "whatsapp")]
    Whatsapp,
    #[serde(rename = "other")]
    Other,
}

impl Default for ProviderType {
    fn default() -> ProviderType {
        Self::Facebook
    }
}

