# \SubscriptionsApi

All URIs are relative to *https://api.pipedrive.com/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**add_recurring_subscription**](SubscriptionsApi.md#add_recurring_subscription) | **POST** /subscriptions/recurring | Add a recurring subscription
[**add_subscription_installment**](SubscriptionsApi.md#add_subscription_installment) | **POST** /subscriptions/installment | Add an installment subscription
[**cancel_recurring_subscription**](SubscriptionsApi.md#cancel_recurring_subscription) | **PUT** /subscriptions/recurring/{id}/cancel | Cancel a recurring subscription
[**delete_subscription**](SubscriptionsApi.md#delete_subscription) | **DELETE** /subscriptions/{id} | Delete a subscription
[**find_subscription_by_deal**](SubscriptionsApi.md#find_subscription_by_deal) | **GET** /subscriptions/find/{dealId} | Find subscription by deal
[**get_subscription**](SubscriptionsApi.md#get_subscription) | **GET** /subscriptions/{id} | Get details of a subscription
[**get_subscription_payments**](SubscriptionsApi.md#get_subscription_payments) | **GET** /subscriptions/{id}/payments | Get all payments of a subscription
[**update_recurring_subscription**](SubscriptionsApi.md#update_recurring_subscription) | **PUT** /subscriptions/recurring/{id} | Update a recurring subscription
[**update_subscription_installment**](SubscriptionsApi.md#update_subscription_installment) | **PUT** /subscriptions/installment/{id} | Update an installment subscription



## add_recurring_subscription

> crate::models::SubscriptionsIdResponse200 add_recurring_subscription(add_recurring_subscription_request)
Add a recurring subscription

Adds a new recurring subscription.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**add_recurring_subscription_request** | Option<[**AddRecurringSubscriptionRequest**](AddRecurringSubscriptionRequest.md)> |  |  |

### Return type

[**crate::models::SubscriptionsIdResponse200**](subscriptionsIdResponse200.md)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## add_subscription_installment

> crate::models::SubscriptionsIdResponse200 add_subscription_installment(add_subscription_installment_request)
Add an installment subscription

Adds a new installment subscription.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**add_subscription_installment_request** | Option<[**AddSubscriptionInstallmentRequest**](AddSubscriptionInstallmentRequest.md)> |  |  |

### Return type

[**crate::models::SubscriptionsIdResponse200**](subscriptionsIdResponse200.md)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## cancel_recurring_subscription

> crate::models::SubscriptionsIdResponse200 cancel_recurring_subscription(id, cancel_recurring_subscription_request)
Cancel a recurring subscription

Cancels a recurring subscription.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | The ID of the subscription | [required] |
**cancel_recurring_subscription_request** | Option<[**CancelRecurringSubscriptionRequest**](CancelRecurringSubscriptionRequest.md)> |  |  |

### Return type

[**crate::models::SubscriptionsIdResponse200**](subscriptionsIdResponse200.md)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_subscription

> crate::models::SubscriptionsIdResponse200 delete_subscription(id)
Delete a subscription

Marks an installment or a recurring subscription as deleted.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | The ID of the subscription | [required] |

### Return type

[**crate::models::SubscriptionsIdResponse200**](subscriptionsIdResponse200.md)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## find_subscription_by_deal

> crate::models::SubscriptionsIdResponse200 find_subscription_by_deal(deal_id)
Find subscription by deal

Returns details of an installment or a recurring subscription by the deal ID.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**deal_id** | **i32** | The ID of the deal | [required] |

### Return type

[**crate::models::SubscriptionsIdResponse200**](subscriptionsIdResponse200.md)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_subscription

> crate::models::SubscriptionsIdResponse200 get_subscription(id)
Get details of a subscription

Returns details of an installment or a recurring subscription.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | The ID of the subscription | [required] |

### Return type

[**crate::models::SubscriptionsIdResponse200**](subscriptionsIdResponse200.md)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_subscription_payments

> crate::models::PaymentResponse200 get_subscription_payments(id)
Get all payments of a subscription

Returns all payments of an installment or recurring subscription.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | The ID of the subscription | [required] |

### Return type

[**crate::models::PaymentResponse200**](paymentResponse200.md)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_recurring_subscription

> crate::models::SubscriptionsIdResponse200 update_recurring_subscription(id, update_recurring_subscription_request)
Update a recurring subscription

Updates a recurring subscription.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | The ID of the subscription | [required] |
**update_recurring_subscription_request** | Option<[**UpdateRecurringSubscriptionRequest**](UpdateRecurringSubscriptionRequest.md)> |  |  |

### Return type

[**crate::models::SubscriptionsIdResponse200**](subscriptionsIdResponse200.md)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_subscription_installment

> crate::models::SubscriptionsIdResponse200 update_subscription_installment(id, update_subscription_installment_request)
Update an installment subscription

Updates an installment subscription.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | The ID of the subscription | [required] |
**update_subscription_installment_request** | Option<[**UpdateSubscriptionInstallmentRequest**](UpdateSubscriptionInstallmentRequest.md)> |  |  |

### Return type

[**crate::models::SubscriptionsIdResponse200**](subscriptionsIdResponse200.md)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

