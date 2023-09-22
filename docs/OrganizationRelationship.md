# OrganizationRelationship

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**org_id** | Option<**i32**> | The ID of the base organization for the returned calculated values | [optional]
**r#type** | Option<**String**> | The type of organization relationship | [optional]
**rel_owner_org_id** | Option<**i32**> | The owner of this relationship. If type is `parent`, then the owner is the parent and the linked organization is the daughter. | [optional]
**rel_linked_org_id** | Option<**i32**> | The linked organization in this relationship. If type is `parent`, then the linked organization is the daughter. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


