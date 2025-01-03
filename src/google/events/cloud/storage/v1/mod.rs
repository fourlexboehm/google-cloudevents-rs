// This file is @generated by prost-build.
/// An object within Google Cloud Storage.
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StorageObjectData {
    /// Content-Encoding of the object data, matching
    /// [<https://tools.ietf.org/html/rfc7231#section-3.1.2.2][RFC> 7231 §3.1.2.2]
    #[prost(string, tag = "1")]
    pub content_encoding: ::prost::alloc::string::String,
    /// Content-Disposition of the object data, matching
    /// [<https://tools.ietf.org/html/rfc6266][RFC> 6266].
    #[prost(string, tag = "2")]
    pub content_disposition: ::prost::alloc::string::String,
    /// Cache-Control directive for the object data, matching
    /// [<https://tools.ietf.org/html/rfc7234#section-5.2"][RFC> 7234 §5.2].
    #[prost(string, tag = "3")]
    pub cache_control: ::prost::alloc::string::String,
    /// Content-Language of the object data, matching
    /// [<https://tools.ietf.org/html/rfc7231#section-3.1.3.2][RFC> 7231 §3.1.3.2].
    #[prost(string, tag = "5")]
    pub content_language: ::prost::alloc::string::String,
    /// The version of the metadata for this object at this generation. Used for
    /// preconditions and for detecting changes in metadata. A metageneration
    /// number is only meaningful in the context of a particular generation of a
    /// particular object.
    #[prost(int64, tag = "6")]
    pub metageneration: i64,
    /// The deletion time of the object. Will be returned if and only if this
    /// version of the object has been deleted.
    #[prost(message, optional, tag = "7")]
    pub time_deleted: ::core::option::Option<::pbjson_types::Timestamp>,
    /// Content-Type of the object data, matching
    /// [<https://tools.ietf.org/html/rfc7231#section-3.1.1.5][RFC> 7231 §3.1.1.5].
    /// If an object is stored without a Content-Type, it is served as
    /// `application/octet-stream`.
    #[prost(string, tag = "8")]
    pub content_type: ::prost::alloc::string::String,
    /// Content-Length of the object data in bytes, matching
    /// [<https://tools.ietf.org/html/rfc7230#section-3.3.2][RFC> 7230 §3.3.2].
    #[prost(int64, tag = "9")]
    pub size: i64,
    /// The creation time of the object.
    /// Attempting to set this field will result in an error.
    #[prost(message, optional, tag = "10")]
    pub time_created: ::core::option::Option<::pbjson_types::Timestamp>,
    /// CRC32c checksum. For more information about using the CRC32c
    /// checksum, see
    /// [<https://cloud.google.com/storage/docs/hashes-etags#_JSONAPI][Hashes> and
    /// ETags: Best Practices].
    #[prost(string, tag = "11")]
    pub crc32c: ::prost::alloc::string::String,
    /// Number of underlying components that make up this object. Components are
    /// accumulated by compose operations.
    /// Attempting to set this field will result in an error.
    #[prost(int32, tag = "12")]
    pub component_count: i32,
    /// MD5 hash of the data; encoded using base64 as per
    /// [<https://tools.ietf.org/html/rfc4648#section-4][RFC> 4648 §4]. For more
    /// information about using the MD5 hash, see
    /// [<https://cloud.google.com/storage/docs/hashes-etags#_JSONAPI][Hashes> and
    /// ETags: Best Practices].
    #[prost(string, tag = "13")]
    pub md5_hash: ::prost::alloc::string::String,
    /// HTTP 1.1 Entity tag for the object. See
    /// [<https://tools.ietf.org/html/rfc7232#section-2.3][RFC> 7232 §2.3].
    #[prost(string, tag = "14")]
    pub etag: ::prost::alloc::string::String,
    /// The modification time of the object metadata.
    #[prost(message, optional, tag = "15")]
    pub updated: ::core::option::Option<::pbjson_types::Timestamp>,
    /// Storage class of the object.
    #[prost(string, tag = "16")]
    pub storage_class: ::prost::alloc::string::String,
    /// Cloud KMS Key used to encrypt this object, if the object is encrypted by
    /// such a key.
    #[prost(string, tag = "17")]
    pub kms_key_name: ::prost::alloc::string::String,
    /// The time at which the object's storage class was last changed.
    #[prost(message, optional, tag = "18")]
    pub time_storage_class_updated: ::core::option::Option<::pbjson_types::Timestamp>,
    /// Whether an object is under temporary hold.
    #[prost(bool, tag = "19")]
    pub temporary_hold: bool,
    /// A server-determined value that specifies the earliest time that the
    /// object's retention period expires.
    #[prost(message, optional, tag = "20")]
    pub retention_expiration_time: ::core::option::Option<::pbjson_types::Timestamp>,
    /// User-provided metadata, in key/value pairs.
    #[prost(map = "string, string", tag = "21")]
    pub metadata: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        ::prost::alloc::string::String,
    >,
    /// Whether an object is under event-based hold.
    #[prost(bool, tag = "29")]
    pub event_based_hold: bool,
    /// The name of the object.
    #[prost(string, tag = "23")]
    pub name: ::prost::alloc::string::String,
    /// The ID of the object, including the bucket name, object name, and
    /// generation number.
    #[prost(string, tag = "24")]
    pub id: ::prost::alloc::string::String,
    /// The name of the bucket containing this object.
    #[prost(string, tag = "25")]
    pub bucket: ::prost::alloc::string::String,
    /// The content generation of this object. Used for object versioning.
    /// Attempting to set this field will result in an error.
    #[prost(int64, tag = "26")]
    pub generation: i64,
    /// Metadata of customer-supplied encryption key, if the object is encrypted by
    /// such a key.
    #[prost(message, optional, tag = "28")]
    pub customer_encryption: ::core::option::Option<
        storage_object_data::CustomerEncryption,
    >,
    /// Media download link.
    #[prost(string, tag = "100")]
    pub media_link: ::prost::alloc::string::String,
    /// The link to this object.
    #[prost(string, tag = "101")]
    pub self_link: ::prost::alloc::string::String,
    /// The kind of item this is. For objects, this is always "storage#object".
    #[prost(string, tag = "102")]
    pub kind: ::prost::alloc::string::String,
}
/// Nested message and enum types in `StorageObjectData`.
pub mod storage_object_data {
    /// Describes the customer-specified mechanism used to store the data at rest.
    #[derive(serde::Serialize, serde::Deserialize)]
    #[serde(rename_all = "snake_case")]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct CustomerEncryption {
        /// The encryption algorithm.
        #[prost(string, tag = "1")]
        pub encryption_algorithm: ::prost::alloc::string::String,
        /// SHA256 hash value of the encryption key.
        #[prost(string, tag = "2")]
        pub key_sha256: ::prost::alloc::string::String,
    }
}
/// The CloudEvent raised when an object is finalized in Google Cloud Storage.
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ObjectFinalizedEvent {
    /// The data associated with the event.
    #[prost(message, optional, tag = "1")]
    pub data: ::core::option::Option<StorageObjectData>,
}
/// The CloudEvent raised when an object is archived in Google Cloud Storage.
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ObjectArchivedEvent {
    /// The data associated with the event.
    #[prost(message, optional, tag = "1")]
    pub data: ::core::option::Option<StorageObjectData>,
}
/// The CloudEvent raised when an object is deleted  in Google Cloud Storage.
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ObjectDeletedEvent {
    /// The data associated with the event.
    #[prost(message, optional, tag = "1")]
    pub data: ::core::option::Option<StorageObjectData>,
}
/// The CloudEvent raised when an object's metadata is updated  in Google Cloud
/// Storage.
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ObjectMetadataUpdatedEvent {
    /// The data associated with the event.
    #[prost(message, optional, tag = "1")]
    pub data: ::core::option::Option<StorageObjectData>,
}
