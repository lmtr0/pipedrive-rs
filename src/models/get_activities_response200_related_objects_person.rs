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
pub struct GetActivitiesResponse200RelatedObjectsPerson {
    #[serde(rename = "PERSON_ID", skip_serializing_if = "Option::is_none")]
    pub person_id: Option<Box<crate::models::GetActivitiesResponse200RelatedObjectsPersonPersonId>>,
}

impl GetActivitiesResponse200RelatedObjectsPerson {
    pub fn new() -> GetActivitiesResponse200RelatedObjectsPerson {
        GetActivitiesResponse200RelatedObjectsPerson {
            person_id: None,
        }
    }
}


