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
pub struct UpdateDealProductRequest {
    /// The ID of the product to use
    #[serde(rename = "product_id", skip_serializing_if = "Option::is_none")]
    pub product_id: Option<i32>,
    /// The price at which this product will be added to the deal
    #[serde(rename = "item_price", skip_serializing_if = "Option::is_none")]
    pub item_price: Option<f32>,
    /// How many items of this product will be added to the deal
    #[serde(rename = "quantity", skip_serializing_if = "Option::is_none")]
    pub quantity: Option<i32>,
    /// The value of the discount. The `discount_type` field can be used to specify whether the value is an amount or a percentage.
    #[serde(rename = "discount", skip_serializing_if = "Option::is_none")]
    pub discount: Option<f32>,
    /// The type of the discount's value.
    #[serde(rename = "discount_type", skip_serializing_if = "Option::is_none")]
    pub discount_type: Option<DiscountType>,
    /// The duration of the product
    #[serde(rename = "duration", skip_serializing_if = "Option::is_none")]
    pub duration: Option<f32>,
    #[serde(rename = "duration_unit", skip_serializing_if = "Option::is_none")]
    pub duration_unit: Option<DurationUnit>,
    /// The ID of the product variation to use. When omitted, no variation will be used.
    #[serde(rename = "product_variation_id", skip_serializing_if = "Option::is_none")]
    pub product_variation_id: Option<i32>,
    /// A textual comment associated with this product-deal attachment
    #[serde(rename = "comments", skip_serializing_if = "Option::is_none")]
    pub comments: Option<String>,
    /// The tax percentage
    #[serde(rename = "tax", skip_serializing_if = "Option::is_none")]
    pub tax: Option<f32>,
    /// The tax option to be applied to the products. When using `inclusive`, the tax percentage will already be included in the price. When using `exclusive`, the tax will not be included in the price. When using `none`, no tax will be added. Use the `tax` field for defining the tax percentage amount.
    #[serde(rename = "tax_method", skip_serializing_if = "Option::is_none")]
    pub tax_method: Option<TaxMethod>,
    /// Whether the product is enabled for a deal or not. This makes it possible to add products to a deal with a specific price and discount criteria, but keep them disabled, which refrains them from being included in the deal value calculation. When omitted, the product will be marked as enabled by default.
    #[serde(rename = "enabled_flag", skip_serializing_if = "Option::is_none")]
    pub enabled_flag: Option<bool>,
}

impl UpdateDealProductRequest {
    pub fn new() -> UpdateDealProductRequest {
        UpdateDealProductRequest {
            product_id: None,
            item_price: None,
            quantity: None,
            discount: None,
            discount_type: None,
            duration: None,
            duration_unit: None,
            product_variation_id: None,
            comments: None,
            tax: None,
            tax_method: None,
            enabled_flag: None,
        }
    }
}

/// The type of the discount's value.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum DiscountType {
    #[serde(rename = "percentage")]
    Percentage,
    #[serde(rename = "amount")]
    Amount,
}

impl Default for DiscountType {
    fn default() -> DiscountType {
        Self::Percentage
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum DurationUnit {
    #[serde(rename = "hourly")]
    Hourly,
    #[serde(rename = "daily")]
    Daily,
    #[serde(rename = "weekly")]
    Weekly,
    #[serde(rename = "monthly")]
    Monthly,
    #[serde(rename = "yearly")]
    Yearly,
}

impl Default for DurationUnit {
    fn default() -> DurationUnit {
        Self::Hourly
    }
}
/// The tax option to be applied to the products. When using `inclusive`, the tax percentage will already be included in the price. When using `exclusive`, the tax will not be included in the price. When using `none`, no tax will be added. Use the `tax` field for defining the tax percentage amount.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum TaxMethod {
    #[serde(rename = "exclusive")]
    Exclusive,
    #[serde(rename = "inclusive")]
    Inclusive,
    #[serde(rename = "none")]
    None,
}

impl Default for TaxMethod {
    fn default() -> TaxMethod {
        Self::Exclusive
    }
}

