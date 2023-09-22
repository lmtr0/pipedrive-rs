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
pub struct ListProductsResponse200AllOfAdditionalData {
    /// The total quantity of the products
    #[serde(rename = "products_quantity_total", skip_serializing_if = "Option::is_none")]
    pub products_quantity_total: Option<i32>,
    /// The total sum of the products
    #[serde(rename = "products_sum_total", skip_serializing_if = "Option::is_none")]
    pub products_sum_total: Option<i32>,
    /// The total formatted quantity of the products
    #[serde(rename = "products_quantity_total_formatted", skip_serializing_if = "Option::is_none")]
    pub products_quantity_total_formatted: Option<String>,
    /// The total formatted sum of the products
    #[serde(rename = "products_sum_total_formatted", skip_serializing_if = "Option::is_none")]
    pub products_sum_total_formatted: Option<String>,
    #[serde(rename = "pagination", skip_serializing_if = "Option::is_none")]
    pub pagination: Option<Box<crate::models::GetActivitiesResponse200AdditionalDataPagination>>,
}

impl ListProductsResponse200AllOfAdditionalData {
    pub fn new() -> ListProductsResponse200AllOfAdditionalData {
        ListProductsResponse200AllOfAdditionalData {
            products_quantity_total: None,
            products_sum_total: None,
            products_quantity_total_formatted: None,
            products_sum_total_formatted: None,
            pagination: None,
        }
    }
}


