// This file is @generated by prost-build.
/// The device resource.
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Device {
    /// The user-defined device identifier. The device ID must be unique
    /// within a device registry.
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    /// The resource path name. For example,
    /// `projects/p1/locations/us-central1/registries/registry0/devices/dev0` or
    /// `projects/p1/locations/us-central1/registries/registry0/devices/{num_id}`.
    /// When `name` is populated as a response from the service, it always ends
    /// in the device numeric ID.
    #[prost(string, tag = "2")]
    pub name: ::prost::alloc::string::String,
    /// \[Output only\] A server-defined unique numeric ID for the device. This is a
    /// more compact way to identify devices, and it is globally unique.
    #[prost(uint64, tag = "3")]
    pub num_id: u64,
    /// The credentials used to authenticate this device. To allow credential
    /// rotation without interruption, multiple device credentials can be bound to
    /// this device. No more than 3 credentials can be bound to a single device at
    /// a time. When new credentials are added to a device, they are verified
    /// against the registry credentials. For details, see the description of the
    /// `DeviceRegistry.credentials` field.
    #[prost(message, repeated, tag = "12")]
    pub credentials: ::prost::alloc::vec::Vec<DeviceCredential>,
    /// \[Output only\] The last time an MQTT `PINGREQ` was received. This field
    /// applies only to devices connecting through MQTT. MQTT clients usually only
    /// send `PINGREQ` messages if the connection is idle, and no other messages
    /// have been sent. Timestamps are periodically collected and written to
    /// storage; they may be stale by a few minutes.
    #[prost(message, optional, tag = "7")]
    pub last_heartbeat_time: ::core::option::Option<::pbjson_types::Timestamp>,
    /// \[Output only\] The last time a telemetry event was received. Timestamps are
    /// periodically collected and written to storage; they may be stale by a few
    /// minutes.
    #[prost(message, optional, tag = "8")]
    pub last_event_time: ::core::option::Option<::pbjson_types::Timestamp>,
    /// \[Output only\] The last time a state event was received. Timestamps are
    /// periodically collected and written to storage; they may be stale by a few
    /// minutes.
    #[prost(message, optional, tag = "20")]
    pub last_state_time: ::core::option::Option<::pbjson_types::Timestamp>,
    /// \[Output only\] The last time a cloud-to-device config version acknowledgment
    /// was received from the device. This field is only for configurations
    /// sent through MQTT.
    #[prost(message, optional, tag = "14")]
    pub last_config_ack_time: ::core::option::Option<::pbjson_types::Timestamp>,
    /// \[Output only\] The last time a cloud-to-device config version was sent to
    /// the device.
    #[prost(message, optional, tag = "18")]
    pub last_config_send_time: ::core::option::Option<::pbjson_types::Timestamp>,
    /// If a device is blocked, connections or requests from this device will fail.
    /// Can be used to temporarily prevent the device from connecting if, for
    /// example, the sensor is generating bad data and needs maintenance.
    #[prost(bool, tag = "19")]
    pub blocked: bool,
    /// \[Output only\] The time the most recent error occurred, such as a failure to
    /// publish to Cloud Pub/Sub. This field is the timestamp of
    /// 'last_error_status'.
    #[prost(message, optional, tag = "10")]
    pub last_error_time: ::core::option::Option<::pbjson_types::Timestamp>,
    /// \[Output only\] The error message of the most recent error, such as a failure
    /// to publish to Cloud Pub/Sub. 'last_error_time' is the timestamp of this
    /// field. If no errors have occurred, this field has an empty message
    /// and the status code 0 == OK. Otherwise, this field is expected to have a
    /// status code other than OK.
    #[prost(message, optional, tag = "11")]
    pub last_error_status: ::core::option::Option<
        super::super::super::super::rpc::Status,
    >,
    /// The most recent device configuration, which is eventually sent from
    /// Cloud IoT Core to the device. If not present on creation, the
    /// configuration will be initialized with an empty payload and version value
    /// of `1`. To update this field after creation, use the
    /// `DeviceManager.ModifyCloudToDeviceConfig` method.
    #[prost(message, optional, tag = "13")]
    pub config: ::core::option::Option<DeviceConfig>,
    /// \[Output only\] The state most recently received from the device. If no state
    /// has been reported, this field is not present.
    #[prost(message, optional, tag = "16")]
    pub state: ::core::option::Option<DeviceState>,
    /// **Beta Feature**
    ///
    /// The logging verbosity for device activity. If unspecified,
    /// DeviceRegistry.log_level will be used.
    #[prost(enumeration = "LogLevel", tag = "21")]
    pub log_level: i32,
    /// The metadata key-value pairs assigned to the device. This metadata is not
    /// interpreted or indexed by Cloud IoT Core. It can be used to add contextual
    /// information for the device.
    ///
    /// Keys must conform to the regular expression [a-zA-Z][a-zA-Z0-9-_.+~%]+ and
    /// be less than 128 bytes in length.
    ///
    /// Values are free-form strings. Each value must be less than or equal to 32
    /// KB in size.
    ///
    /// The total size of all keys and values must be less than 256 KB, and the
    /// maximum number of key-value pairs is 500.
    #[prost(map = "string, string", tag = "17")]
    pub metadata: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        ::prost::alloc::string::String,
    >,
    /// Gateway-related configuration and state.
    #[prost(message, optional, tag = "24")]
    pub gateway_config: ::core::option::Option<GatewayConfig>,
}
/// Gateway-related configuration and state.
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GatewayConfig {
    /// Indicates whether the device is a gateway.
    #[prost(enumeration = "GatewayType", tag = "1")]
    pub gateway_type: i32,
    /// Indicates how to authorize and/or authenticate devices to access the
    /// gateway.
    #[prost(enumeration = "GatewayAuthMethod", tag = "2")]
    pub gateway_auth_method: i32,
    /// \[Output only\] The ID of the gateway the device accessed most recently.
    #[prost(string, tag = "3")]
    pub last_accessed_gateway_id: ::prost::alloc::string::String,
    /// \[Output only\] The most recent time at which the device accessed the gateway
    /// specified in `last_accessed_gateway`.
    #[prost(message, optional, tag = "4")]
    pub last_accessed_gateway_time: ::core::option::Option<::pbjson_types::Timestamp>,
}
/// A container for a group of devices.
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeviceRegistry {
    /// The identifier of this device registry. For example, `myRegistry`.
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    /// The resource path name. For example,
    /// `projects/example-project/locations/us-central1/registries/my-registry`.
    #[prost(string, tag = "2")]
    pub name: ::prost::alloc::string::String,
    /// The configuration for notification of telemetry events received from the
    /// device. All telemetry events that were successfully published by the
    /// device and acknowledged by Cloud IoT Core are guaranteed to be
    /// delivered to Cloud Pub/Sub. If multiple configurations match a message,
    /// only the first matching configuration is used. If you try to publish a
    /// device telemetry event using MQTT without specifying a Cloud Pub/Sub topic
    /// for the device's registry, the connection closes automatically. If you try
    /// to do so using an HTTP connection, an error is returned. Up to 10
    /// configurations may be provided.
    #[prost(message, repeated, tag = "10")]
    pub event_notification_configs: ::prost::alloc::vec::Vec<EventNotificationConfig>,
    /// The configuration for notification of new states received from the device.
    /// State updates are guaranteed to be stored in the state history, but
    /// notifications to Cloud Pub/Sub are not guaranteed. For example, if
    /// permissions are misconfigured or the specified topic doesn't exist, no
    /// notification will be published but the state will still be stored in Cloud
    /// IoT Core.
    #[prost(message, optional, tag = "7")]
    pub state_notification_config: ::core::option::Option<StateNotificationConfig>,
    /// The MQTT configuration for this device registry.
    #[prost(message, optional, tag = "4")]
    pub mqtt_config: ::core::option::Option<MqttConfig>,
    /// The DeviceService (HTTP) configuration for this device registry.
    #[prost(message, optional, tag = "9")]
    pub http_config: ::core::option::Option<HttpConfig>,
    /// **Beta Feature**
    ///
    /// The default logging verbosity for activity from devices in this registry.
    /// The verbosity level can be overridden by Device.log_level.
    #[prost(enumeration = "LogLevel", tag = "11")]
    pub log_level: i32,
    /// The credentials used to verify the device credentials. No more than 10
    /// credentials can be bound to a single registry at a time. The verification
    /// process occurs at the time of device creation or update. If this field is
    /// empty, no verification is performed. Otherwise, the credentials of a newly
    /// created device or added credentials of an updated device should be signed
    /// with one of these registry credentials.
    ///
    /// Note, however, that existing devices will never be affected by
    /// modifications to this list of credentials: after a device has been
    /// successfully created in a registry, it should be able to connect even if
    /// its registry credentials are revoked, deleted, or modified.
    #[prost(message, repeated, tag = "8")]
    pub credentials: ::prost::alloc::vec::Vec<RegistryCredential>,
}
/// The configuration of MQTT for a device registry.
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct MqttConfig {
    /// If enabled, allows connections using the MQTT protocol. Otherwise, MQTT
    /// connections to this registry will fail.
    #[prost(enumeration = "MqttState", tag = "1")]
    pub mqtt_enabled_state: i32,
}
/// The configuration of the HTTP bridge for a device registry.
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct HttpConfig {
    /// If enabled, allows devices to use DeviceService via the HTTP protocol.
    /// Otherwise, any requests to DeviceService will fail for this registry.
    #[prost(enumeration = "HttpState", tag = "1")]
    pub http_enabled_state: i32,
}
/// The configuration for forwarding telemetry events.
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EventNotificationConfig {
    /// If the subfolder name matches this string exactly, this configuration will
    /// be used. The string must not include the leading '/' character. If empty,
    /// all strings are matched. This field is used only for telemetry events;
    /// subfolders are not supported for state changes.
    #[prost(string, tag = "2")]
    pub subfolder_matches: ::prost::alloc::string::String,
    /// A Cloud Pub/Sub topic name. For example,
    /// `projects/myProject/topics/deviceEvents`.
    #[prost(string, tag = "1")]
    pub pubsub_topic_name: ::prost::alloc::string::String,
}
/// The configuration for notification of new states received from the device.
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StateNotificationConfig {
    /// A Cloud Pub/Sub topic name. For example,
    /// `projects/myProject/topics/deviceEvents`.
    #[prost(string, tag = "1")]
    pub pubsub_topic_name: ::prost::alloc::string::String,
}
/// A server-stored registry credential used to validate device credentials.
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RegistryCredential {
    /// The credential data. Reserved for expansion in the future.
    #[prost(oneof = "registry_credential::Credential", tags = "1")]
    pub credential: ::core::option::Option<registry_credential::Credential>,
}
/// Nested message and enum types in `RegistryCredential`.
pub mod registry_credential {
    /// The credential data. Reserved for expansion in the future.
    #[derive(serde::Serialize, serde::Deserialize)]
    #[serde(rename_all = "snake_case")]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Credential {
        /// A public key certificate used to verify the device credentials.
        #[prost(message, tag = "1")]
        PublicKeyCertificate(super::PublicKeyCertificate),
    }
}
/// Details of an X.509 certificate. For informational purposes only.
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct X509CertificateDetails {
    /// The entity that signed the certificate.
    #[prost(string, tag = "1")]
    pub issuer: ::prost::alloc::string::String,
    /// The entity the certificate and public key belong to.
    #[prost(string, tag = "2")]
    pub subject: ::prost::alloc::string::String,
    /// The time the certificate becomes valid.
    #[prost(message, optional, tag = "3")]
    pub start_time: ::core::option::Option<::pbjson_types::Timestamp>,
    /// The time the certificate becomes invalid.
    #[prost(message, optional, tag = "4")]
    pub expiry_time: ::core::option::Option<::pbjson_types::Timestamp>,
    /// The algorithm used to sign the certificate.
    #[prost(string, tag = "5")]
    pub signature_algorithm: ::prost::alloc::string::String,
    /// The type of public key in the certificate.
    #[prost(string, tag = "6")]
    pub public_key_type: ::prost::alloc::string::String,
}
/// A public key certificate format and data.
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PublicKeyCertificate {
    /// The certificate format.
    #[prost(enumeration = "PublicKeyCertificateFormat", tag = "1")]
    pub format: i32,
    /// The certificate data.
    #[prost(string, tag = "2")]
    pub certificate: ::prost::alloc::string::String,
    /// \[Output only\] The certificate details. Used only for X.509 certificates.
    #[prost(message, optional, tag = "3")]
    pub x509_details: ::core::option::Option<X509CertificateDetails>,
}
/// A server-stored device credential used for authentication.
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeviceCredential {
    /// \[Optional\] The time at which this credential becomes invalid. This
    /// credential will be ignored for new client authentication requests after
    /// this timestamp; however, it will not be automatically deleted.
    #[prost(message, optional, tag = "6")]
    pub expiration_time: ::core::option::Option<::pbjson_types::Timestamp>,
    /// The credential data. Reserved for expansion in the future.
    #[prost(oneof = "device_credential::Credential", tags = "2")]
    pub credential: ::core::option::Option<device_credential::Credential>,
}
/// Nested message and enum types in `DeviceCredential`.
pub mod device_credential {
    /// The credential data. Reserved for expansion in the future.
    #[derive(serde::Serialize, serde::Deserialize)]
    #[serde(rename_all = "snake_case")]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Credential {
        /// A public key used to verify the signature of JSON Web Tokens (JWTs).
        /// When adding a new device credential, either via device creation or via
        /// modifications, this public key credential may be required to be signed by
        /// one of the registry level certificates. More specifically, if the
        /// registry contains at least one certificate, any new device credential
        /// must be signed by one of the registry certificates. As a result,
        /// when the registry contains certificates, only X.509 certificates are
        /// accepted as device credentials. However, if the registry does
        /// not contain a certificate, self-signed certificates and public keys will
        /// be accepted. New device credentials must be different from every
        /// registry-level certificate.
        #[prost(message, tag = "2")]
        PublicKey(super::PublicKeyCredential),
    }
}
/// A public key format and data.
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PublicKeyCredential {
    /// The format of the key.
    #[prost(enumeration = "PublicKeyFormat", tag = "1")]
    pub format: i32,
    /// The key data.
    #[prost(string, tag = "2")]
    pub key: ::prost::alloc::string::String,
}
/// The device configuration. Eventually delivered to devices.
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeviceConfig {
    /// \[Output only\] The version of this update. The version number is assigned by
    /// the server, and is always greater than 0 after device creation. The
    /// version must be 0 on the `CreateDevice` request if a `config` is
    /// specified; the response of `CreateDevice` will always have a value of 1.
    #[prost(int64, tag = "1")]
    pub version: i64,
    /// \[Output only\] The time at which this configuration version was updated in
    /// Cloud IoT Core. This timestamp is set by the server.
    #[prost(message, optional, tag = "2")]
    pub cloud_update_time: ::core::option::Option<::pbjson_types::Timestamp>,
    /// \[Output only\] The time at which Cloud IoT Core received the
    /// acknowledgment from the device, indicating that the device has received
    /// this configuration version. If this field is not present, the device has
    /// not yet acknowledged that it received this version. Note that when
    /// the config was sent to the device, many config versions may have been
    /// available in Cloud IoT Core while the device was disconnected, and on
    /// connection, only the latest version is sent to the device. Some
    /// versions may never be sent to the device, and therefore are never
    /// acknowledged. This timestamp is set by Cloud IoT Core.
    #[prost(message, optional, tag = "3")]
    pub device_ack_time: ::core::option::Option<::pbjson_types::Timestamp>,
    /// The device configuration data.
    #[serde(with = "serde_bytes")]
#[prost(bytes = "vec", tag =
 "4")]
    pub binary_data: ::prost::alloc::vec::Vec<u8>,
}
/// The device state, as reported by the device.
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeviceState {
    /// \[Output only\] The time at which this state version was updated in Cloud
    /// IoT Core.
    #[prost(message, optional, tag = "1")]
    pub update_time: ::core::option::Option<::pbjson_types::Timestamp>,
    /// The device state data.
    #[serde(with = "serde_bytes")]
#[prost(bytes = "vec", tag =
 "2")]
    pub binary_data: ::prost::alloc::vec::Vec<u8>,
}
/// The data within all Cloud IoT Device events.
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeviceEventData {
    /// Optional. The Device event payload. Unset for deletion events.
    #[prost(message, optional, tag = "1")]
    pub payload: ::core::option::Option<Device>,
}
/// The data within all Cloud IoT Registry events.
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RegistryEventData {
    /// Optional. The Registry event payload. Unset for deletion events.
    #[prost(message, optional, tag = "1")]
    pub payload: ::core::option::Option<DeviceRegistry>,
}
/// Indicates whether an MQTT connection is enabled or disabled. See the field
/// description for details.
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum MqttState {
    /// No MQTT state specified. If not specified, MQTT will be enabled by default.
    Unspecified = 0,
    /// Enables a MQTT connection.
    MqttEnabled = 1,
    /// Disables a MQTT connection.
    MqttDisabled = 2,
}
impl MqttState {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            Self::Unspecified => "MQTT_STATE_UNSPECIFIED",
            Self::MqttEnabled => "MQTT_ENABLED",
            Self::MqttDisabled => "MQTT_DISABLED",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "MQTT_STATE_UNSPECIFIED" => Some(Self::Unspecified),
            "MQTT_ENABLED" => Some(Self::MqttEnabled),
            "MQTT_DISABLED" => Some(Self::MqttDisabled),
            _ => None,
        }
    }
}
/// Indicates whether DeviceService (HTTP) is enabled or disabled for the
/// registry. See the field description for details.
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum HttpState {
    /// No HTTP state specified. If not specified, DeviceService will be
    /// enabled by default.
    Unspecified = 0,
    /// Enables DeviceService (HTTP) service for the registry.
    HttpEnabled = 1,
    /// Disables DeviceService (HTTP) service for the registry.
    HttpDisabled = 2,
}
impl HttpState {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            Self::Unspecified => "HTTP_STATE_UNSPECIFIED",
            Self::HttpEnabled => "HTTP_ENABLED",
            Self::HttpDisabled => "HTTP_DISABLED",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "HTTP_STATE_UNSPECIFIED" => Some(Self::Unspecified),
            "HTTP_ENABLED" => Some(Self::HttpEnabled),
            "HTTP_DISABLED" => Some(Self::HttpDisabled),
            _ => None,
        }
    }
}
/// **Beta Feature**
///
/// The logging verbosity for device activity. Specifies which events should be
/// written to logs. For example, if the LogLevel is ERROR, only events that
/// terminate in errors will be logged. LogLevel is inclusive; enabling INFO
/// logging will also enable ERROR logging.
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum LogLevel {
    /// No logging specified. If not specified, logging will be disabled.
    Unspecified = 0,
    /// Disables logging.
    None = 10,
    /// Error events will be logged.
    Error = 20,
    /// Informational events will be logged, such as connections and
    /// disconnections.
    Info = 30,
    /// All events will be logged.
    Debug = 40,
}
impl LogLevel {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            Self::Unspecified => "LOG_LEVEL_UNSPECIFIED",
            Self::None => "NONE",
            Self::Error => "ERROR",
            Self::Info => "INFO",
            Self::Debug => "DEBUG",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "LOG_LEVEL_UNSPECIFIED" => Some(Self::Unspecified),
            "NONE" => Some(Self::None),
            "ERROR" => Some(Self::Error),
            "INFO" => Some(Self::Info),
            "DEBUG" => Some(Self::Debug),
            _ => None,
        }
    }
}
/// Gateway type.
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum GatewayType {
    /// If unspecified, the device is considered a non-gateway device.
    Unspecified = 0,
    /// The device is a gateway.
    Gateway = 1,
    /// The device is not a gateway.
    NonGateway = 2,
}
impl GatewayType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            Self::Unspecified => "GATEWAY_TYPE_UNSPECIFIED",
            Self::Gateway => "GATEWAY",
            Self::NonGateway => "NON_GATEWAY",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "GATEWAY_TYPE_UNSPECIFIED" => Some(Self::Unspecified),
            "GATEWAY" => Some(Self::Gateway),
            "NON_GATEWAY" => Some(Self::NonGateway),
            _ => None,
        }
    }
}
/// The gateway authorization/authentication method. This setting determines how
/// Cloud IoT Core authorizes/authenticate devices to access the gateway.
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum GatewayAuthMethod {
    /// No authentication/authorization method specified. No devices are allowed to
    /// access the gateway.
    Unspecified = 0,
    /// The device is authenticated through the gateway association only. Device
    /// credentials are ignored even if provided.
    AssociationOnly = 1,
    /// The device is authenticated through its own credentials. Gateway
    /// association is not checked.
    DeviceAuthTokenOnly = 2,
    /// The device is authenticated through both device credentials and gateway
    /// association. The device must be bound to the gateway and must provide its
    /// own credentials.
    AssociationAndDeviceAuthToken = 3,
}
impl GatewayAuthMethod {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            Self::Unspecified => "GATEWAY_AUTH_METHOD_UNSPECIFIED",
            Self::AssociationOnly => "ASSOCIATION_ONLY",
            Self::DeviceAuthTokenOnly => "DEVICE_AUTH_TOKEN_ONLY",
            Self::AssociationAndDeviceAuthToken => "ASSOCIATION_AND_DEVICE_AUTH_TOKEN",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "GATEWAY_AUTH_METHOD_UNSPECIFIED" => Some(Self::Unspecified),
            "ASSOCIATION_ONLY" => Some(Self::AssociationOnly),
            "DEVICE_AUTH_TOKEN_ONLY" => Some(Self::DeviceAuthTokenOnly),
            "ASSOCIATION_AND_DEVICE_AUTH_TOKEN" => {
                Some(Self::AssociationAndDeviceAuthToken)
            }
            _ => None,
        }
    }
}
/// The supported formats for the public key.
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum PublicKeyCertificateFormat {
    /// The format has not been specified. This is an invalid default value and
    /// must not be used.
    UnspecifiedPublicKeyCertificateFormat = 0,
    /// An X.509v3 certificate ([RFC5280](<https://www.ietf.org/rfc/rfc5280.txt>)),
    /// encoded in base64, and wrapped by `-----BEGIN CERTIFICATE-----` and
    /// `-----END CERTIFICATE-----`.
    X509CertificatePem = 1,
}
impl PublicKeyCertificateFormat {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            Self::UnspecifiedPublicKeyCertificateFormat => {
                "UNSPECIFIED_PUBLIC_KEY_CERTIFICATE_FORMAT"
            }
            Self::X509CertificatePem => "X509_CERTIFICATE_PEM",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "UNSPECIFIED_PUBLIC_KEY_CERTIFICATE_FORMAT" => {
                Some(Self::UnspecifiedPublicKeyCertificateFormat)
            }
            "X509_CERTIFICATE_PEM" => Some(Self::X509CertificatePem),
            _ => None,
        }
    }
}
/// The supported formats for the public key.
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum PublicKeyFormat {
    /// The format has not been specified. This is an invalid default value and
    /// must not be used.
    UnspecifiedPublicKeyFormat = 0,
    /// An RSA public key encoded in base64, and wrapped by
    /// `-----BEGIN PUBLIC KEY-----` and `-----END PUBLIC KEY-----`. This can be
    /// used to verify `RS256` signatures in JWT tokens ([RFC7518](
    /// <https://www.ietf.org/rfc/rfc7518.txt>)).
    RsaPem = 3,
    /// As RSA_PEM, but wrapped in an X.509v3 certificate ([RFC5280](
    /// <https://www.ietf.org/rfc/rfc5280.txt>)), encoded in base64, and wrapped by
    /// `-----BEGIN CERTIFICATE-----` and `-----END CERTIFICATE-----`.
    RsaX509Pem = 1,
    /// Public key for the ECDSA algorithm using P-256 and SHA-256, encoded in
    /// base64, and wrapped by `-----BEGIN PUBLIC KEY-----` and `-----END
    /// PUBLIC KEY-----`. This can be used to verify JWT tokens with the `ES256`
    /// algorithm ([RFC7518](<https://www.ietf.org/rfc/rfc7518.txt>)). This curve is
    /// defined in [OpenSSL](<https://www.openssl.org/>) as the `prime256v1` curve.
    Es256Pem = 2,
    /// As ES256_PEM, but wrapped in an X.509v3 certificate ([RFC5280](
    /// <https://www.ietf.org/rfc/rfc5280.txt>)), encoded in base64, and wrapped by
    /// `-----BEGIN CERTIFICATE-----` and `-----END CERTIFICATE-----`.
    Es256X509Pem = 4,
}
impl PublicKeyFormat {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            Self::UnspecifiedPublicKeyFormat => "UNSPECIFIED_PUBLIC_KEY_FORMAT",
            Self::RsaPem => "RSA_PEM",
            Self::RsaX509Pem => "RSA_X509_PEM",
            Self::Es256Pem => "ES256_PEM",
            Self::Es256X509Pem => "ES256_X509_PEM",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "UNSPECIFIED_PUBLIC_KEY_FORMAT" => Some(Self::UnspecifiedPublicKeyFormat),
            "RSA_PEM" => Some(Self::RsaPem),
            "RSA_X509_PEM" => Some(Self::RsaX509Pem),
            "ES256_PEM" => Some(Self::Es256Pem),
            "ES256_X509_PEM" => Some(Self::Es256X509Pem),
            _ => None,
        }
    }
}
/// CreateDevice event.
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateDeviceEvent {
    /// The data associated with the event.
    #[prost(message, optional, tag = "1")]
    pub data: ::core::option::Option<DeviceEventData>,
}
/// UpdateDevice event.
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateDeviceEvent {
    /// The data associated with the event.
    #[prost(message, optional, tag = "1")]
    pub data: ::core::option::Option<DeviceEventData>,
}
/// DeleteDevice event.
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteDeviceEvent {
    /// The data associated with the event.
    #[prost(message, optional, tag = "1")]
    pub data: ::core::option::Option<DeviceEventData>,
}
/// CreateDeviceRegistry event.
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateDeviceRegistryEvent {
    /// The data associated with the event.
    #[prost(message, optional, tag = "1")]
    pub data: ::core::option::Option<RegistryEventData>,
}
/// UpdateDeviceRegistry event.
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateDeviceRegistryEvent {
    /// The data associated with the event.
    #[prost(message, optional, tag = "1")]
    pub data: ::core::option::Option<RegistryEventData>,
}
/// DeleteDeviceRegistry event.
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteDeviceRegistryEvent {
    /// The data associated with the event.
    #[prost(message, optional, tag = "1")]
    pub data: ::core::option::Option<RegistryEventData>,
}
