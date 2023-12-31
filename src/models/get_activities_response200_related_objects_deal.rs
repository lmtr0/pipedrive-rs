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
pub struct GetActivitiesResponse200RelatedObjectsDeal {
    #[serde(rename = "DEAL_ID", skip_serializing_if = "Option::is_none")]
    pub deal_id: Option<Box<crate::models::GetActivitiesResponse200RelatedObjectsDealDealId>>,
}

impl GetActivitiesResponse200RelatedObjectsDeal {
    pub fn new() -> GetActivitiesResponse200RelatedObjectsDeal {
        GetActivitiesResponse200RelatedObjectsDeal {
            deal_id: None,
        }
    }
}


