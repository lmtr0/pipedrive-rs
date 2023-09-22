# \OrganizationsApi

All URIs are relative to *https://api.pipedrive.com/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**add_organization**](OrganizationsApi.md#add_organization) | **POST** /organizations | Add an organization
[**add_organization_follower**](OrganizationsApi.md#add_organization_follower) | **POST** /organizations/{id}/followers | Add a follower to an organization
[**delete_organization**](OrganizationsApi.md#delete_organization) | **DELETE** /organizations/{id} | Delete an organization
[**delete_organization_follower**](OrganizationsApi.md#delete_organization_follower) | **DELETE** /organizations/{id}/followers/{follower_id} | Delete a follower from an organization
[**delete_organizations**](OrganizationsApi.md#delete_organizations) | **DELETE** /organizations | Delete multiple organizations in bulk
[**get_organization**](OrganizationsApi.md#get_organization) | **GET** /organizations/{id} | Get details of an organization
[**get_organization_activities**](OrganizationsApi.md#get_organization_activities) | **GET** /organizations/{id}/activities | List activities associated with an organization
[**get_organization_deals**](OrganizationsApi.md#get_organization_deals) | **GET** /organizations/{id}/deals | List deals associated with an organization
[**get_organization_files**](OrganizationsApi.md#get_organization_files) | **GET** /organizations/{id}/files | List files attached to an organization
[**get_organization_followers**](OrganizationsApi.md#get_organization_followers) | **GET** /organizations/{id}/followers | List followers of an organization
[**get_organization_mail_messages**](OrganizationsApi.md#get_organization_mail_messages) | **GET** /organizations/{id}/mailMessages | List mail messages associated with an organization
[**get_organization_persons**](OrganizationsApi.md#get_organization_persons) | **GET** /organizations/{id}/persons | List persons of an organization
[**get_organization_updates**](OrganizationsApi.md#get_organization_updates) | **GET** /organizations/{id}/flow | List updates about an organization
[**get_organization_users**](OrganizationsApi.md#get_organization_users) | **GET** /organizations/{id}/permittedUsers | List permitted users
[**get_organizations**](OrganizationsApi.md#get_organizations) | **GET** /organizations | Get all organizations
[**get_organizations_collection**](OrganizationsApi.md#get_organizations_collection) | **GET** /organizations/collection | Get all organizations (BETA)
[**merge_organizations**](OrganizationsApi.md#merge_organizations) | **PUT** /organizations/{id}/merge | Merge two organizations
[**search_organization**](OrganizationsApi.md#search_organization) | **GET** /organizations/search | Search organizations
[**update_organization**](OrganizationsApi.md#update_organization) | **PUT** /organizations/{id} | Update an organization



## add_organization

> crate::models::AddOrganizationResponse200 add_organization(add_organization_request)
Add an organization

Adds a new organization. Note that you can supply additional custom fields along with the request that are not described here. These custom fields are different for each Pipedrive account and can be recognized by long hashes as keys. To determine which custom fields exists, fetch the organizationFields and look for `key` values. For more information, see the tutorial for <a href=\"https://pipedrive.readme.io/docs/adding-an-organization\" target=\"_blank\" rel=\"noopener noreferrer\">adding an organization</a>.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**add_organization_request** | Option<[**AddOrganizationRequest**](AddOrganizationRequest.md)> |  |  |

### Return type

[**crate::models::AddOrganizationResponse200**](addOrganizationResponse200.md)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## add_organization_follower

> crate::models::AddOrganizationFollowerResponse200 add_organization_follower(id, add_organization_follower_request)
Add a follower to an organization

Adds a follower to an organization.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | The ID of the organization | [required] |
**add_organization_follower_request** | Option<[**AddOrganizationFollowerRequest**](AddOrganizationFollowerRequest.md)> |  |  |

### Return type

[**crate::models::AddOrganizationFollowerResponse200**](addOrganizationFollowerResponse200.md)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_organization

> crate::models::DeleteOrganizationResponse200 delete_organization(id)
Delete an organization

Marks an organization as deleted. After 30 days, the organization will be permanently deleted.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | The ID of the organization | [required] |

### Return type

[**crate::models::DeleteOrganizationResponse200**](deleteOrganizationResponse200.md)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_organization_follower

> crate::models::DeleteOrganizationFollowerResponse200 delete_organization_follower(id, follower_id)
Delete a follower from an organization

Deletes a follower from an organization. You can retrieve the `follower_id` from the <a href=\"https://developers.pipedrive.com/docs/api/v1/Organizations#getOrganizationFollowers\">List followers of an organization</a> endpoint.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | The ID of the organization | [required] |
**follower_id** | **i32** | The ID of the follower | [required] |

### Return type

[**crate::models::DeleteOrganizationFollowerResponse200**](deleteOrganizationFollowerResponse200.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_organizations

> crate::models::DeleteOrganizationsResponse200 delete_organizations(ids)
Delete multiple organizations in bulk

Marks multiple organizations as deleted. After 30 days, the organizations will be permanently deleted.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ids** | **String** | The comma-separated IDs that will be deleted | [required] |

### Return type

[**crate::models::DeleteOrganizationsResponse200**](deleteOrganizationsResponse200.md)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_organization

> crate::models::GetOrganizationResponse200 get_organization(id)
Get details of an organization

Returns the details of an organization. Note that this also returns some additional fields which are not present when asking for all organizations. Also note that custom fields appear as long hashes in the resulting data. These hashes can be mapped against the `key` value of organizationFields.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | The ID of the organization | [required] |

### Return type

[**crate::models::GetOrganizationResponse200**](getOrganizationResponse200.md)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_organization_activities

> crate::models::GetAssociatedActivitiesResponse200 get_organization_activities(id, start, limit, done, exclude)
List activities associated with an organization

Lists activities associated with an organization.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | The ID of the organization | [required] |
**start** | Option<**i32**> | Pagination start |  |[default to 0]
**limit** | Option<**i32**> | Items shown per page |  |
**done** | Option<**f32**> | Whether the activity is done or not. 0 = Not done, 1 = Done. If omitted returns both Done and Not done activities. |  |
**exclude** | Option<**String**> | A comma-separated string of activity IDs to exclude from result |  |

### Return type

[**crate::models::GetAssociatedActivitiesResponse200**](getAssociatedActivitiesResponse200.md)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_organization_deals

> crate::models::GetAssociatedDealsResponse200 get_organization_deals(id, start, limit, status, sort, only_primary_association)
List deals associated with an organization

Lists deals associated with an organization.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | The ID of the organization | [required] |
**start** | Option<**i32**> | Pagination start |  |[default to 0]
**limit** | Option<**i32**> | Items shown per page |  |
**status** | Option<**String**> | Only fetch deals with a specific status. If omitted, all not deleted deals are returned. If set to deleted, deals that have been deleted up to 30 days ago will be included. |  |[default to all_not_deleted]
**sort** | Option<**String**> | The field names and sorting mode separated by a comma (`field_name_1 ASC`, `field_name_2 DESC`). Only first-level field keys are supported (no nested keys). |  |
**only_primary_association** | Option<**f32**> | If set, only deals that are directly associated to the organization are fetched. If not set (default), all deals are fetched that are either directly or indirectly related to the organization. Indirect relations include relations through custom, organization-type fields and through persons of the given organization. |  |

### Return type

[**crate::models::GetAssociatedDealsResponse200**](getAssociatedDealsResponse200.md)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_organization_files

> crate::models::GetAssociatedFilesResponse200 get_organization_files(id, start, limit, sort)
List files attached to an organization

Lists files associated with an organization.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | The ID of the organization | [required] |
**start** | Option<**i32**> | Pagination start |  |[default to 0]
**limit** | Option<**i32**> | Items shown per page |  |
**sort** | Option<**String**> | The field names and sorting mode separated by a comma (`field_name_1 ASC`, `field_name_2 DESC`). Only first-level field keys are supported (no nested keys). Supported fields: `id`, `user_id`, `deal_id`, `person_id`, `org_id`, `product_id`, `add_time`, `update_time`, `file_name`, `file_type`, `file_size`, `comment`. |  |

### Return type

[**crate::models::GetAssociatedFilesResponse200**](getAssociatedFilesResponse200.md)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_organization_followers

> crate::models::GetAssociatedFollowersResponse2001 get_organization_followers(id)
List followers of an organization

Lists the followers of an organization.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | The ID of the organization | [required] |

### Return type

[**crate::models::GetAssociatedFollowersResponse2001**](getAssociatedFollowersResponse200_1.md)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_organization_mail_messages

> crate::models::GetAssociatedMailMessagesResponse200 get_organization_mail_messages(id, start, limit)
List mail messages associated with an organization

Lists mail messages associated with an organization.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | The ID of the organization | [required] |
**start** | Option<**i32**> | Pagination start |  |[default to 0]
**limit** | Option<**i32**> | Items shown per page |  |

### Return type

[**crate::models::GetAssociatedMailMessagesResponse200**](getAssociatedMailMessagesResponse200.md)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_organization_persons

> crate::models::ListPersonsResponse200 get_organization_persons(id, start, limit)
List persons of an organization

Lists persons associated with an organization.<br>If a company uses the [Campaigns product](https://pipedrive.readme.io/docs/campaigns-in-pipedrive-api), then this endpoint will also return the `data.marketing_status` field.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | The ID of the organization | [required] |
**start** | Option<**i32**> | Pagination start |  |[default to 0]
**limit** | Option<**i32**> | Items shown per page |  |

### Return type

[**crate::models::ListPersonsResponse200**](listPersonsResponse200.md)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_organization_updates

> crate::models::GetAssociatedUpdatesResponse200 get_organization_updates(id, start, limit, all_changes, items)
List updates about an organization

Lists updates about an organization.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | The ID of the organization | [required] |
**start** | Option<**i32**> | Pagination start |  |[default to 0]
**limit** | Option<**i32**> | Items shown per page |  |
**all_changes** | Option<**String**> | Whether to show custom field updates or not. 1 = Include custom field changes. If omitted, returns changes without custom field updates. |  |
**items** | Option<**String**> | A comma-separated string for filtering out item specific updates. (Possible values - activity, plannedActivity, note, file, change, deal, follower, participant, mailMessage, mailMessageWithAttachment, invoice, activityFile, document) |  |

### Return type

[**crate::models::GetAssociatedUpdatesResponse200**](getAssociatedUpdatesResponse200.md)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_organization_users

> crate::models::ListPermittedUsersResponse2001 get_organization_users(id)
List permitted users

List users permitted to access an organization.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | The ID of the organization | [required] |

### Return type

[**crate::models::ListPermittedUsersResponse2001**](listPermittedUsersResponse200_1.md)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_organizations

> crate::models::GetOrganizationsResponse200 get_organizations(user_id, filter_id, first_char, start, limit, sort)
Get all organizations

Returns all organizations.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | Option<**i32**> | If supplied, only organizations owned by the given user will be returned. However, `filter_id` takes precedence over `user_id` when both are supplied. |  |
**filter_id** | Option<**i32**> | The ID of the filter to use |  |
**first_char** | Option<**String**> | If supplied, only organizations whose name starts with the specified letter will be returned (case-insensitive) |  |
**start** | Option<**i32**> | Pagination start |  |[default to 0]
**limit** | Option<**i32**> | Items shown per page |  |
**sort** | Option<**String**> | The field names and sorting mode separated by a comma (`field_name_1 ASC`, `field_name_2 DESC`). Only first-level field keys are supported (no nested keys). |  |

### Return type

[**crate::models::GetOrganizationsResponse200**](getOrganizationsResponse200.md)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_organizations_collection

> crate::models::GetOrganizationsCollection200Response get_organizations_collection(cursor, limit, since, until, owner_id, first_char)
Get all organizations (BETA)

Returns all organizations. This is a cursor-paginated endpoint that is currently in BETA. For more information, please refer to our documentation on <a href=\"https://pipedrive.readme.io/docs/core-api-concepts-pagination\" target=\"_blank\" rel=\"noopener noreferrer\">pagination</a>. Please note that only global admins (those with global permissions) can access these endpoints. Users with regular permissions will receive a 403 response. Read more about global permissions <a href=\"https://support.pipedrive.com/en/article/global-user-management\" target=\"_blank\" rel=\"noopener noreferrer\">here</a>.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**cursor** | Option<**String**> | For pagination, the marker (an opaque string value) representing the first item on the next page |  |
**limit** | Option<**i32**> | For pagination, the limit of entries to be returned. If not provided, 100 items will be returned. Please note that a maximum value of 500 is allowed. |  |
**since** | Option<**String**> | The time boundary that points to the start of the range of data. Datetime in ISO 8601 format. E.g. 2022-11-01 08:55:59. Operates on the `update_time` field. |  |
**until** | Option<**String**> | The time boundary that points to the end of the range of data. Datetime in ISO 8601 format. E.g. 2022-11-01 08:55:59. Operates on the `update_time` field. |  |
**owner_id** | Option<**i32**> | If supplied, only organizations owned by the given user will be returned |  |
**first_char** | Option<**String**> | If supplied, only organizations whose name starts with the specified letter will be returned (case-insensitive) |  |

### Return type

[**crate::models::GetOrganizationsCollection200Response**](getOrganizationsCollection_200_response.md)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## merge_organizations

> crate::models::MergeOrganizationsResponse200 merge_organizations(id, merge_organizations_request)
Merge two organizations

Merges an organization with another organization. For more information, see the tutorial for <a href=\"https://pipedrive.readme.io/docs/merging-two-organizations\" target=\"_blank\" rel=\"noopener noreferrer\">merging two organizations</a>.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | The ID of the organization | [required] |
**merge_organizations_request** | Option<[**MergeOrganizationsRequest**](MergeOrganizationsRequest.md)> |  |  |

### Return type

[**crate::models::MergeOrganizationsResponse200**](mergeOrganizationsResponse200.md)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## search_organization

> crate::models::SearchOrganizationResponse200 search_organization(term, fields, exact_match, start, limit)
Search organizations

Searches all organizations by name, address, notes and/or custom fields. This endpoint is a wrapper of <a href=\"https://developers.pipedrive.com/docs/api/v1/ItemSearch#searchItem\">/v1/itemSearch</a> with a narrower OAuth scope.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**term** | **String** | The search term to look for. Minimum 2 characters (or 1 if using `exact_match`). Please note that the search term has to be URL encoded. | [required] |
**fields** | Option<**String**> | A comma-separated string array. The fields to perform the search from. Defaults to all of them. Only the following custom field types are searchable: `address`, `varchar`, `text`, `varchar_auto`, `double`, `monetary` and `phone`. Read more about searching by custom fields <a href=\"https://support.pipedrive.com/en/article/search-finding-what-you-need#searching-by-custom-fields\" target=\"_blank\" rel=\"noopener noreferrer\">here</a>. |  |
**exact_match** | Option<**bool**> | When enabled, only full exact matches against the given term are returned. It is <b>not</b> case sensitive. |  |
**start** | Option<**i32**> | Pagination start. Note that the pagination is based on main results and does not include related items when using `search_for_related_items` parameter. |  |[default to 0]
**limit** | Option<**i32**> | Items shown per page |  |

### Return type

[**crate::models::SearchOrganizationResponse200**](searchOrganizationResponse200.md)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_organization

> crate::models::UpdateOrganizationResponse200 update_organization(id, update_organization_request)
Update an organization

Updates the properties of an organization.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | The ID of the organization | [required] |
**update_organization_request** | Option<[**UpdateOrganizationRequest**](UpdateOrganizationRequest.md)> |  |  |

### Return type

[**crate::models::UpdateOrganizationResponse200**](updateOrganizationResponse200.md)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

