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
pub struct AddFilterRequest {
    /// The name of the filter
    #[serde(rename = "name")]
    pub name: String,
    /// The conditions of the filter as a JSON object. Please note that a maximum of 16 conditions is allowed per filter and `date` values must be supplied in the `YYYY-MM-DD` format. It requires a minimum structure as follows: `{\"glue\":\"and\",\"conditions\":[{\"glue\":\"and\",\"conditions\": [CONDITION_OBJECTS]},{\"glue\":\"or\",\"conditions\":[CONDITION_OBJECTS]}]}`. Replace `CONDITION_OBJECTS` with JSON objects of the following structure: `{\"object\":\"\",\"field_id\":\"\", \"operator\":\"\",\"value\":\"\", \"extra_value\":\"\"}` or leave the array empty. Depending on the object type you should use another API endpoint to get `field_id`. There are five types of objects you can choose from: `\"person\"`, `\"deal\"`, `\"organization\"`, `\"product\"`, `\"activity\"` and you can use these types of operators depending on what type of a field you have: `\"IS NOT NULL\"`, `\"IS NULL\"`, `\"<=\"`, `\">=\"`, `\"<\"`, `\">\"`, `\"!=\"`, `\"=\"`, `\"LIKE '$%'\"`, `\"LIKE '%$%'\"`, `\"NOT LIKE '$%'\"`. To get a better understanding of how filters work try creating them directly from the Pipedrive application.
    #[serde(rename = "conditions")]
    pub conditions: serde_json::Value,
    #[serde(rename = "type")]
    pub r#type: Type,
}

impl AddFilterRequest {
    pub fn new(name: String, conditions: serde_json::Value, r#type: Type) -> AddFilterRequest {
        AddFilterRequest {
            name,
            conditions,
            r#type,
        }
    }
}

/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "deals")]
    Deals,
    #[serde(rename = "leads")]
    Leads,
    #[serde(rename = "org")]
    Org,
    #[serde(rename = "people")]
    People,
    #[serde(rename = "products")]
    Products,
    #[serde(rename = "activity")]
    Activity,
}

impl Default for Type {
    fn default() -> Type {
        Self::Deals
    }
}

