# AddDealProductRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**product_id** | **i32** | The ID of the product to use | 
**item_price** | **f32** | The price at which this product will be added to the deal | 
**quantity** | **i32** | Quantity – e.g. how many items of this product will be added to the deal | 
**discount** | Option<**f32**> | The value of the discount. The `discount_type` field can be used to specify whether the value is an amount or a percentage. | [optional][default to 0]
**discount_type** | Option<**String**> | The type of the discount's value. | [optional][default to Percentage]
**duration** | Option<**f32**> | The duration of the product. If omitted, will be set to 1. | [optional][default to 1]
**duration_unit** | Option<**String**> |  | [optional]
**product_variation_id** | Option<**i32**> | The ID of the product variation to use. When omitted, no variation will be used. | [optional]
**comments** | Option<**String**> | A textual comment associated with this product-deal attachment | [optional]
**tax** | Option<**f32**> | The tax percentage | [optional][default to 0]
**tax_method** | Option<**String**> | The tax option to be applied to the products. When using `inclusive`, the tax percentage will already be included in the price. When using `exclusive`, the tax will not be included in the price. When using `none`, no tax will be added. Use the `tax` field for defining the tax percentage amount. By default, the user setting value for tax options will be used. Changing this in one product affects the rest of the products attached to the deal. | [optional]
**enabled_flag** | Option<**bool**> | Whether the product is enabled for a deal or not. This makes it possible to add products to a deal with a specific price and discount criteria, but keep them disabled, which refrains them from being included in the deal value calculation. When omitted, the product will be marked as enabled by default. | [optional][default to true]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


