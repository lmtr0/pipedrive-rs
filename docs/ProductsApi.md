# \ProductsApi

All URIs are relative to *https://api.pipedrive.com/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**add_product**](ProductsApi.md#add_product) | **POST** /products | Add a product
[**add_product_follower**](ProductsApi.md#add_product_follower) | **POST** /products/{id}/followers | Add a follower to a product
[**delete_product**](ProductsApi.md#delete_product) | **DELETE** /products/{id} | Delete a product
[**delete_product_follower**](ProductsApi.md#delete_product_follower) | **DELETE** /products/{id}/followers/{follower_id} | Delete a follower from a product
[**get_product**](ProductsApi.md#get_product) | **GET** /products/{id} | Get one product
[**get_product_deals**](ProductsApi.md#get_product_deals) | **GET** /products/{id}/deals | Get deals where a product is attached to
[**get_product_files**](ProductsApi.md#get_product_files) | **GET** /products/{id}/files | List files attached to a product
[**get_product_followers**](ProductsApi.md#get_product_followers) | **GET** /products/{id}/followers | List followers of a product
[**get_product_users**](ProductsApi.md#get_product_users) | **GET** /products/{id}/permittedUsers | List permitted users
[**get_products**](ProductsApi.md#get_products) | **GET** /products | Get all products
[**search_products**](ProductsApi.md#search_products) | **GET** /products/search | Search products
[**update_product**](ProductsApi.md#update_product) | **PUT** /products/{id} | Update a product



## add_product

> crate::models::GetproductResponse200 add_product(add_product_request)
Add a product

Adds a new product to the Products inventory. For more information, see the tutorial for <a href=\"https://pipedrive.readme.io/docs/adding-a-product\" target=\"_blank\" rel=\"noopener noreferrer\">adding a product</a>.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**add_product_request** | Option<[**AddProductRequest**](AddProductRequest.md)> |  |  |

### Return type

[**crate::models::GetproductResponse200**](getproductResponse200.md)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## add_product_follower

> crate::models::NewFollowerResponse200 add_product_follower(id, add_product_follower_request)
Add a follower to a product

Adds a follower to a product.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | The ID of the product | [required] |
**add_product_follower_request** | Option<[**AddProductFollowerRequest**](AddProductFollowerRequest.md)> |  |  |

### Return type

[**crate::models::NewFollowerResponse200**](newFollowerResponse200.md)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_product

> crate::models::DeleteProductResponse200 delete_product(id)
Delete a product

Marks a product as deleted. After 30 days, the product will be permanently deleted.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | The ID of the product | [required] |

### Return type

[**crate::models::DeleteProductResponse200**](deleteProductResponse200.md)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_product_follower

> crate::models::DeleteProductFollowerResponse200 delete_product_follower(id, follower_id)
Delete a follower from a product

Deletes a follower from a product.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | The ID of the product | [required] |
**follower_id** | **i32** | The ID of the relationship between the follower and the product | [required] |

### Return type

[**crate::models::DeleteProductFollowerResponse200**](deleteProductFollowerResponse200.md)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_product

> crate::models::GetproductResponse200 get_product(id)
Get one product

Returns data about a specific product.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | The ID of the product | [required] |

### Return type

[**crate::models::GetproductResponse200**](getproductResponse200.md)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_product_deals

> crate::models::GetAssociatedDealsResponse200 get_product_deals(id, start, limit, status)
Get deals where a product is attached to

Returns data about deals that have a product attached to it.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | The ID of the product | [required] |
**start** | Option<**i32**> | Pagination start |  |[default to 0]
**limit** | Option<**i32**> | Items shown per page |  |
**status** | Option<**String**> | Only fetch deals with a specific status. If omitted, all not deleted deals are returned. If set to deleted, deals that have been deleted up to 30 days ago will be included. |  |[default to all_not_deleted]

### Return type

[**crate::models::GetAssociatedDealsResponse200**](getAssociatedDealsResponse200.md)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_product_files

> crate::models::GetAssociatedFilesResponse2001 get_product_files(id, start, limit, sort)
List files attached to a product

Lists files associated with a product.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | The ID of the product | [required] |
**start** | Option<**i32**> | Pagination start |  |[default to 0]
**limit** | Option<**i32**> | Items shown per page |  |
**sort** | Option<**String**> | The field name and sorting mode (`field_name_1 ASC` or `field_name_1 DESC`). Supported fields: `update_time`, `id` |  |

### Return type

[**crate::models::GetAssociatedFilesResponse2001**](getAssociatedFilesResponse200_1.md)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_product_followers

> crate::models::GetProductFollowersResponseSuccess get_product_followers(id, start, limit)
List followers of a product

Lists the followers of a product.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | The ID of the product | [required] |
**start** | Option<**i32**> | Pagination start |  |[default to 0]
**limit** | Option<**i32**> | Items shown per page |  |

### Return type

[**crate::models::GetProductFollowersResponseSuccess**](getProductFollowersResponseSuccess.md)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_product_users

> crate::models::UserIds get_product_users(id)
List permitted users

Lists users permitted to access a product.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | The ID of the product | [required] |

### Return type

[**crate::models::UserIds**](userIds.md)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_products

> crate::models::GetProductsResponse200 get_products(user_id, filter_id, ids, first_char, get_summary, start, limit)
Get all products

Returns data about all products.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | Option<**i32**> | If supplied, only products owned by the given user will be returned |  |
**filter_id** | Option<**i32**> | The ID of the filter to use |  |
**ids** | Option<[**Vec<i32>**](i32.md)> | An array of integers with the IDs of the products that should be returned in the response |  |
**first_char** | Option<**String**> | If supplied, only products whose name starts with the specified letter will be returned (case-insensitive) |  |
**get_summary** | Option<**bool**> | If supplied, the response will return the total numbers of products in the `additional_data.summary.total_count` property |  |
**start** | Option<**i32**> | Pagination start |  |[default to 0]
**limit** | Option<**i32**> | Items shown per page |  |

### Return type

[**crate::models::GetProductsResponse200**](getProductsResponse200.md)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## search_products

> crate::models::SearchProductsResponse200 search_products(term, fields, exact_match, include_fields, start, limit)
Search products

Searches all products by name, code and/or custom fields. This endpoint is a wrapper of <a href=\"https://developers.pipedrive.com/docs/api/v1/ItemSearch#searchItem\">/v1/itemSearch</a> with a narrower OAuth scope.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**term** | **String** | The search term to look for. Minimum 2 characters (or 1 if using `exact_match`). Please note that the search term has to be URL encoded. | [required] |
**fields** | Option<**String**> | A comma-separated string array. The fields to perform the search from. Defaults to all of them. Only the following custom field types are searchable: `address`, `varchar`, `text`, `varchar_auto`, `double`, `monetary` and `phone`. Read more about searching by custom fields <a href=\"https://support.pipedrive.com/en/article/search-finding-what-you-need#searching-by-custom-fields\" target=\"_blank\" rel=\"noopener noreferrer\">here</a>. |  |
**exact_match** | Option<**bool**> | When enabled, only full exact matches against the given term are returned. It is <b>not</b> case sensitive. |  |
**include_fields** | Option<**String**> | Supports including optional fields in the results which are not provided by default |  |
**start** | Option<**i32**> | Pagination start. Note that the pagination is based on main results and does not include related items when using `search_for_related_items` parameter. |  |[default to 0]
**limit** | Option<**i32**> | Items shown per page |  |

### Return type

[**crate::models::SearchProductsResponse200**](searchProductsResponse200.md)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_product

> crate::models::UpdateProductResponse200 update_product(id, add_product_request1)
Update a product

Updates product data.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | The ID of the product | [required] |
**add_product_request1** | Option<[**AddProductRequest1**](AddProductRequest1.md)> |  |  |

### Return type

[**crate::models::UpdateProductResponse200**](updateProductResponse200.md)

### Authorization

[api_key](../README.md#api_key), [oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

