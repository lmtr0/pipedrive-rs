# UpdateRecurringSubscriptionRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**description** | Option<**String**> | The description of the recurring subscription | [optional]
**cycle_amount** | Option<**i32**> | The amount of each payment | [optional]
**payments** | Option<[**Vec<serde_json::Value>**](serde_json::Value.md)> | Array of additional payments. It requires a minimum structure as follows: [{ amount:SUM, description:DESCRIPTION, due_at:PAYMENT_DATE }]. Replace SUM with a payment amount, DESCRIPTION with an explanation string, PAYMENT_DATE with a date (format YYYY-MM-DD). | [optional]
**update_deal_value** | Option<**bool**> | Indicates that the deal value must be set to recurring subscription's MRR value | [optional]
**effective_date** | [**String**](string.md) | All payments after that date will be affected. Format: YYYY-MM-DD | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


