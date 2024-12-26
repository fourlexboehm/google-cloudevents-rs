pub mod google;
pub static ALL_EVENT_PATHS: &[&str] = &[
    // alloydb/v1
    "google_cloudevents::google::events::cloud::alloydb::v1::ClusterCreatedEvent",
    "google_cloudevents::google::events::cloud::alloydb::v1::ClusterUpdatedEvent",
    "google_cloudevents::google::events::cloud::alloydb::v1::ClusterDeletedEvent",
    "google_cloudevents::google::events::cloud::alloydb::v1::InstanceCreatedEvent",
    "google_cloudevents::google::events::cloud::alloydb::v1::InstanceUpdatedEvent",
    "google_cloudevents::google::events::cloud::alloydb::v1::InstanceDeletedEvent",
    "google_cloudevents::google::events::cloud::alloydb::v1::BackupCreatedEvent",
    "google_cloudevents::google::events::cloud::alloydb::v1::BackupUpdatedEvent",
    "google_cloudevents::google::events::cloud::alloydb::v1::BackupDeletedEvent",

    // apigateway/v1
    "google_cloudevents::google::events::cloud::apigateway::v1::GatewayCreatedEvent",
    "google_cloudevents::google::events::cloud::apigateway::v1::GatewayUpdatedEvent",
    "google_cloudevents::google::events::cloud::apigateway::v1::GatewayDeletedEvent",
    "google_cloudevents::google::events::cloud::apigateway::v1::ApiCreatedEvent",
    "google_cloudevents::google::events::cloud::apigateway::v1::ApiUpdatedEvent",
    "google_cloudevents::google::events::cloud::apigateway::v1::ApiDeletedEvent",
    "google_cloudevents::google::events::cloud::apigateway::v1::ApiConfigCreatedEvent",
    "google_cloudevents::google::events::cloud::apigateway::v1::ApiConfigUpdatedEvent",
    "google_cloudevents::google::events::cloud::apigateway::v1::ApiConfigDeletedEvent",

    // apigeeregistry/v1
    "google_cloudevents::google::events::cloud::apigeeregistry::v1::InstanceCreatedEvent",
    "google_cloudevents::google::events::cloud::apigeeregistry::v1::InstanceDeletedEvent",
    "google_cloudevents::google::events::cloud::apigeeregistry::v1::ApiCreatedEvent",
    "google_cloudevents::google::events::cloud::apigeeregistry::v1::ApiUpdatedEvent",
    "google_cloudevents::google::events::cloud::apigeeregistry::v1::ApiDeletedEvent",
    "google_cloudevents::google::events::cloud::apigeeregistry::v1::ApiVersionCreatedEvent",
    "google_cloudevents::google::events::cloud::apigeeregistry::v1::ApiVersionUpdatedEvent",
    "google_cloudevents::google::events::cloud::apigeeregistry::v1::ApiVersionDeletedEvent",
    "google_cloudevents::google::events::cloud::apigeeregistry::v1::ApiSpecCreatedEvent",
    "google_cloudevents::google::events::cloud::apigeeregistry::v1::ApiSpecUpdatedEvent",
    "google_cloudevents::google::events::cloud::apigeeregistry::v1::ApiSpecDeletedEvent",
    "google_cloudevents::google::events::cloud::apigeeregistry::v1::ApiDeploymentCreatedEvent",
    "google_cloudevents::google::events::cloud::apigeeregistry::v1::ApiDeploymentUpdatedEvent",
    "google_cloudevents::google::events::cloud::apigeeregistry::v1::ApiDeploymentDeletedEvent",

    // audit/v1
    "google_cloudevents::google::events::cloud::audit::v1::AuditLogWrittenEvent",

    // batch/v1
    "google_cloudevents::google::events::cloud::batch::v1::StatusEvent",
    "google_cloudevents::google::events::cloud::batch::v1::JobCreatedEvent",
    "google_cloudevents::google::events::cloud::batch::v1::JobDeletedEvent",

    // beyondcorp/appconnections/v1
    "google_cloudevents::google::events::cloud::beyondcorp::appconnections::v1::AppConnectionCreatedEvent",
    "google_cloudevents::google::events::cloud::beyondcorp::appconnections::v1::AppConnectionUpdatedEvent",
    "google_cloudevents::google::events::cloud::beyondcorp::appconnections::v1::AppConnectionDeletedEvent",

    // beyondcorp/appconnectors/v1
    "google_cloudevents::google::events::cloud::beyondcorp::appconnectors::v1::AppConnectorCreatedEvent",
    "google_cloudevents::google::events::cloud::beyondcorp::appconnectors::v1::AppConnectorUpdatedEvent",
    "google_cloudevents::google::events::cloud::beyondcorp::appconnectors::v1::AppConnectorDeletedEvent",

    // beyondcorp/appgateways/v1
    "google_cloudevents::google::events::cloud::beyondcorp::appgateways::v1::AppGatewayCreatedEvent",
    "google_cloudevents::google::events::cloud::beyondcorp::appgateways::v1::AppGatewayDeletedEvent",

    // beyondcorp/clientconnectorservices/v1
    "google_cloudevents::google::events::cloud::beyondcorp::clientconnectorservices::v1::ClientConnectorServiceCreatedEvent",
    "google_cloudevents::google::events::cloud::beyondcorp::clientconnectorservices::v1::ClientConnectorServiceUpdatedEvent",
    "google_cloudevents::google::events::cloud::beyondcorp::clientconnectorservices::v1::ClientConnectorServiceDeletedEvent",

    // beyondcorp/clientgateways/v1
    "google_cloudevents::google::events::cloud::beyondcorp::clientgateways::v1::ClientGatewayCreatedEvent",
    "google_cloudevents::google::events::cloud::beyondcorp::clientgateways::v1::ClientGatewayDeletedEvent",

    // certificatemanager/v1
    "google_cloudevents::google::events::cloud::certificatemanager::v1::CertificateCreatedEvent",
    "google_cloudevents::google::events::cloud::certificatemanager::v1::CertificateUpdatedEvent",
    "google_cloudevents::google::events::cloud::certificatemanager::v1::CertificateDeletedEvent",
    "google_cloudevents::google::events::cloud::certificatemanager::v1::CertificateMapCreatedEvent",
    "google_cloudevents::google::events::cloud::certificatemanager::v1::CertificateMapUpdatedEvent",
    "google_cloudevents::google::events::cloud::certificatemanager::v1::CertificateMapDeletedEvent",
    "google_cloudevents::google::events::cloud::certificatemanager::v1::CertificateMapEntryCreatedEvent",
    "google_cloudevents::google::events::cloud::certificatemanager::v1::CertificateMapEntryUpdatedEvent",
    "google_cloudevents::google::events::cloud::certificatemanager::v1::CertificateMapEntryDeletedEvent",
    "google_cloudevents::google::events::cloud::certificatemanager::v1::DnsAuthorizationCreatedEvent",
    "google_cloudevents::google::events::cloud::certificatemanager::v1::DnsAuthorizationUpdatedEvent",
    "google_cloudevents::google::events::cloud::certificatemanager::v1::DnsAuthorizationDeletedEvent",
    "google_cloudevents::google::events::cloud::certificatemanager::v1::CertificateIssuanceConfigCreatedEvent",
    "google_cloudevents::google::events::cloud::certificatemanager::v1::CertificateIssuanceConfigDeletedEvent",

    // cloudbuild/v1
    "google_cloudevents::google::events::cloud::cloudbuild::v1::CloudBuildEvent",

    // clouddms/v1
    "google_cloudevents::google::events::cloud::clouddms::v1::MigrationJobCreatedEvent",
    "google_cloudevents::google::events::cloud::clouddms::v1::MigrationJobUpdatedEvent",
    "google_cloudevents::google::events::cloud::clouddms::v1::MigrationJobDeletedEvent",
    "google_cloudevents::google::events::cloud::clouddms::v1::ConnectionProfileCreatedEvent",
    "google_cloudevents::google::events::cloud::clouddms::v1::ConnectionProfileUpdatedEvent",
    "google_cloudevents::google::events::cloud::clouddms::v1::ConnectionProfileDeletedEvent",

    // dataflow/v1beta3
    "google_cloudevents::google::events::cloud::dataflow::v1beta3::JobStatusChangedEvent",

    // datafusion/v1
    "google_cloudevents::google::events::cloud::datafusion::v1::InstanceCreatedEvent",
    "google_cloudevents::google::events::cloud::datafusion::v1::InstanceDeletedEvent",
    "google_cloudevents::google::events::cloud::datafusion::v1::InstanceUpdatedEvent",
    "google_cloudevents::google::events::cloud::datafusion::v1::DnsPeeringCreatedEvent",
    "google_cloudevents::google::events::cloud::datafusion::v1::DnsPeeringDeletedEvent",

    // dataplex/v1
    "google_cloudevents::google::events::cloud::dataplex::v1::DataTaxonomyCreatedEvent",
    "google_cloudevents::google::events::cloud::dataplex::v1::DataTaxonomyUpdatedEvent",
    "google_cloudevents::google::events::cloud::dataplex::v1::DataTaxonomyDeletedEvent",
    "google_cloudevents::google::events::cloud::dataplex::v1::DataAttributeBindingCreatedEvent",
    "google_cloudevents::google::events::cloud::dataplex::v1::DataAttributeBindingUpdatedEvent",
    "google_cloudevents::google::events::cloud::dataplex::v1::DataAttributeBindingDeletedEvent",
    "google_cloudevents::google::events::cloud::dataplex::v1::DataAttributeCreatedEvent",
    "google_cloudevents::google::events::cloud::dataplex::v1::DataAttributeUpdatedEvent",
    "google_cloudevents::google::events::cloud::dataplex::v1::DataAttributeDeletedEvent",
    "google_cloudevents::google::events::cloud::dataplex::v1::DataScanCreatedEvent",
    "google_cloudevents::google::events::cloud::dataplex::v1::DataScanUpdatedEvent",
    "google_cloudevents::google::events::cloud::dataplex::v1::DataScanDeletedEvent",
    "google_cloudevents::google::events::cloud::dataplex::v1::LakeCreatedEvent",
    "google_cloudevents::google::events::cloud::dataplex::v1::LakeUpdatedEvent",
    "google_cloudevents::google::events::cloud::dataplex::v1::LakeDeletedEvent",
    "google_cloudevents::google::events::cloud::dataplex::v1::ZoneCreatedEvent",
    "google_cloudevents::google::events::cloud::dataplex::v1::ZoneUpdatedEvent",
    "google_cloudevents::google::events::cloud::dataplex::v1::ZoneDeletedEvent",
    "google_cloudevents::google::events::cloud::dataplex::v1::AssetCreatedEvent",
    "google_cloudevents::google::events::cloud::dataplex::v1::AssetUpdatedEvent",
    "google_cloudevents::google::events::cloud::dataplex::v1::AssetDeletedEvent",
    "google_cloudevents::google::events::cloud::dataplex::v1::TaskCreatedEvent",
    "google_cloudevents::google::events::cloud::dataplex::v1::TaskUpdatedEvent",
    "google_cloudevents::google::events::cloud::dataplex::v1::TaskDeletedEvent",
    "google_cloudevents::google::events::cloud::dataplex::v1::EnvironmentCreatedEvent",
    "google_cloudevents::google::events::cloud::dataplex::v1::EnvironmentUpdatedEvent",
    "google_cloudevents::google::events::cloud::dataplex::v1::EnvironmentDeletedEvent",

    // datastore/v1
    "google_cloudevents::google::events::cloud::datastore::v1::EntityCreatedEvent",
    "google_cloudevents::google::events::cloud::datastore::v1::EntityUpdatedEvent",
    "google_cloudevents::google::events::cloud::datastore::v1::EntityDeletedEvent",
    "google_cloudevents::google::events::cloud::datastore::v1::EntityWrittenEvent",

    // datastream/v1
    "google_cloudevents::google::events::cloud::datastream::v1::ConnectionProfileCreatedEvent",
    "google_cloudevents::google::events::cloud::datastream::v1::ConnectionProfileUpdatedEvent",
    "google_cloudevents::google::events::cloud::datastream::v1::ConnectionProfileDeletedEvent",
    "google_cloudevents::google::events::cloud::datastream::v1::StreamCreatedEvent",
    "google_cloudevents::google::events::cloud::datastream::v1::StreamUpdatedEvent",
    "google_cloudevents::google::events::cloud::datastream::v1::StreamDeletedEvent",
    "google_cloudevents::google::events::cloud::datastream::v1::PrivateConnectionCreatedEvent",
    "google_cloudevents::google::events::cloud::datastream::v1::PrivateConnectionDeletedEvent",
    "google_cloudevents::google::events::cloud::datastream::v1::RouteCreatedEvent",
    "google_cloudevents::google::events::cloud::datastream::v1::RouteDeletedEvent",

    // deploy/v1
    "google_cloudevents::google::events::cloud::deploy::v1::DeliveryPipelineCreatedEvent",
    "google_cloudevents::google::events::cloud::deploy::v1::DeliveryPipelineUpdatedEvent",
    "google_cloudevents::google::events::cloud::deploy::v1::DeliveryPipelineDeletedEvent",
    "google_cloudevents::google::events::cloud::deploy::v1::TargetCreatedEvent",
    "google_cloudevents::google::events::cloud::deploy::v1::TargetUpdatedEvent",
    "google_cloudevents::google::events::cloud::deploy::v1::TargetDeletedEvent",
    "google_cloudevents::google::events::cloud::deploy::v1::CustomTargetTypeCreatedEvent",
    "google_cloudevents::google::events::cloud::deploy::v1::CustomTargetTypeUpdatedEvent",
    "google_cloudevents::google::events::cloud::deploy::v1::CustomTargetTypeDeletedEvent",
    "google_cloudevents::google::events::cloud::deploy::v1::ReleaseCreatedEvent",
    "google_cloudevents::google::events::cloud::deploy::v1::RolloutCreatedEvent",
    "google_cloudevents::google::events::cloud::deploy::v1::AutomationCreatedEvent",
    "google_cloudevents::google::events::cloud::deploy::v1::AutomationUpdatedEvent",
    "google_cloudevents::google::events::cloud::deploy::v1::AutomationDeletedEvent",

    // eventarc/v1
    "google_cloudevents::google::events::cloud::eventarc::v1::TriggerCreatedEvent",
    "google_cloudevents::google::events::cloud::eventarc::v1::TriggerUpdatedEvent",
    "google_cloudevents::google::events::cloud::eventarc::v1::TriggerDeletedEvent",
    "google_cloudevents::google::events::cloud::eventarc::v1::ChannelCreatedEvent",
    "google_cloudevents::google::events::cloud::eventarc::v1::ChannelUpdatedEvent",
    "google_cloudevents::google::events::cloud::eventarc::v1::ChannelDeletedEvent",
    "google_cloudevents::google::events::cloud::eventarc::v1::ChannelConnectionCreatedEvent",
    "google_cloudevents::google::events::cloud::eventarc::v1::ChannelConnectionDeletedEvent",

    // firestore/v1
    "google_cloudevents::google::events::cloud::firestore::v1::DocumentCreatedEvent",
    "google_cloudevents::google::events::cloud::firestore::v1::DocumentUpdatedEvent",
    "google_cloudevents::google::events::cloud::firestore::v1::DocumentDeletedEvent",
    "google_cloudevents::google::events::cloud::firestore::v1::DocumentWrittenEvent",

    // functions/v2
    "google_cloudevents::google::events::cloud::functions::v2::FunctionCreatedEvent",
    "google_cloudevents::google::events::cloud::functions::v2::FunctionUpdatedEvent",
    "google_cloudevents::google::events::cloud::functions::v2::FunctionDeletedEvent",

    // gkebackup/v1
    "google_cloudevents::google::events::cloud::gkebackup::v1::BackupPlanCreatedEvent",
    "google_cloudevents::google::events::cloud::gkebackup::v1::BackupPlanUpdatedEvent",
    "google_cloudevents::google::events::cloud::gkebackup::v1::BackupPlanDeletedEvent",
    "google_cloudevents::google::events::cloud::gkebackup::v1::BackupCreatedEvent",
    "google_cloudevents::google::events::cloud::gkebackup::v1::BackupUpdatedEvent",
    "google_cloudevents::google::events::cloud::gkebackup::v1::BackupDeletedEvent",
    "google_cloudevents::google::events::cloud::gkebackup::v1::RestorePlanCreatedEvent",
    "google_cloudevents::google::events::cloud::gkebackup::v1::RestorePlanUpdatedEvent",
    "google_cloudevents::google::events::cloud::gkebackup::v1::RestorePlanDeletedEvent",
    "google_cloudevents::google::events::cloud::gkebackup::v1::RestoreCreatedEvent",
    "google_cloudevents::google::events::cloud::gkebackup::v1::RestoreUpdatedEvent",
    "google_cloudevents::google::events::cloud::gkebackup::v1::RestoreDeletedEvent",

    // gkehub/v1
    "google_cloudevents::google::events::cloud::gkehub::v1::MembershipCreatedEvent",
    "google_cloudevents::google::events::cloud::gkehub::v1::FeatureCreatedEvent",
    "google_cloudevents::google::events::cloud::gkehub::v1::MembershipDeletedEvent",
    "google_cloudevents::google::events::cloud::gkehub::v1::FeatureDeletedEvent",
    "google_cloudevents::google::events::cloud::gkehub::v1::MembershipUpdatedEvent",
    "google_cloudevents::google::events::cloud::gkehub::v1::FeatureUpdatedEvent",
    "google_cloudevents::google::events::cloud::gkehub::v1::ScopeCreatedEvent",
    "google_cloudevents::google::events::cloud::gkehub::v1::ScopeDeletedEvent",
    "google_cloudevents::google::events::cloud::gkehub::v1::MembershipBindingCreatedEvent",
    "google_cloudevents::google::events::cloud::gkehub::v1::MembershipBindingUpdatedEvent",
    "google_cloudevents::google::events::cloud::gkehub::v1::MembershipBindingDeletedEvent",

    // iot/v1
    "google_cloudevents::google::events::cloud::iot::v1::CreateDeviceEvent",
    "google_cloudevents::google::events::cloud::iot::v1::UpdateDeviceEvent",
    "google_cloudevents::google::events::cloud::iot::v1::DeleteDeviceEvent",
    "google_cloudevents::google::events::cloud::iot::v1::CreateDeviceRegistryEvent",
    "google_cloudevents::google::events::cloud::iot::v1::UpdateDeviceRegistryEvent",
    "google_cloudevents::google::events::cloud::iot::v1::DeleteDeviceRegistryEvent",

    // memcache/v1
    "google_cloudevents::google::events::cloud::memcache::v1::InstanceCreatedEvent",
    "google_cloudevents::google::events::cloud::memcache::v1::InstanceUpdatedEvent",
    "google_cloudevents::google::events::cloud::memcache::v1::InstanceDeletedEvent",

    // metastore/v1
    "google_cloudevents::google::events::cloud::metastore::v1::FederationCreatedEvent",
    "google_cloudevents::google::events::cloud::metastore::v1::FederationUpdatedEvent",
    "google_cloudevents::google::events::cloud::metastore::v1::FederationDeletedEvent",
    "google_cloudevents::google::events::cloud::metastore::v1::ServiceCreatedEvent",
    "google_cloudevents::google::events::cloud::metastore::v1::ServiceUpdatedEvent",
    "google_cloudevents::google::events::cloud::metastore::v1::ServiceDeletedEvent",
    "google_cloudevents::google::events::cloud::metastore::v1::MetadataImportCreatedEvent",
    "google_cloudevents::google::events::cloud::metastore::v1::MetadataImportUpdatedEvent",
    "google_cloudevents::google::events::cloud::metastore::v1::BackupCreatedEvent",
    "google_cloudevents::google::events::cloud::metastore::v1::BackupDeletedEvent",

    // networkconnectivity/v1
    "google_cloudevents::google::events::cloud::networkconnectivity::v1::ServiceConnectionMapCreatedEvent",
    "google_cloudevents::google::events::cloud::networkconnectivity::v1::ServiceConnectionMapUpdatedEvent",
    "google_cloudevents::google::events::cloud::networkconnectivity::v1::ServiceConnectionMapDeletedEvent",
    "google_cloudevents::google::events::cloud::networkconnectivity::v1::ServiceConnectionPolicyCreatedEvent",
    "google_cloudevents::google::events::cloud::networkconnectivity::v1::ServiceConnectionPolicyUpdatedEvent",
    "google_cloudevents::google::events::cloud::networkconnectivity::v1::ServiceConnectionPolicyDeletedEvent",
    "google_cloudevents::google::events::cloud::networkconnectivity::v1::ServiceClassUpdatedEvent",
    "google_cloudevents::google::events::cloud::networkconnectivity::v1::ServiceClassDeletedEvent",
    "google_cloudevents::google::events::cloud::networkconnectivity::v1::ServiceConnectionTokenCreatedEvent",
    "google_cloudevents::google::events::cloud::networkconnectivity::v1::ServiceConnectionTokenDeletedEvent",
    "google_cloudevents::google::events::cloud::networkconnectivity::v1::HubCreatedEvent",
    "google_cloudevents::google::events::cloud::networkconnectivity::v1::HubUpdatedEvent",
    "google_cloudevents::google::events::cloud::networkconnectivity::v1::HubDeletedEvent",
    "google_cloudevents::google::events::cloud::networkconnectivity::v1::SpokeCreatedEvent",
    "google_cloudevents::google::events::cloud::networkconnectivity::v1::SpokeUpdatedEvent",
    "google_cloudevents::google::events::cloud::networkconnectivity::v1::SpokeDeletedEvent",

    // networkmanagement/v1
    "google_cloudevents::google::events::cloud::networkmanagement::v1::ConnectivityTestCreatedEvent",
    "google_cloudevents::google::events::cloud::networkmanagement::v1::ConnectivityTestUpdatedEvent",
    "google_cloudevents::google::events::cloud::networkmanagement::v1::ConnectivityTestDeletedEvent",

    // networkservices/v1
    "google_cloudevents::google::events::cloud::networkservices::v1::EndpointPolicyCreatedEvent",
    "google_cloudevents::google::events::cloud::networkservices::v1::EndpointPolicyUpdatedEvent",
    "google_cloudevents::google::events::cloud::networkservices::v1::EndpointPolicyDeletedEvent",
    "google_cloudevents::google::events::cloud::networkservices::v1::GatewayCreatedEvent",
    "google_cloudevents::google::events::cloud::networkservices::v1::GatewayUpdatedEvent",
    "google_cloudevents::google::events::cloud::networkservices::v1::GatewayDeletedEvent",
    "google_cloudevents::google::events::cloud::networkservices::v1::GrpcRouteCreatedEvent",
    "google_cloudevents::google::events::cloud::networkservices::v1::GrpcRouteUpdatedEvent",
    "google_cloudevents::google::events::cloud::networkservices::v1::GrpcRouteDeletedEvent",
    "google_cloudevents::google::events::cloud::networkservices::v1::HttpRouteCreatedEvent",
    "google_cloudevents::google::events::cloud::networkservices::v1::HttpRouteUpdatedEvent",
    "google_cloudevents::google::events::cloud::networkservices::v1::HttpRouteDeletedEvent",
    "google_cloudevents::google::events::cloud::networkservices::v1::TcpRouteCreatedEvent",
    "google_cloudevents::google::events::cloud::networkservices::v1::TcpRouteUpdatedEvent",
    "google_cloudevents::google::events::cloud::networkservices::v1::TcpRouteDeletedEvent",
    "google_cloudevents::google::events::cloud::networkservices::v1::TlsRouteCreatedEvent",
    "google_cloudevents::google::events::cloud::networkservices::v1::TlsRouteUpdatedEvent",
    "google_cloudevents::google::events::cloud::networkservices::v1::TlsRouteDeletedEvent",
    "google_cloudevents::google::events::cloud::networkservices::v1::ServiceBindingCreatedEvent",
    "google_cloudevents::google::events::cloud::networkservices::v1::ServiceBindingDeletedEvent",
    "google_cloudevents::google::events::cloud::networkservices::v1::MeshCreatedEvent",
    "google_cloudevents::google::events::cloud::networkservices::v1::MeshUpdatedEvent",
    "google_cloudevents::google::events::cloud::networkservices::v1::MeshDeletedEvent",

    // notebooks/v1
    "google_cloudevents::google::events::cloud::notebooks::v1::RuntimeCreatedEvent",
    "google_cloudevents::google::events::cloud::notebooks::v1::RuntimeUpdatedEvent",
    "google_cloudevents::google::events::cloud::notebooks::v1::RuntimeDeletedEvent",
    "google_cloudevents::google::events::cloud::notebooks::v1::InstanceCreatedEvent",
    "google_cloudevents::google::events::cloud::notebooks::v1::InstanceDeletedEvent",
    "google_cloudevents::google::events::cloud::notebooks::v1::EnvironmentCreatedEvent",
    "google_cloudevents::google::events::cloud::notebooks::v1::EnvironmentDeletedEvent",
    "google_cloudevents::google::events::cloud::notebooks::v1::ScheduleDeletedEvent",
    "google_cloudevents::google::events::cloud::notebooks::v1::ScheduleCreatedEvent",
    "google_cloudevents::google::events::cloud::notebooks::v1::ExecutionDeletedEvent",
    "google_cloudevents::google::events::cloud::notebooks::v1::ExecutionCreatedEvent",

    // pubsub/v1
    "google_cloudevents::google::events::cloud::pubsub::v1::MessagePublishedEvent",

    // redis/v1
    "google_cloudevents::google::events::cloud::redis::v1::InstanceCreatedEvent",
    "google_cloudevents::google::events::cloud::redis::v1::InstanceUpdatedEvent",
    "google_cloudevents::google::events::cloud::redis::v1::InstanceDeletedEvent",

    // scheduler/v1
    "google_cloudevents::google::events::cloud::scheduler::v1::JobCreatedEvent",
    "google_cloudevents::google::events::cloud::scheduler::v1::JobUpdatedEvent",
    "google_cloudevents::google::events::cloud::scheduler::v1::JobDeletedEvent",
    "google_cloudevents::google::events::cloud::scheduler::v1::JobExecutedEvent",

    // speech/v1
    "google_cloudevents::google::events::cloud::speech::v1::PhraseSetCreatedEvent",
    "google_cloudevents::google::events::cloud::speech::v1::PhraseSetUpdatedEvent",
    "google_cloudevents::google::events::cloud::speech::v1::PhraseSetDeletedEvent",
    "google_cloudevents::google::events::cloud::speech::v1::CustomClassCreatedEvent",
    "google_cloudevents::google::events::cloud::speech::v1::CustomClassUpdatedEvent",
    "google_cloudevents::google::events::cloud::speech::v1::CustomClassDeletedEvent",

    // storage/v1
    "google_cloudevents::google::events::cloud::storage::v1::ObjectFinalizedEvent",
    "google_cloudevents::google::events::cloud::storage::v1::ObjectArchivedEvent",
    "google_cloudevents::google::events::cloud::storage::v1::ObjectDeletedEvent",
    "google_cloudevents::google::events::cloud::storage::v1::ObjectMetadataUpdatedEvent",

    // video/transcoder/v1
    "google_cloudevents::google::events::cloud::video::transcoder::v1::JobCreatedEvent",
    "google_cloudevents::google::events::cloud::video::transcoder::v1::JobDeletedEvent",
    "google_cloudevents::google::events::cloud::video::transcoder::v1::JobTemplateCreatedEvent",
    "google_cloudevents::google::events::cloud::video::transcoder::v1::JobTemplateDeletedEvent",

    // visionai/v1
    "google_cloudevents::google::events::cloud::visionai::v1::Event",
    // (Skipping pub mod event, since it's not a struct)
    "google_cloudevents::google::events::cloud::visionai::v1::AnalysisCreatedEvent",
    "google_cloudevents::google::events::cloud::visionai::v1::AnalysisUpdatedEvent",
    "google_cloudevents::google::events::cloud::visionai::v1::AnalysisDeletedEvent",
    "google_cloudevents::google::events::cloud::visionai::v1::ProcessCreatedEvent",
    "google_cloudevents::google::events::cloud::visionai::v1::ProcessUpdatedEvent",
    "google_cloudevents::google::events::cloud::visionai::v1::ProcessDeletedEvent",
    "google_cloudevents::google::events::cloud::visionai::v1::ApplicationCreatedEvent",
    "google_cloudevents::google::events::cloud::visionai::v1::ApplicationUpdatedEvent",
    "google_cloudevents::google::events::cloud::visionai::v1::ApplicationDeletedEvent",
    "google_cloudevents::google::events::cloud::visionai::v1::DraftCreatedEvent",
    "google_cloudevents::google::events::cloud::visionai::v1::DraftUpdatedEvent",
    "google_cloudevents::google::events::cloud::visionai::v1::DraftDeletedEvent",
    "google_cloudevents::google::events::cloud::visionai::v1::ProcessorCreatedEvent",
    "google_cloudevents::google::events::cloud::visionai::v1::ProcessorUpdatedEvent",
    "google_cloudevents::google::events::cloud::visionai::v1::ProcessorDeletedEvent",
    "google_cloudevents::google::events::cloud::visionai::v1::ClusterCreatedEvent",
    "google_cloudevents::google::events::cloud::visionai::v1::ClusterUpdatedEvent",
    "google_cloudevents::google::events::cloud::visionai::v1::ClusterDeletedEvent",
    "google_cloudevents::google::events::cloud::visionai::v1::StreamCreatedEvent",
    "google_cloudevents::google::events::cloud::visionai::v1::StreamUpdatedEvent",
    "google_cloudevents::google::events::cloud::visionai::v1::StreamDeletedEvent",
    "google_cloudevents::google::events::cloud::visionai::v1::EventCreatedEvent",
    "google_cloudevents::google::events::cloud::visionai::v1::EventUpdatedEvent",
    "google_cloudevents::google::events::cloud::visionai::v1::EventDeletedEvent",
    "google_cloudevents::google::events::cloud::visionai::v1::SeriesCreatedEvent",
    "google_cloudevents::google::events::cloud::visionai::v1::SeriesUpdatedEvent",
    "google_cloudevents::google::events::cloud::visionai::v1::SeriesDeletedEvent",

    // vmmigration/v1
    "google_cloudevents::google::events::cloud::vmmigration::v1::SourceCreatedEvent",
    "google_cloudevents::google::events::cloud::vmmigration::v1::SourceUpdatedEvent",
    "google_cloudevents::google::events::cloud::vmmigration::v1::SourceDeletedEvent",
    "google_cloudevents::google::events::cloud::vmmigration::v1::UtilizationReportCreatedEvent",
    "google_cloudevents::google::events::cloud::vmmigration::v1::UtilizationReportDeletedEvent",
    "google_cloudevents::google::events::cloud::vmmigration::v1::DatacenterConnectorCreatedEvent",
    "google_cloudevents::google::events::cloud::vmmigration::v1::DatacenterConnectorDeletedEvent",
    "google_cloudevents::google::events::cloud::vmmigration::v1::MigratingVmCreatedEvent",
    "google_cloudevents::google::events::cloud::vmmigration::v1::MigratingVmUpdatedEvent",
    "google_cloudevents::google::events::cloud::vmmigration::v1::MigratingVmDeletedEvent",
    "google_cloudevents::google::events::cloud::vmmigration::v1::CloneJobCreatedEvent",
    "google_cloudevents::google::events::cloud::vmmigration::v1::CutoverJobCreatedEvent",
    "google_cloudevents::google::events::cloud::vmmigration::v1::GroupCreatedEvent",
    "google_cloudevents::google::events::cloud::vmmigration::v1::GroupUpdatedEvent",
    "google_cloudevents::google::events::cloud::vmmigration::v1::GroupDeletedEvent",
    "google_cloudevents::google::events::cloud::vmmigration::v1::TargetProjectCreatedEvent",
    "google_cloudevents::google::events::cloud::vmmigration::v1::TargetProjectUpdatedEvent",
    "google_cloudevents::google::events::cloud::vmmigration::v1::TargetProjectDeletedEvent",

    // workflows/v1
    "google_cloudevents::google::events::cloud::workflows::v1::WorkflowCreatedEvent",
    "google_cloudevents::google::events::cloud::workflows::v1::WorkflowDeletedEvent",
    "google_cloudevents::google::events::cloud::workflows::v1::WorkflowUpdatedEvent",

    // firebase/analytics/v1
    "google_cloudevents::google::events::firebase::analytics::v1::AnalyticsLogWrittenEvent",

    // firebase/auth/v1
    "google_cloudevents::google::events::firebase::auth::v1::UserCreatedEvent",
    "google_cloudevents::google::events::firebase::auth::v1::UserDeletedEvent",

    // firebase/database/v1
    "google_cloudevents::google::events::firebase::database::v1::ReferenceCreatedEvent",
    "google_cloudevents::google::events::firebase::database::v1::ReferenceUpdatedEvent",
    "google_cloudevents::google::events::firebase::database::v1::ReferenceDeletedEvent",
    "google_cloudevents::google::events::firebase::database::v1::ReferenceWrittenEvent",

    // firebase/firebasealerts/v1
    "google_cloudevents::google::events::firebase::firebasealerts::v1::AlertPublishedEvent",

    // firebase/remoteconfig/v1
    "google_cloudevents::google::events::firebase::remoteconfig::v1::RemoteConfigUpdatedEvent",

    // firebase/testlab/v1
    "google_cloudevents::google::events::firebase::testlab::v1::TestMatrixCompletedEvent",
];