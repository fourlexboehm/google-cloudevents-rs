pub mod cloud;
pub mod firebase;
// This file is @generated by prost-build.
/// Description of an extension attribute.
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExtensionAttribute {
    /// Name of the CloudEvents attribute, e.g. "topic".
    /// This must be all lower-case, to satisfy CloudEvent requirements.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Description of the attribute.
    #[prost(string, tag = "2")]
    pub description: ::prost::alloc::string::String,
    /// The name of the CloudEvents attribute in lower Camel case, e.g. "firebaseDatabaseHost".
    /// This only needs to be populated if the name would otherwise be mis-represented.
    #[prost(string, tag = "3")]
    pub camel_case_name: ::prost::alloc::string::String,
}
