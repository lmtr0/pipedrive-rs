# AddProductRequest1

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | **String** | The name of the product | 
**code** | Option<**String**> | The product code | [optional]
**unit** | Option<**String**> | The unit in which this product is sold | [optional]
**tax** | Option<**f32**> | The tax percentage | [optional][default to 0]
**active_flag** | Option<**bool**> | Whether this product will be made active or not | [optional][default to true]
**selectable** | Option<**bool**> | Whether this product can be selected in deals or not | [optional][default to true]
**visible_to** | Option<**String**> |  | [optional]
**owner_id** | Option<**i32**> | The ID of the user who will be marked as the owner of this product. When omitted, the authorized user ID will be used. | [optional]
**prices** | Option<[**Vec<serde_json::Value>**](serde_json::Value.md)> | An array of objects, each containing: `currency` (string), `price` (number), `cost` (number, optional), `overhead_cost` (number, optional). Note that there can only be one price per product per currency. When `prices` is omitted altogether, a default price of 0 and a default currency based on the company's currency will be assigned. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


