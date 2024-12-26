// This file is @generated by prost-build.
/// Message describing ClientGateway object.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ClientGateway {
    /// Required. name of resource. The name is ignored during creation.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Output only. \[Output only\] Create time stamp.
    #[prost(message, optional, tag = "2")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. \[Output only\] Update time stamp.
    #[prost(message, optional, tag = "3")]
    pub update_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. The operational state of the gateway.
    #[prost(enumeration = "client_gateway::State", tag = "4")]
    pub state: i32,
    /// Output only. A unique identifier for the instance generated by the system.
    #[prost(string, tag = "5")]
    pub id: ::prost::alloc::string::String,
    /// Output only. The client connector service name that the client gateway is
    /// associated to. Client Connector Services, named as follows:
    ///    `projects/{project_id}/locations/{location_id}/client_connector_services/{client_connector_service_id}`.
    #[prost(string, tag = "6")]
    pub client_connector_service: ::prost::alloc::string::String,
}
/// Nested message and enum types in `ClientGateway`.
pub mod client_gateway {
    /// Represents the different states of a gateway.
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
        /// Gateway is being created.
        Creating = 1,
        /// Gateway is being updated.
        Updating = 2,
        /// Gateway is being deleted.
        Deleting = 3,
        /// Gateway is running.
        Running = 4,
        /// Gateway is down and may be restored in the future.
        /// This happens when CCFE sends ProjectState = OFF.
        Down = 5,
        /// ClientGateway encountered an error and is in indeterministic state.
        Error = 6,
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
                Self::Updating => "UPDATING",
                Self::Deleting => "DELETING",
                Self::Running => "RUNNING",
                Self::Down => "DOWN",
                Self::Error => "ERROR",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "STATE_UNSPECIFIED" => Some(Self::Unspecified),
                "CREATING" => Some(Self::Creating),
                "UPDATING" => Some(Self::Updating),
                "DELETING" => Some(Self::Deleting),
                "RUNNING" => Some(Self::Running),
                "DOWN" => Some(Self::Down),
                "ERROR" => Some(Self::Error),
                _ => None,
            }
        }
    }
}
/// The data within all ClientGateway events.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ClientGatewayEventData {
    /// Optional. The ClientGateway event payload. Unset for deletion events.
    #[prost(message, optional, tag = "1")]
    pub payload: ::core::option::Option<ClientGateway>,
}
/// The CloudEvent raised when a ClientGateway is created.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ClientGatewayCreatedEvent {
    /// The data associated with the event.
    #[prost(message, optional, tag = "1")]
    pub data: ::core::option::Option<ClientGatewayEventData>,
}
/// The CloudEvent raised when a ClientGateway is deleted.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ClientGatewayDeletedEvent {
    /// The data associated with the event.
    #[prost(message, optional, tag = "1")]
    pub data: ::core::option::Option<ClientGatewayEventData>,
}
