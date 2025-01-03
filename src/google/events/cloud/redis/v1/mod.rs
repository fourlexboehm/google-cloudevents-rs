// This file is @generated by prost-build.
/// Node specific properties.
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NodeInfo {
    /// Output only. Node identifying string. e.g. 'node-0', 'node-1'
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    /// Output only. Location of the node.
    #[prost(string, tag = "2")]
    pub zone: ::prost::alloc::string::String,
}
/// A Memorystore for Redis instance.
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Instance {
    /// Required. Unique name of the resource in this scope including project and
    /// location using the form:
    ///      `projects/{project_id}/locations/{location_id}/instances/{instance_id}`
    ///
    /// Note: Redis instances are managed and addressed at regional level so
    /// location_id here refers to a GCP region; however, users may choose which
    /// specific zone (or collection of zones for cross-zone instances) an instance
    /// should be provisioned in. Refer to
    /// [location_id][google.cloud.redis.v1.Instance.location_id] and
    /// [alternative_location_id][google.cloud.redis.v1.Instance.alternative_location_id]
    /// fields for more details.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// An arbitrary and optional user-provided name for the instance.
    #[prost(string, tag = "2")]
    pub display_name: ::prost::alloc::string::String,
    /// Resource labels to represent user provided metadata
    #[prost(map = "string, string", tag = "3")]
    pub labels: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        ::prost::alloc::string::String,
    >,
    /// Optional. The zone where the instance will be provisioned. If not provided,
    /// the service will choose a zone from the specified region for the instance.
    /// For standard tier, additional nodes will be added across multiple zones for
    /// protection against zonal failures. If specified, at least one node will be
    /// provisioned in this zone.
    #[prost(string, tag = "4")]
    pub location_id: ::prost::alloc::string::String,
    /// Optional. If specified, at least one node will be provisioned in this zone
    /// in addition to the zone specified in location_id. Only applicable to
    /// standard tier. If provided, it must be a different zone from the one
    /// provided in \[location_id\]. Additional nodes beyond the first 2 will be
    /// placed in zones selected by the service.
    #[prost(string, tag = "5")]
    pub alternative_location_id: ::prost::alloc::string::String,
    /// Optional. The version of Redis software.
    /// If not provided, latest supported version will be used. Currently, the
    /// supported values are:
    ///
    ///   *   `REDIS_3_2` for Redis 3.2 compatibility
    ///   *   `REDIS_4_0` for Redis 4.0 compatibility (default)
    ///   *   `REDIS_5_0` for Redis 5.0 compatibility
    ///   *   `REDIS_6_X` for Redis 6.x compatibility
    #[prost(string, tag = "7")]
    pub redis_version: ::prost::alloc::string::String,
    /// Optional. For DIRECT_PEERING mode, the CIDR range of internal addresses
    /// that are reserved for this instance. Range must
    /// be unique and non-overlapping with existing subnets in an authorized
    /// network. For PRIVATE_SERVICE_ACCESS mode, the name of one allocated IP
    /// address ranges associated with this private service access connection.
    /// If not provided, the service will choose an unused /29 block, for
    /// example, 10.0.0.0/29 or 192.168.0.0/29.  For READ_REPLICAS_ENABLED
    /// the default block size is /28.
    #[prost(string, tag = "9")]
    pub reserved_ip_range: ::prost::alloc::string::String,
    /// Optional. Additional IP range for node placement. Required when enabling
    /// read replicas on an existing instance. For DIRECT_PEERING mode value must
    /// be a CIDR range of size /28, or "auto". For PRIVATE_SERVICE_ACCESS mode
    /// value must be the name of an allocated address range associated with the
    /// private service access connection, or "auto".
    #[prost(string, tag = "30")]
    pub secondary_ip_range: ::prost::alloc::string::String,
    /// Output only. Hostname or IP address of the exposed Redis endpoint used by
    /// clients to connect to the service.
    #[prost(string, tag = "10")]
    pub host: ::prost::alloc::string::String,
    /// Output only. The port number of the exposed Redis endpoint.
    #[prost(int32, tag = "11")]
    pub port: i32,
    /// Output only. The current zone where the Redis primary node is located. In
    /// basic tier, this will always be the same as \[location_id\]. In
    /// standard tier, this can be the zone of any node in the instance.
    #[prost(string, tag = "12")]
    pub current_location_id: ::prost::alloc::string::String,
    /// Output only. The time the instance was created.
    #[prost(message, optional, tag = "13")]
    pub create_time: ::core::option::Option<::pbjson_types::Timestamp>,
    /// Output only. The current state of this instance.
    #[prost(enumeration = "instance::State", tag = "14")]
    pub state: i32,
    /// Output only. Additional information about the current status of this
    /// instance, if available.
    #[prost(string, tag = "15")]
    pub status_message: ::prost::alloc::string::String,
    /// Optional. Redis configuration parameters, according to
    /// <http://redis.io/topics/config.> Currently, the only supported parameters
    /// are:
    ///
    ///   Redis version 3.2 and newer:
    ///
    ///   *   maxmemory-policy
    ///   *   notify-keyspace-events
    ///
    ///   Redis version 4.0 and newer:
    ///
    ///   *   activedefrag
    ///   *   lfu-decay-time
    ///   *   lfu-log-factor
    ///   *   maxmemory-gb
    ///
    ///   Redis version 5.0 and newer:
    ///
    ///   *   stream-node-max-bytes
    ///   *   stream-node-max-entries
    #[prost(map = "string, string", tag = "16")]
    pub redis_configs: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        ::prost::alloc::string::String,
    >,
    /// Required. The service tier of the instance.
    #[prost(enumeration = "instance::Tier", tag = "17")]
    pub tier: i32,
    /// Required. Redis memory size in GiB.
    #[prost(int32, tag = "18")]
    pub memory_size_gb: i32,
    /// Optional. The full name of the Google Compute Engine
    /// [network](<https://cloud.google.com/vpc/docs/vpc>) to which the
    /// instance is connected. If left unspecified, the `default` network
    /// will be used.
    #[prost(string, tag = "20")]
    pub authorized_network: ::prost::alloc::string::String,
    /// Output only. Cloud IAM identity used by import / export operations to
    /// transfer data to/from Cloud Storage. Format is
    /// "serviceAccount:<service_account_email>". The value may change over time
    /// for a given instance so should be checked before each import/export
    /// operation.
    #[prost(string, tag = "21")]
    pub persistence_iam_identity: ::prost::alloc::string::String,
    /// Optional. The network connect mode of the Redis instance.
    /// If not provided, the connect mode defaults to DIRECT_PEERING.
    #[prost(enumeration = "instance::ConnectMode", tag = "22")]
    pub connect_mode: i32,
    /// Optional. Indicates whether OSS Redis AUTH is enabled for the instance. If
    /// set to "true" AUTH is enabled on the instance. Default value is "false"
    /// meaning AUTH is disabled.
    #[prost(bool, tag = "23")]
    pub auth_enabled: bool,
    /// Output only. List of server CA certificates for the instance.
    #[prost(message, repeated, tag = "25")]
    pub server_ca_certs: ::prost::alloc::vec::Vec<TlsCertificate>,
    /// Optional. The TLS mode of the Redis instance.
    /// If not provided, TLS is disabled for the instance.
    #[prost(enumeration = "instance::TransitEncryptionMode", tag = "26")]
    pub transit_encryption_mode: i32,
    /// Optional. The maintenance policy for the instance. If not provided,
    /// maintenance events can be performed at any time.
    #[prost(message, optional, tag = "27")]
    pub maintenance_policy: ::core::option::Option<MaintenancePolicy>,
    /// Output only. Date and time of upcoming maintenance events which have been
    /// scheduled.
    #[prost(message, optional, tag = "28")]
    pub maintenance_schedule: ::core::option::Option<MaintenanceSchedule>,
    /// Optional. The number of replica nodes. The valid range for the Standard
    /// Tier with read replicas enabled is \[1-5\] and defaults to 2. If read
    /// replicas are not enabled for a Standard Tier instance, the only valid value
    /// is 1 and the default is 1. The valid value for basic tier is 0 and the
    /// default is also 0.
    #[prost(int32, tag = "31")]
    pub replica_count: i32,
    /// Output only. Info per node.
    #[prost(message, repeated, tag = "32")]
    pub nodes: ::prost::alloc::vec::Vec<NodeInfo>,
    /// Output only. Hostname or IP address of the exposed readonly Redis
    /// endpoint. Standard tier only. Targets all healthy replica nodes in
    /// instance. Replication is asynchronous and replica nodes will exhibit some
    /// lag behind the primary. Write requests must target 'host'.
    #[prost(string, tag = "33")]
    pub read_endpoint: ::prost::alloc::string::String,
    /// Output only. The port number of the exposed readonly redis
    /// endpoint. Standard tier only. Write requests should target 'port'.
    #[prost(int32, tag = "34")]
    pub read_endpoint_port: i32,
    /// Optional. Read replicas mode for the instance. Defaults to
    /// READ_REPLICAS_DISABLED.
    #[prost(enumeration = "instance::ReadReplicasMode", tag = "35")]
    pub read_replicas_mode: i32,
    /// Optional. The KMS key reference that the customer provides when trying to
    /// create the instance.
    #[prost(string, tag = "36")]
    pub customer_managed_key: ::prost::alloc::string::String,
    /// Optional. Persistence configuration parameters
    #[prost(message, optional, tag = "37")]
    pub persistence_config: ::core::option::Option<PersistenceConfig>,
    /// Optional. reasons that causes instance in "SUSPENDED" state.
    #[prost(enumeration = "instance::SuspensionReason", repeated, tag = "38")]
    pub suspension_reasons: ::prost::alloc::vec::Vec<i32>,
    /// Optional. The self service update maintenance version.
    /// The version is date based such as "20210712_00_00".
    #[prost(string, tag = "39")]
    pub maintenance_version: ::prost::alloc::string::String,
    /// Optional. The available maintenance versions that an instance could update
    /// to.
    #[prost(string, repeated, tag = "40")]
    pub available_maintenance_versions: ::prost::alloc::vec::Vec<
        ::prost::alloc::string::String,
    >,
}
/// Nested message and enum types in `Instance`.
pub mod instance {
    /// Represents the different states of a Redis instance.
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
        /// Not set.
        Unspecified = 0,
        /// Redis instance is being created.
        Creating = 1,
        /// Redis instance has been created and is fully usable.
        Ready = 2,
        /// Redis instance configuration is being updated. Certain kinds of updates
        /// may cause the instance to become unusable while the update is in
        /// progress.
        Updating = 3,
        /// Redis instance is being deleted.
        Deleting = 4,
        /// Redis instance is being repaired and may be unusable.
        Repairing = 5,
        /// Maintenance is being performed on this Redis instance.
        Maintenance = 6,
        /// Redis instance is importing data (availability may be affected).
        Importing = 8,
        /// Redis instance is failing over (availability may be affected).
        FailingOver = 9,
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
                Self::Ready => "READY",
                Self::Updating => "UPDATING",
                Self::Deleting => "DELETING",
                Self::Repairing => "REPAIRING",
                Self::Maintenance => "MAINTENANCE",
                Self::Importing => "IMPORTING",
                Self::FailingOver => "FAILING_OVER",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "STATE_UNSPECIFIED" => Some(Self::Unspecified),
                "CREATING" => Some(Self::Creating),
                "READY" => Some(Self::Ready),
                "UPDATING" => Some(Self::Updating),
                "DELETING" => Some(Self::Deleting),
                "REPAIRING" => Some(Self::Repairing),
                "MAINTENANCE" => Some(Self::Maintenance),
                "IMPORTING" => Some(Self::Importing),
                "FAILING_OVER" => Some(Self::FailingOver),
                _ => None,
            }
        }
    }
    /// Available service tiers to choose from
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
    pub enum Tier {
        /// Not set.
        Unspecified = 0,
        /// BASIC tier: standalone instance
        Basic = 1,
        /// STANDARD_HA tier: highly available primary/replica instances
        StandardHa = 3,
    }
    impl Tier {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Self::Unspecified => "TIER_UNSPECIFIED",
                Self::Basic => "BASIC",
                Self::StandardHa => "STANDARD_HA",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "TIER_UNSPECIFIED" => Some(Self::Unspecified),
                "BASIC" => Some(Self::Basic),
                "STANDARD_HA" => Some(Self::StandardHa),
                _ => None,
            }
        }
    }
    /// Available connection modes.
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
    pub enum ConnectMode {
        /// Not set.
        Unspecified = 0,
        /// Connect via direct peering to the Memorystore for Redis hosted service.
        DirectPeering = 1,
        /// Connect your Memorystore for Redis instance using Private Service
        /// Access. Private services access provides an IP address range for multiple
        /// Google Cloud services, including Memorystore.
        PrivateServiceAccess = 2,
    }
    impl ConnectMode {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Self::Unspecified => "CONNECT_MODE_UNSPECIFIED",
                Self::DirectPeering => "DIRECT_PEERING",
                Self::PrivateServiceAccess => "PRIVATE_SERVICE_ACCESS",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "CONNECT_MODE_UNSPECIFIED" => Some(Self::Unspecified),
                "DIRECT_PEERING" => Some(Self::DirectPeering),
                "PRIVATE_SERVICE_ACCESS" => Some(Self::PrivateServiceAccess),
                _ => None,
            }
        }
    }
    /// Available TLS modes.
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
    pub enum TransitEncryptionMode {
        /// Not set.
        Unspecified = 0,
        /// Client to Server traffic encryption enabled with server authentication.
        ServerAuthentication = 1,
        /// TLS is disabled for the instance.
        Disabled = 2,
    }
    impl TransitEncryptionMode {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Self::Unspecified => "TRANSIT_ENCRYPTION_MODE_UNSPECIFIED",
                Self::ServerAuthentication => "SERVER_AUTHENTICATION",
                Self::Disabled => "DISABLED",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "TRANSIT_ENCRYPTION_MODE_UNSPECIFIED" => Some(Self::Unspecified),
                "SERVER_AUTHENTICATION" => Some(Self::ServerAuthentication),
                "DISABLED" => Some(Self::Disabled),
                _ => None,
            }
        }
    }
    /// Read replicas mode.
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
    pub enum ReadReplicasMode {
        /// If not set, Memorystore Redis backend will default to
        /// READ_REPLICAS_DISABLED.
        Unspecified = 0,
        /// If disabled, read endpoint will not be provided and the instance cannot
        /// scale up or down the number of replicas.
        ReadReplicasDisabled = 1,
        /// If enabled, read endpoint will be provided and the instance can scale
        /// up and down the number of replicas. Not valid for basic tier.
        ReadReplicasEnabled = 2,
    }
    impl ReadReplicasMode {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Self::Unspecified => "READ_REPLICAS_MODE_UNSPECIFIED",
                Self::ReadReplicasDisabled => "READ_REPLICAS_DISABLED",
                Self::ReadReplicasEnabled => "READ_REPLICAS_ENABLED",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "READ_REPLICAS_MODE_UNSPECIFIED" => Some(Self::Unspecified),
                "READ_REPLICAS_DISABLED" => Some(Self::ReadReplicasDisabled),
                "READ_REPLICAS_ENABLED" => Some(Self::ReadReplicasEnabled),
                _ => None,
            }
        }
    }
    /// Possible reasons for the instance to be in a "SUSPENDED" state.
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
    pub enum SuspensionReason {
        /// Not set.
        Unspecified = 0,
        /// Something wrong with the CMEK key provided by customer.
        CustomerManagedKeyIssue = 1,
    }
    impl SuspensionReason {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Self::Unspecified => "SUSPENSION_REASON_UNSPECIFIED",
                Self::CustomerManagedKeyIssue => "CUSTOMER_MANAGED_KEY_ISSUE",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "SUSPENSION_REASON_UNSPECIFIED" => Some(Self::Unspecified),
                "CUSTOMER_MANAGED_KEY_ISSUE" => Some(Self::CustomerManagedKeyIssue),
                _ => None,
            }
        }
    }
}
/// Configuration of the persistence functionality.
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct PersistenceConfig {
    /// Optional. Controls whether Persistence features are enabled.
    /// If not provided, the existing value will be used.
    #[prost(enumeration = "persistence_config::PersistenceMode", tag = "1")]
    pub persistence_mode: i32,
    /// Optional. Period between RDB snapshots. Snapshots will be attempted every
    /// period starting from the provided snapshot start time. For example, a start
    /// time of 01/01/2033 06:45 and SIX_HOURS snapshot period will do nothing
    /// until 01/01/2033, and then trigger snapshots every day at 06:45, 12:45,
    /// 18:45, and 00:45 the next day, and so on. If not provided,
    /// TWENTY_FOUR_HOURS will be used as default.
    #[prost(enumeration = "persistence_config::SnapshotPeriod", tag = "2")]
    pub rdb_snapshot_period: i32,
    /// Output only. The next time that a snapshot attempt is scheduled to occur.
    #[prost(message, optional, tag = "4")]
    pub rdb_next_snapshot_time: ::core::option::Option<::pbjson_types::Timestamp>,
    /// Optional. Date and time that the first snapshot was/will be attempted, and
    /// to which future snapshots will be aligned. If not provided, the current
    /// time will be used.
    #[prost(message, optional, tag = "5")]
    pub rdb_snapshot_start_time: ::core::option::Option<::pbjson_types::Timestamp>,
}
/// Nested message and enum types in `PersistenceConfig`.
pub mod persistence_config {
    /// Available Persistence modes.
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
    pub enum PersistenceMode {
        /// Not set.
        Unspecified = 0,
        /// Persistence is disabled for the instance,
        /// and any existing snapshots are deleted.
        Disabled = 1,
        /// RDB based Persistence is enabled.
        Rdb = 2,
    }
    impl PersistenceMode {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Self::Unspecified => "PERSISTENCE_MODE_UNSPECIFIED",
                Self::Disabled => "DISABLED",
                Self::Rdb => "RDB",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "PERSISTENCE_MODE_UNSPECIFIED" => Some(Self::Unspecified),
                "DISABLED" => Some(Self::Disabled),
                "RDB" => Some(Self::Rdb),
                _ => None,
            }
        }
    }
    /// Available snapshot periods for scheduling.
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
    pub enum SnapshotPeriod {
        /// Not set.
        Unspecified = 0,
        /// Snapshot every 1 hour.
        OneHour = 3,
        /// Snapshot every 6 hours.
        SixHours = 4,
        /// Snapshot every 12 hours.
        TwelveHours = 5,
        /// Snapshot every 24 hours.
        TwentyFourHours = 6,
    }
    impl SnapshotPeriod {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Self::Unspecified => "SNAPSHOT_PERIOD_UNSPECIFIED",
                Self::OneHour => "ONE_HOUR",
                Self::SixHours => "SIX_HOURS",
                Self::TwelveHours => "TWELVE_HOURS",
                Self::TwentyFourHours => "TWENTY_FOUR_HOURS",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "SNAPSHOT_PERIOD_UNSPECIFIED" => Some(Self::Unspecified),
                "ONE_HOUR" => Some(Self::OneHour),
                "SIX_HOURS" => Some(Self::SixHours),
                "TWELVE_HOURS" => Some(Self::TwelveHours),
                "TWENTY_FOUR_HOURS" => Some(Self::TwentyFourHours),
                _ => None,
            }
        }
    }
}
/// Maintenance policy for an instance.
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MaintenancePolicy {
    /// Output only. The time when the policy was created.
    #[prost(message, optional, tag = "1")]
    pub create_time: ::core::option::Option<::pbjson_types::Timestamp>,
    /// Output only. The time when the policy was last updated.
    #[prost(message, optional, tag = "2")]
    pub update_time: ::core::option::Option<::pbjson_types::Timestamp>,
    /// Optional. Description of what this policy is for. Create/Update methods
    /// return INVALID_ARGUMENT if the length is greater than 512.
    #[prost(string, tag = "3")]
    pub description: ::prost::alloc::string::String,
    /// Optional. Maintenance window that is applied to resources covered by this
    /// policy. Minimum 1. For the current version, the maximum number of
    /// weekly_window is expected to be one.
    #[prost(message, repeated, tag = "4")]
    pub weekly_maintenance_window: ::prost::alloc::vec::Vec<WeeklyMaintenanceWindow>,
}
/// Time window in which disruptive maintenance updates occur. Non-disruptive
/// updates can occur inside or outside this window.
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct WeeklyMaintenanceWindow {
    /// Required. The day of week that maintenance updates occur.
    #[prost(enumeration = "super::super::super::super::r#type::DayOfWeek", tag = "1")]
    pub day: i32,
    /// Required. Start time of the window in UTC time.
    #[prost(message, optional, tag = "2")]
    pub start_time: ::core::option::Option<
        super::super::super::super::r#type::TimeOfDay,
    >,
    /// Output only. Duration of the maintenance window. The current window is
    /// fixed at 1 hour.
    #[prost(message, optional, tag = "3")]
    pub duration: ::core::option::Option<::pbjson_types::Duration>,
}
/// Upcoming maintenance schedule. If no maintenance is scheduled, fields are not
/// populated.
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct MaintenanceSchedule {
    /// Output only. The start time of any upcoming scheduled maintenance for this
    /// instance.
    #[prost(message, optional, tag = "1")]
    pub start_time: ::core::option::Option<::pbjson_types::Timestamp>,
    /// Output only. The end time of any upcoming scheduled maintenance for this
    /// instance.
    #[prost(message, optional, tag = "2")]
    pub end_time: ::core::option::Option<::pbjson_types::Timestamp>,
    /// If the scheduled maintenance can be rescheduled, default is true.
    #[prost(bool, tag = "3")]
    pub can_reschedule: bool,
    /// Output only. The deadline that the maintenance schedule start time can not
    /// go beyond, including reschedule.
    #[prost(message, optional, tag = "5")]
    pub schedule_deadline_time: ::core::option::Option<::pbjson_types::Timestamp>,
}
/// TlsCertificate Resource
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TlsCertificate {
    /// Serial number, as extracted from the certificate.
    #[prost(string, tag = "1")]
    pub serial_number: ::prost::alloc::string::String,
    /// Output only. The time when the certificate was created in [RFC
    /// 3339](<https://tools.ietf.org/html/rfc3339>) format, for example
    /// `2020-05-18T00:00:00.094Z`.
    #[prost(message, optional, tag = "3")]
    pub create_time: ::core::option::Option<::pbjson_types::Timestamp>,
    /// Output only. The time when the certificate expires in [RFC
    /// 3339](<https://tools.ietf.org/html/rfc3339>) format, for example
    /// `2020-05-18T00:00:00.094Z`.
    #[prost(message, optional, tag = "4")]
    pub expire_time: ::core::option::Option<::pbjson_types::Timestamp>,
    /// Sha1 Fingerprint of the certificate.
    #[prost(string, tag = "5")]
    pub sha1_fingerprint: ::prost::alloc::string::String,
}
/// The data within all Instance events.
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InstanceEventData {
    /// Optional. The Instance event payload. Unset for deletion events.
    #[prost(message, optional, tag = "1")]
    pub payload: ::core::option::Option<Instance>,
}
/// The CloudEvent raised when an Instance is created.
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InstanceCreatedEvent {
    /// The data associated with the event.
    #[prost(message, optional, tag = "1")]
    pub data: ::core::option::Option<InstanceEventData>,
}
/// The CloudEvent raised when an Instance is updated.
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InstanceUpdatedEvent {
    /// The data associated with the event.
    #[prost(message, optional, tag = "1")]
    pub data: ::core::option::Option<InstanceEventData>,
}
/// The CloudEvent raised when an Instance is deleted.
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InstanceDeletedEvent {
    /// The data associated with the event.
    #[prost(message, optional, tag = "1")]
    pub data: ::core::option::Option<InstanceEventData>,
}
