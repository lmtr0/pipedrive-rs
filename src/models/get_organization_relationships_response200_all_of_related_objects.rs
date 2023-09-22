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
pub struct GetOrganizationRelationshipsResponse200AllOfRelatedObjects {
    #[serde(rename = "organization", skip_serializing_if = "Option::is_none")]
    pub organization: Option<Box<crate::models::GetActivitiesResponse200RelatedObjectsOrganization>>,
}

impl GetOrganizationRelationshipsResponse200AllOfRelatedObjects {
    pub fn new() -> GetOrganizationRelationshipsResponse200AllOfRelatedObjects {
        GetOrganizationRelationshipsResponse200AllOfRelatedObjects {
            organization: None,
        }
    }
}


