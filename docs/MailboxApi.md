# \MailboxApi

All URIs are relative to *https://api.pipedrive.com/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**delete_mail_thread**](MailboxApi.md#delete_mail_thread) | **DELETE** /mailbox/mailThreads/{id} | Delete mail thread
[**get_mail_message**](MailboxApi.md#get_mail_message) | **GET** /mailbox/mailMessages/{id} | Get one mail message
[**get_mail_thread**](MailboxApi.md#get_mail_thread) | **GET** /mailbox/mailThreads/{id} | Get one mail thread
[**get_mail_thread_messages**](MailboxApi.md#get_mail_thread_messages) | **GET** /mailbox/mailThreads/{id}/mailMessages | Get all mail messages of mail thread
[**get_mail_threads**](MailboxApi.md#get_mail_threads) | **GET** /mailbox/mailThreads | Get mail threads
[**update_mail_thread_details**](MailboxApi.md#update_mail_thread_details) | **PUT** /mailbox/mailThreads/{id} | Update mail thread details



## delete_mail_thread

> crate::models::DeleteMailThreadResponse200 delete_mail_thread(id)
Delete mail thread

Marks a mail thread as deleted.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | The ID of the mail thread | [required] |

### Return type

[**crate::models::DeleteMailThreadResponse200**](deleteMailThreadResponse200.md)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_mail_message

> crate::models::MailMessageResponse200 get_mail_message(id, include_body)
Get one mail message

Returns data about a specific mail message.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | The ID of the mail message to fetch | [required] |
**include_body** | Option<**f32**> | Whether to include the full message body or not. `0` = Don't include, `1` = Include |  |[default to 0]

### Return type

[**crate::models::MailMessageResponse200**](mailMessageResponse200.md)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_mail_thread

> crate::models::GetOneMailThreadResponse200 get_mail_thread(id)
Get one mail thread

Returns a specific mail thread.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | The ID of the mail thread | [required] |

### Return type

[**crate::models::GetOneMailThreadResponse200**](getOneMailThreadResponse200.md)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_mail_thread_messages

> crate::models::GetAllMailMessagesOfMailThreadResponse200 get_mail_thread_messages(id)
Get all mail messages of mail thread

Returns all the mail messages inside a specified mail thread.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | The ID of the mail thread | [required] |

### Return type

[**crate::models::GetAllMailMessagesOfMailThreadResponse200**](getAllMailMessagesOfMailThreadResponse200.md)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_mail_threads

> crate::models::GetMailThreadsResponse200 get_mail_threads(folder, start, limit)
Get mail threads

Returns mail threads in a specified folder ordered by the most recent message within.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**folder** | **String** | The type of folder to fetch | [required] |[default to inbox]
**start** | Option<**i32**> | Pagination start |  |[default to 0]
**limit** | Option<**i32**> | Items shown per page |  |

### Return type

[**crate::models::GetMailThreadsResponse200**](getMailThreadsResponse200.md)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_mail_thread_details

> crate::models::UpdateMailThreadDetailsResponse200 update_mail_thread_details(id, deal_id, lead_id, shared_flag, read_flag, archived_flag)
Update mail thread details

Updates the properties of a mail thread.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | The ID of the mail thread | [required] |
**deal_id** | Option<**i32**> | The ID of the deal this thread is associated with |  |
**lead_id** | Option<**uuid::Uuid**> | The ID of the lead this thread is associated with |  |
**shared_flag** | Option<**f32**> |  |  |
**read_flag** | Option<**f32**> |  |  |
**archived_flag** | Option<**f32**> |  |  |

### Return type

[**crate::models::UpdateMailThreadDetailsResponse200**](updateMailThreadDetailsResponse200.md)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/x-www-form-urlencoded
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

