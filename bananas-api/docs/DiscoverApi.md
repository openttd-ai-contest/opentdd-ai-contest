# \DiscoverApi

All URIs are relative to *http://127.0.0.1:8080*

Method | HTTP request | Description
------------- | ------------- | -------------
[**package_content_type_get**](DiscoverApi.md#package_content_type_get) | **GET** /package/{content-type} | Get listing of all the packages with this Content Type.
[**package_content_type_unique_id_get**](DiscoverApi.md#package_content_type_unique_id_get) | **GET** /package/{content-type}/{unique-id} | Get information about a single package.
[**package_content_type_unique_id_upload_date_get**](DiscoverApi.md#package_content_type_unique_id_upload_date_get) | **GET** /package/{content-type}/{unique-id}/{upload-date} | Get information about a single version of a package. The key is \"upload-date\", as that is considered unique.
[**package_self_get**](DiscoverApi.md#package_self_get) | **GET** /package/self | Get listing of the packages you are (one of) the author(s) of.



## package_content_type_get

> Vec<models::Package> package_content_type_get(content_type, since)
Get listing of all the packages with this Content Type.

The listing will contain only versions that are available for new games. the versions available for savegames only can be found per unique-id. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**content_type** | [**ContentType**](.md) | Content Type of package. | [required] |
**since** | Option<**String**> | Package that are modifed since this date. Used to fetch updates of this endpoint after initial call. |  |

### Return type

[**Vec<models::Package>**](Package.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## package_content_type_unique_id_get

> models::Package package_content_type_unique_id_get(content_type, unique_id)
Get information about a single package.

This contains the versions for new games and available for savegames only. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**content_type** | [**ContentType**](.md) | Content Type of package. | [required] |
**unique_id** | **String** | Unique ID of package (lowercase). | [required] |

### Return type

[**models::Package**](Package.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## package_content_type_unique_id_upload_date_get

> models::Version package_content_type_unique_id_upload_date_get(content_type, unique_id, upload_date)
Get information about a single version of a package. The key is \"upload-date\", as that is considered unique.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**content_type** | [**ContentType**](.md) | Content Type of package. | [required] |
**unique_id** | **String** | Unique ID of package (lowercase). | [required] |
**upload_date** | **String** | Upload Date of the version (in ISO-8601). | [required] |

### Return type

[**models::Version**](Version.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## package_self_get

> Vec<models::Package> package_self_get()
Get listing of the packages you are (one of) the author(s) of.

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<models::Package>**](Package.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

