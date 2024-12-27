// This file is @generated by prost-build.
/// A BeyondCorp AppConnection resource represents a BeyondCorp protected
/// AppConnection to a remote application. It creates all the necessary GCP
/// components needed for creating a BeyondCorp protected AppConnection. Multiple
/// connectors can be authorised for a single AppConnection.
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AppConnection {
    /// Required. Unique resource name of the AppConnection.
    /// The name is ignored when creating a AppConnection.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Output only. Timestamp when the resource was created.
    #[prost(message, optional, tag = "2")]
    pub create_time: ::core::option::Option<::pbjson_types::Timestamp>,
    /// Output only. Timestamp when the resource was last modified.
    #[prost(message, optional, tag = "3")]
    pub update_time: ::core::option::Option<::pbjson_types::Timestamp>,
    /// Optional. Resource labels to represent user provided metadata.
    #[prost(map = "string, string", tag = "4")]
    pub labels: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        ::prost::alloc::string::String,
    >,
    /// Optional. An arbitrary user-provided name for the AppConnection. Cannot
    /// exceed 64 characters.
    #[prost(string, tag = "5")]
    pub display_name: ::prost::alloc::string::String,
    /// Output only. A unique identifier for the instance generated by the
    /// system.
    #[prost(string, tag = "6")]
    pub uid: ::prost::alloc::string::String,
    /// Required. The type of network connectivity used by the AppConnection.
    #[prost(enumeration = "app_connection::Type", tag = "7")]
    pub r#type: i32,
    /// Required. Address of the remote application endpoint for the BeyondCorp
    /// AppConnection.
    #[prost(message, optional, tag = "8")]
    pub application_endpoint: ::core::option::Option<
        app_connection::ApplicationEndpoint,
    >,
    /// Optional. List of \[google.cloud.beyondcorp.v1main.Connector.name\] that are
    /// authorised to be associated with this AppConnection.
    #[prost(string, repeated, tag = "9")]
    pub connectors: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Output only. The current state of the AppConnection.
    #[prost(enumeration = "app_connection::State", tag = "10")]
    pub state: i32,
    /// Optional. Gateway used by the AppConnection.
    #[prost(message, optional, tag = "11")]
    pub gateway: ::core::option::Option<app_connection::Gateway>,
}
/// Nested message and enum types in `AppConnection`.
pub mod app_connection {
    /// ApplicationEndpoint represents a remote application endpoint.
    #[derive(serde::Serialize, serde::Deserialize)]
    #[serde(rename_all = "snake_case")]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ApplicationEndpoint {
        /// Required. Hostname or IP address of the remote application endpoint.
        #[prost(string, tag = "1")]
        pub host: ::prost::alloc::string::String,
        /// Required. Port of the remote application endpoint.
        #[prost(int32, tag = "2")]
        pub port: i32,
    }
    /// Gateway represents a user facing component that serves as an entrance to
    /// enable connectivity.
    #[derive(serde::Serialize, serde::Deserialize)]
    #[serde(rename_all = "snake_case")]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Gateway {
        /// Required. The type of hosting used by the gateway.
        #[prost(enumeration = "gateway::Type", tag = "2")]
        pub r#type: i32,
        /// Output only. Server-defined URI for this resource.
        #[prost(string, tag = "3")]
        pub uri: ::prost::alloc::string::String,
        /// Output only. Ingress port reserved on the gateways for this
        /// AppConnection, if not specified or zero, the default port is 19443.
        #[prost(int32, tag = "4")]
        pub ingress_port: i32,
        /// Required. AppGateway name in following format:
        /// `projects/{project_id}/locations/{location_id}/appgateways/{gateway_id}`
        #[prost(string, tag = "5")]
        pub app_gateway: ::prost::alloc::string::String,
        /// Output only. L7 private service connection for this resource.
        #[prost(string, tag = "6")]
        pub l7psc: ::prost::alloc::string::String,
    }
    /// Nested message and enum types in `Gateway`.
    pub mod gateway {
        /// Enum listing possible gateway hosting options.
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
        pub enum Type {
            /// Default value. This value is unused.
            Unspecified = 0,
            /// Gateway hosted in a GCP regional managed instance group.
            GcpRegionalMig = 1,
        }
        impl Type {
            /// String value of the enum field names used in the ProtoBuf definition.
            ///
            /// The values are not transformed in any way and thus are considered stable
            /// (if the ProtoBuf definition does not change) and safe for programmatic use.
            pub fn as_str_name(&self) -> &'static str {
                match self {
                    Self::Unspecified => "TYPE_UNSPECIFIED",
                    Self::GcpRegionalMig => "GCP_REGIONAL_MIG",
                }
            }
            /// Creates an enum from field names used in the ProtoBuf definition.
            pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
                match value {
                    "TYPE_UNSPECIFIED" => Some(Self::Unspecified),
                    "GCP_REGIONAL_MIG" => Some(Self::GcpRegionalMig),
                    _ => None,
                }
            }
        }
    }
    /// Enum containing list of all possible network connectivity options
    /// supported by BeyondCorp AppConnection.
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
    pub enum Type {
        /// Default value. This value is unused.
        Unspecified = 0,
        /// TCP Proxy based BeyondCorp AppConnection. API will default to this if
        /// unset.
        TcpProxy = 1,
    }
    impl Type {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Self::Unspecified => "TYPE_UNSPECIFIED",
                Self::TcpProxy => "TCP_PROXY",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "TYPE_UNSPECIFIED" => Some(Self::Unspecified),
                "TCP_PROXY" => Some(Self::TcpProxy),
                _ => None,
            }
        }
    }
    /// Represents the different states of a AppConnection.
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
    pub enum State {
        /// Default value. This value is unused.
        Unspecified = 0,
        /// AppConnection is being created.
        Creating = 1,
        /// AppConnection has been created.
        Created = 2,
        /// AppConnection's configuration is being updated.
        Updating = 3,
        /// AppConnection is being deleted.
        Deleting = 4,
        /// AppConnection is down and may be restored in the future.
        /// This happens when CCFE sends ProjectState = OFF.
        Down = 5,
    }
    impl State {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Self::Unspecified => "STATE_UNSPECIFIED",
                Self::Creating => "CREATING",
                Self::Created => "CREATED",
                Self::Updating => "UPDATING",
                Self::Deleting => "DELETING",
                Self::Down => "DOWN",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "STATE_UNSPECIFIED" => Some(Self::Unspecified),
                "CREATING" => Some(Self::Creating),
                "CREATED" => Some(Self::Created),
                "UPDATING" => Some(Self::Updating),
                "DELETING" => Some(Self::Deleting),
                "DOWN" => Some(Self::Down),
                _ => None,
            }
        }
    }
}
/// The data within all AppConnection events.
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AppConnectionEventData {
    /// Optional. The AppConnection event payload. Unset for deletion events.
    #[prost(message, optional, tag = "1")]
    pub payload: ::core::option::Option<AppConnection>,
}
/// The CloudEvent raised when an AppConnection is created.
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AppConnectionCreatedEvent {
    /// The data associated with the event.
    #[prost(message, optional, tag = "1")]
    pub data: ::core::option::Option<AppConnectionEventData>,
}
/// The CloudEvent raised when an AppConnection is updated.
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AppConnectionUpdatedEvent {
    /// The data associated with the event.
    #[prost(message, optional, tag = "1")]
    pub data: ::core::option::Option<AppConnectionEventData>,
}
/// The CloudEvent raised when an AppConnection is deleted.
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AppConnectionDeletedEvent {
    /// The data associated with the event.
    #[prost(message, optional, tag = "1")]
    pub data: ::core::option::Option<AppConnectionEventData>,
}
