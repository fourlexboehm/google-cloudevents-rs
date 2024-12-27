// This file is @generated by prost-build.
/// The data within all Firebase Remote Config events.
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RemoteConfigEventData {
    /// The version number of the version's corresponding Remote Config template.
    #[prost(int64, tag = "1")]
    pub version_number: i64,
    /// When the Remote Config template was written to the Remote Config server.
    #[prost(message, optional, tag = "2")]
    pub update_time: ::core::option::Option<::pbjson_types::Timestamp>,
    ///   Aggregation of all metadata fields about the account that performed the
    ///   update.
    #[prost(message, optional, tag = "3")]
    pub update_user: ::core::option::Option<RemoteConfigUser>,
    /// The user-provided description of the corresponding Remote Config template.
    #[prost(string, tag = "4")]
    pub description: ::prost::alloc::string::String,
    /// Where the update action originated.
    #[prost(enumeration = "RemoteConfigUpdateOrigin", tag = "5")]
    pub update_origin: i32,
    /// What type of update was made.
    #[prost(enumeration = "RemoteConfigUpdateType", tag = "6")]
    pub update_type: i32,
    /// Only present if this version is the result of a rollback, and will be the
    /// version number of the Remote Config template that was rolled-back to.
    #[prost(int64, tag = "7")]
    pub rollback_source: i64,
}
/// All the fields associated with the person/service account
/// that wrote a Remote Config template.
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RemoteConfigUser {
    /// Display name.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Email address.
    #[prost(string, tag = "2")]
    pub email: ::prost::alloc::string::String,
    /// Image URL.
    #[prost(string, tag = "3")]
    pub image_url: ::prost::alloc::string::String,
}
/// What type of update was associated with the Remote Config template version.
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum RemoteConfigUpdateOrigin {
    /// Catch-all for unrecognized values.
    Unspecified = 0,
    /// The update came from the Firebase UI.
    Console = 1,
    /// The update came from the Remote Config REST API.
    RestApi = 2,
    /// The update came from the Firebase Admin Node SDK.
    AdminSdkNode = 3,
}
impl RemoteConfigUpdateOrigin {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            Self::Unspecified => "REMOTE_CONFIG_UPDATE_ORIGIN_UNSPECIFIED",
            Self::Console => "CONSOLE",
            Self::RestApi => "REST_API",
            Self::AdminSdkNode => "ADMIN_SDK_NODE",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "REMOTE_CONFIG_UPDATE_ORIGIN_UNSPECIFIED" => Some(Self::Unspecified),
            "CONSOLE" => Some(Self::Console),
            "REST_API" => Some(Self::RestApi),
            "ADMIN_SDK_NODE" => Some(Self::AdminSdkNode),
            _ => None,
        }
    }
}
/// Where the Remote Config update action originated.
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum RemoteConfigUpdateType {
    /// Catch-all for unrecognized enum values.
    Unspecified = 0,
    /// A regular incremental update.
    IncrementalUpdate = 1,
    /// A forced update.
    /// The ETag was specified as "*" in an UpdateRemoteConfigRequest
    /// request or the "Force Update" button was pressed on the console.
    ForcedUpdate = 2,
    /// A rollback to a previous Remote Config template.
    Rollback = 3,
}
impl RemoteConfigUpdateType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            Self::Unspecified => "REMOTE_CONFIG_UPDATE_TYPE_UNSPECIFIED",
            Self::IncrementalUpdate => "INCREMENTAL_UPDATE",
            Self::ForcedUpdate => "FORCED_UPDATE",
            Self::Rollback => "ROLLBACK",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "REMOTE_CONFIG_UPDATE_TYPE_UNSPECIFIED" => Some(Self::Unspecified),
            "INCREMENTAL_UPDATE" => Some(Self::IncrementalUpdate),
            "FORCED_UPDATE" => Some(Self::ForcedUpdate),
            "ROLLBACK" => Some(Self::Rollback),
            _ => None,
        }
    }
}
/// The CloudEvent raised when a Remote Config is updated
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RemoteConfigUpdatedEvent {
    /// The data associated with the event.
    #[prost(message, optional, tag = "1")]
    pub data: ::core::option::Option<RemoteConfigEventData>,
}
