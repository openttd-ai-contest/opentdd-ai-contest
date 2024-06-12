# \UpdateApi

All URIs are relative to *http://127.0.0.1:8080*

Method | HTTP request | Description
------------- | ------------- | -------------
[**package_content_type_unique_id_put**](UpdateApi.md#package_content_type_unique_id_put) | **PUT** /package/{content-type}/{unique-id} | Update the global information of a single package.
[**package_content_type_unique_id_upload_date_put**](UpdateApi.md#package_content_type_unique_id_upload_date_put) | **PUT** /package/{content-type}/{unique-id}/{upload-date} | Update the information of a single version



## package_content_type_unique_id_put

> package_content_type_unique_id_put(content_type, unique_id, package)
Update the global information of a single package.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**content_type** | [**ContentType**](.md) | Content Type of package. | [required] |
**unique_id** | **String** | Unique ID of package (lowercase). | [required] |
**package** | Option<[**Package**](Package.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## package_content_type_unique_id_upload_date_put

> package_content_type_unique_id_upload_date_put(content_type, unique_id, upload_date, version)
Update the information of a single version

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**content_type** | [**ContentType**](.md) | Content Type of package. | [required] |
**unique_id** | **String** | Unique ID of package (lowercase). | [required] |
**upload_date** | **String** | Upload Date of the version (in ISO-8601). | [required] |
**version** | Option<[**Version**](Version.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

