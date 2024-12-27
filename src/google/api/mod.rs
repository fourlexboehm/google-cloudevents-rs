// This file is @generated by prost-build.
/// A description of a label.
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LabelDescriptor {
    /// The label key.
    #[prost(string, tag = "1")]
    pub key: ::prost::alloc::string::String,
    /// The type of data that can be assigned to the label.
    #[prost(enumeration = "label_descriptor::ValueType", tag = "2")]
    pub value_type: i32,
    /// A human-readable description for the label.
    #[prost(string, tag = "3")]
    pub description: ::prost::alloc::string::String,
}
/// Nested message and enum types in `LabelDescriptor`.
pub mod label_descriptor {
    /// Value types that can be used as label values.
    #[derive(serde::Serialize, serde::Deserialize)]
    #[serde(rename_all = "snake_case")]
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration
    )]
    #[repr(i32)]
    pub enum ValueType {
        /// A variable-length string. This is the default.
        String = 0,
        /// Boolean; true or false.
        Bool = 1,
        /// A 64-bit signed integer.
        Int64 = 2,
    }
    impl ValueType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Self::String => "STRING",
                Self::Bool => "BOOL",
                Self::Int64 => "INT64",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "STRING" => Some(Self::String),
                "BOOL" => Some(Self::Bool),
                "INT64" => Some(Self::Int64),
                _ => None,
            }
        }
    }
}
/// The launch stage as defined by [Google Cloud Platform
/// Launch Stages](<https://cloud.google.com/terms/launch-stages>).
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum LaunchStage {
    /// Do not use this default value.
    Unspecified = 0,
    /// The feature is not yet implemented. Users can not use it.
    Unimplemented = 6,
    /// Prelaunch features are hidden from users and are only visible internally.
    Prelaunch = 7,
    /// Early Access features are limited to a closed group of testers. To use
    /// these features, you must sign up in advance and sign a Trusted Tester
    /// agreement (which includes confidentiality provisions). These features may
    /// be unstable, changed in backward-incompatible ways, and are not
    /// guaranteed to be released.
    EarlyAccess = 1,
    /// Alpha is a limited availability test for releases before they are cleared
    /// for widespread use. By Alpha, all significant design issues are resolved
    /// and we are in the process of verifying functionality. Alpha customers
    /// need to apply for access, agree to applicable terms, and have their
    /// projects allowlisted. Alpha releases don't have to be feature complete,
    /// no SLAs are provided, and there are no technical support obligations, but
    /// they will be far enough along that customers can actually use them in
    /// test environments or for limited-use tests -- just like they would in
    /// normal production cases.
    Alpha = 2,
    /// Beta is the point at which we are ready to open a release for any
    /// customer to use. There are no SLA or technical support obligations in a
    /// Beta release. Products will be complete from a feature perspective, but
    /// may have some open outstanding issues. Beta releases are suitable for
    /// limited production use cases.
    Beta = 3,
    /// GA features are open to all developers and are considered stable and
    /// fully qualified for production use.
    Ga = 4,
    /// Deprecated features are scheduled to be shut down and removed. For more
    /// information, see the "Deprecation Policy" section of our [Terms of
    /// Service](<https://cloud.google.com/terms/>)
    /// and the [Google Cloud Platform Subject to the Deprecation
    /// Policy](<https://cloud.google.com/terms/deprecation>) documentation.
    Deprecated = 5,
}
impl LaunchStage {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            Self::Unspecified => "LAUNCH_STAGE_UNSPECIFIED",
            Self::Unimplemented => "UNIMPLEMENTED",
            Self::Prelaunch => "PRELAUNCH",
            Self::EarlyAccess => "EARLY_ACCESS",
            Self::Alpha => "ALPHA",
            Self::Beta => "BETA",
            Self::Ga => "GA",
            Self::Deprecated => "DEPRECATED",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "LAUNCH_STAGE_UNSPECIFIED" => Some(Self::Unspecified),
            "UNIMPLEMENTED" => Some(Self::Unimplemented),
            "PRELAUNCH" => Some(Self::Prelaunch),
            "EARLY_ACCESS" => Some(Self::EarlyAccess),
            "ALPHA" => Some(Self::Alpha),
            "BETA" => Some(Self::Beta),
            "GA" => Some(Self::Ga),
            "DEPRECATED" => Some(Self::Deprecated),
            _ => None,
        }
    }
}
/// An object that describes the schema of a
/// [MonitoredResource][google.api.MonitoredResource] object using a type name
/// and a set of labels.  For example, the monitored resource descriptor for
/// Google Compute Engine VM instances has a type of
/// `"gce_instance"` and specifies the use of the labels `"instance_id"` and
/// `"zone"` to identify particular VM instances.
///
/// Different APIs can support different monitored resource types. APIs generally
/// provide a `list` method that returns the monitored resource descriptors used
/// by the API.
///
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MonitoredResourceDescriptor {
    /// Optional. The resource name of the monitored resource descriptor:
    /// `"projects/{project_id}/monitoredResourceDescriptors/{type}"` where
    /// {type} is the value of the `type` field in this object and
    /// {project_id} is a project ID that provides API-specific context for
    /// accessing the type.  APIs that do not use project information can use the
    /// resource name format `"monitoredResourceDescriptors/{type}"`.
    #[prost(string, tag = "5")]
    pub name: ::prost::alloc::string::String,
    /// Required. The monitored resource type. For example, the type
    /// `"cloudsql_database"` represents databases in Google Cloud SQL.
    ///   For a list of types, see [Monitored resource
    ///   types](<https://cloud.google.com/monitoring/api/resources>)
    /// and [Logging resource
    /// types](<https://cloud.google.com/logging/docs/api/v2/resource-list>).
    #[prost(string, tag = "1")]
    pub r#type: ::prost::alloc::string::String,
    /// Optional. A concise name for the monitored resource type that might be
    /// displayed in user interfaces. It should be a Title Cased Noun Phrase,
    /// without any article or other determiners. For example,
    /// `"Google Cloud SQL Database"`.
    #[prost(string, tag = "2")]
    pub display_name: ::prost::alloc::string::String,
    /// Optional. A detailed description of the monitored resource type that might
    /// be used in documentation.
    #[prost(string, tag = "3")]
    pub description: ::prost::alloc::string::String,
    /// Required. A set of labels used to describe instances of this monitored
    /// resource type. For example, an individual Google Cloud SQL database is
    /// identified by values for the labels `"database_id"` and `"zone"`.
    #[prost(message, repeated, tag = "4")]
    pub labels: ::prost::alloc::vec::Vec<LabelDescriptor>,
    /// Optional. The launch stage of the monitored resource definition.
    #[prost(enumeration = "LaunchStage", tag = "7")]
    pub launch_stage: i32,
}
/// An object representing a resource that can be used for monitoring, logging,
/// billing, or other purposes. Examples include virtual machine instances,
/// databases, and storage devices such as disks. The `type` field identifies a
/// [MonitoredResourceDescriptor][google.api.MonitoredResourceDescriptor] object
/// that describes the resource's schema. Information in the `labels` field
/// identifies the actual resource and its attributes according to the schema.
/// For example, a particular Compute Engine VM instance could be represented by
/// the following object, because the
/// [MonitoredResourceDescriptor][google.api.MonitoredResourceDescriptor] for
/// `"gce_instance"` has labels
/// `"project_id"`, `"instance_id"` and `"zone"`:
///
///      { "type": "gce_instance",
///        "labels": { "project_id": "my-project",
///                    "instance_id": "12345678901234",
///                    "zone": "us-central1-a" }}
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MonitoredResource {
    /// Required. The monitored resource type. This field must match
    /// the `type` field of a
    /// [MonitoredResourceDescriptor][google.api.MonitoredResourceDescriptor]
    /// object. For example, the type of a Compute Engine VM instance is
    /// `gce_instance`. Some descriptors include the service name in the type; for
    /// example, the type of a Datastream stream is
    /// `datastream.googleapis.com/Stream`.
    #[prost(string, tag = "1")]
    pub r#type: ::prost::alloc::string::String,
    /// Required. Values for all of the labels listed in the associated monitored
    /// resource descriptor. For example, Compute Engine VM instances use the
    /// labels `"project_id"`, `"instance_id"`, and `"zone"`.
    #[prost(map = "string, string", tag = "2")]
    pub labels: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        ::prost::alloc::string::String,
    >,
}
/// Auxiliary metadata for a [MonitoredResource][google.api.MonitoredResource]
/// object. [MonitoredResource][google.api.MonitoredResource] objects contain the
/// minimum set of information to uniquely identify a monitored resource
/// instance. There is some other useful auxiliary metadata. Monitoring and
/// Logging use an ingestion pipeline to extract metadata for cloud resources of
/// all types, and store the metadata in this message.
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MonitoredResourceMetadata {
    /// Output only. Values for predefined system metadata labels.
    /// System labels are a kind of metadata extracted by Google, including
    /// "machine_image", "vpc", "subnet_id",
    /// "security_group", "name", etc.
    /// System label values can be only strings, Boolean values, or a list of
    /// strings. For example:
    ///
    ///      { "name": "my-test-instance",
    ///        "security_group": \["a", "b", "c"\],
    ///        "spot_instance": false }
    #[prost(message, optional, tag = "1")]
    pub system_labels: ::core::option::Option<::pbjson_types::Struct>,
    /// Output only. A map of user-defined metadata labels.
    #[prost(map = "string, string", tag = "2")]
    pub user_labels: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        ::prost::alloc::string::String,
    >,
}
