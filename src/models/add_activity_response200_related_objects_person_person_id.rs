/*
 * Pipedrive API v1
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 1.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// AddActivityResponse200RelatedObjectsPersonPersonId : The ID of the person associated with the item



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AddActivityResponse200RelatedObjectsPersonPersonId {
    /// Whether the associated person is active or not
    #[serde(rename = "active_flag", skip_serializing_if = "Option::is_none")]
    pub active_flag: Option<bool>,
    /// The ID of the person associated with the item
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    /// The name of the person associated with the item
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// The emails of the person associated with the item
    #[serde(rename = "email", skip_serializing_if = "Option::is_none")]
    pub email: Option<Vec<crate::models::GetActivitiesResponse200RelatedObjectsPersonPersonIdAllOfEmailInner>>,
    /// The phone numbers of the person associated with the item
    #[serde(rename = "phone", skip_serializing_if = "Option::is_none")]
    pub phone: Option<Vec<crate::models::GetActivitiesResponse200RelatedObjectsPersonPersonIdAllOfPhoneInner>>,
    /// The ID of the owner of the person that is associated with the item
    #[serde(rename = "owner_id", skip_serializing_if = "Option::is_none")]
    pub owner_id: Option<i32>,
}

impl AddActivityResponse200RelatedObjectsPersonPersonId {
    /// The ID of the person associated with the item
    pub fn new() -> AddActivityResponse200RelatedObjectsPersonPersonId {
        AddActivityResponse200RelatedObjectsPersonPersonId {
            active_flag: None,
            id: None,
            name: None,
            email: None,
            phone: None,
            owner_id: None,
        }
    }
}


