# Package

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | Option<**String**> |  | [optional]
**description** | Option<**String**> |  | [optional]
**url** | Option<**String**> |  | [optional]
**tags** | Option<**Vec<String>**> |  | [optional]
**content_type** | [**models::ContentType**](ContentType.md) |  | 
**unique_id** | **String** |  | [readonly]
**archived** | Option<**bool**> |  | [optional][readonly][default to false]
**replaced_by** | Option<[**models::PackageAllOfReplacedBy**](Package_allOf_replaced_by.md)> |  | [optional]
**authors** | Option<[**Vec<models::Author>**](Author.md)> |  | [optional][readonly]
**versions** | Option<[**Vec<models::VersionMinimized>**](VersionMinimized.md)> |  | [optional][readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


