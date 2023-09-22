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
pub struct GetUserSettingsResponse200AllOfData {
    /// If the vendors are allowed to install custom Marketplace apps with functionality and content in the Pipedrive UI or not
    #[serde(rename = "marketplace_allow_custom_install_url", skip_serializing_if = "Option::is_none")]
    pub marketplace_allow_custom_install_url: Option<bool>,
    /// If the vendors are allowed to extend their Marketplace apps with functionality and content in the Pipedrive UI or not
    #[serde(rename = "marketplace_app_extensions_vendor", skip_serializing_if = "Option::is_none")]
    pub marketplace_app_extensions_vendor: Option<bool>,
    /// If the vendors are allowed to be part of the Marketplace team or not
    #[serde(rename = "marketplace_team", skip_serializing_if = "Option::is_none")]
    pub marketplace_team: Option<bool>,
    /// The number of results shown in list by default
    #[serde(rename = "list_limit", skip_serializing_if = "Option::is_none")]
    pub list_limit: Option<f32>,
    /// Whether beta app is enabled
    #[serde(rename = "beta_app", skip_serializing_if = "Option::is_none")]
    pub beta_app: Option<bool>,
    /// Prevent salesphone call to override
    #[serde(rename = "prevent_salesphone_callto_override", skip_serializing_if = "Option::is_none")]
    pub prevent_salesphone_callto_override: Option<bool>,
    /// The destination of file upload
    #[serde(rename = "file_upload_destination", skip_serializing_if = "Option::is_none")]
    pub file_upload_destination: Option<String>,
    /// The call to link syntax
    #[serde(rename = "callto_link_syntax", skip_serializing_if = "Option::is_none")]
    pub callto_link_syntax: Option<String>,
    /// Whether the expected close date of the deal is filled automatically or not
    #[serde(rename = "autofill_deal_expected_close_date", skip_serializing_if = "Option::is_none")]
    pub autofill_deal_expected_close_date: Option<bool>,
    /// Allow the vendors to duplicate a person
    #[serde(rename = "person_duplicate_condition", skip_serializing_if = "Option::is_none")]
    pub person_duplicate_condition: Option<String>,
}

impl GetUserSettingsResponse200AllOfData {
    pub fn new() -> GetUserSettingsResponse200AllOfData {
        GetUserSettingsResponse200AllOfData {
            marketplace_allow_custom_install_url: None,
            marketplace_app_extensions_vendor: None,
            marketplace_team: None,
            list_limit: None,
            beta_app: None,
            prevent_salesphone_callto_override: None,
            file_upload_destination: None,
            callto_link_syntax: None,
            autofill_deal_expected_close_date: None,
            person_duplicate_condition: None,
        }
    }
}


