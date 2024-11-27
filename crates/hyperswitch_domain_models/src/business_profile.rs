use common_utils::{
    crypto::OptionalEncryptableValue,
    date_time,
    encryption::Encryption,
    errors::{CustomResult, ValidationError},
    pii, type_name,
    types::{keymanager, AlwaysRequestExtendedAuthorization},
};
use diesel_models::business_profile::{
    AuthenticationConnectorDetails, BusinessPaymentLinkConfig, BusinessPayoutLinkConfig,
    ProfileUpdateInternal, WebhookDetails,
};
use error_stack::ResultExt;
use masking::{PeekInterface, Secret};

use crate::{
    consts,
    type_encryption::{crypto_operation, AsyncLift, CryptoOperation},
};

#[cfg(feature = "v1")]
#[derive(Clone, Debug)]
pub struct Profile {
    profile_id: common_utils::id_type::ProfileId,
    pub merchant_id: common_utils::id_type::MerchantId,
    pub profile_name: String,
    pub created_at: time::PrimitiveDateTime,
    pub modified_at: time::PrimitiveDateTime,
    pub return_url: Option<String>,
    pub enable_payment_response_hash: bool,
    pub payment_response_hash_key: Option<String>,
    pub redirect_to_merchant_with_http_post: bool,
    pub webhook_details: Option<WebhookDetails>,
    pub metadata: Option<pii::SecretSerdeValue>,
    pub routing_algorithm: Option<serde_json::Value>,
    pub intent_fulfillment_time: Option<i64>,
    pub frm_routing_algorithm: Option<serde_json::Value>,
    pub payout_routing_algorithm: Option<serde_json::Value>,
    pub is_recon_enabled: bool,
    pub applepay_verified_domains: Option<Vec<String>>,
    pub payment_link_config: Option<BusinessPaymentLinkConfig>,
    pub session_expiry: Option<i64>,
    pub authentication_connector_details: Option<AuthenticationConnectorDetails>,
    pub payout_link_config: Option<BusinessPayoutLinkConfig>,
    pub is_extended_card_info_enabled: Option<bool>,
    pub extended_card_info_config: Option<pii::SecretSerdeValue>,
    pub is_connector_agnostic_mit_enabled: Option<bool>,
    pub use_billing_as_payment_method_billing: Option<bool>,
    pub collect_shipping_details_from_wallet_connector: Option<bool>,
    pub collect_billing_details_from_wallet_connector: Option<bool>,
    pub outgoing_webhook_custom_http_headers: OptionalEncryptableValue,
    pub always_collect_billing_details_from_wallet_connector: Option<bool>,
    pub always_collect_shipping_details_from_wallet_connector: Option<bool>,
    pub tax_connector_id: Option<common_utils::id_type::MerchantConnectorAccountId>,
    pub is_tax_connector_enabled: bool,
    pub version: common_enums::ApiVersion,
    pub dynamic_routing_algorithm: Option<serde_json::Value>,
    pub is_network_tokenization_enabled: bool,
    pub is_auto_retries_enabled: bool,
    pub max_auto_retries_enabled: Option<i16>,
    pub always_request_extended_authorization: Option<AlwaysRequestExtendedAuthorization>,
}

#[cfg(feature = "v1")]
pub struct ProfileSetter {
    pub profile_id: common_utils::id_type::ProfileId,
    pub merchant_id: common_utils::id_type::MerchantId,
    pub profile_name: String,
    pub created_at: time::PrimitiveDateTime,
    pub modified_at: time::PrimitiveDateTime,
    pub return_url: Option<String>,
    pub enable_payment_response_hash: bool,
    pub payment_response_hash_key: Option<String>,
    pub redirect_to_merchant_with_http_post: bool,
    pub webhook_details: Option<WebhookDetails>,
    pub metadata: Option<pii::SecretSerdeValue>,
    pub routing_algorithm: Option<serde_json::Value>,
    pub intent_fulfillment_time: Option<i64>,
    pub frm_routing_algorithm: Option<serde_json::Value>,
    pub payout_routing_algorithm: Option<serde_json::Value>,
    pub is_recon_enabled: bool,
    pub applepay_verified_domains: Option<Vec<String>>,
    pub payment_link_config: Option<BusinessPaymentLinkConfig>,
    pub session_expiry: Option<i64>,
    pub authentication_connector_details: Option<AuthenticationConnectorDetails>,
    pub payout_link_config: Option<BusinessPayoutLinkConfig>,
    pub is_extended_card_info_enabled: Option<bool>,
    pub extended_card_info_config: Option<pii::SecretSerdeValue>,
    pub is_connector_agnostic_mit_enabled: Option<bool>,
    pub use_billing_as_payment_method_billing: Option<bool>,
    pub collect_shipping_details_from_wallet_connector: Option<bool>,
    pub collect_billing_details_from_wallet_connector: Option<bool>,
    pub outgoing_webhook_custom_http_headers: OptionalEncryptableValue,
    pub always_collect_billing_details_from_wallet_connector: Option<bool>,
    pub always_collect_shipping_details_from_wallet_connector: Option<bool>,
    pub tax_connector_id: Option<common_utils::id_type::MerchantConnectorAccountId>,
    pub is_tax_connector_enabled: bool,
    pub dynamic_routing_algorithm: Option<serde_json::Value>,
    pub is_network_tokenization_enabled: bool,
    pub is_auto_retries_enabled: bool,
    pub max_auto_retries_enabled: Option<i16>,
    pub always_request_extended_authorization: Option<AlwaysRequestExtendedAuthorization>,
}

#[cfg(feature = "v1")]
impl From<ProfileSetter> for Profile {
    fn from(value: ProfileSetter) -> Self {
        Self {
            profile_id: value.profile_id,
            merchant_id: value.merchant_id,
            profile_name: value.profile_name,
            created_at: value.created_at,
            modified_at: value.modified_at,
            return_url: value.return_url,
            enable_payment_response_hash: value.enable_payment_response_hash,
            payment_response_hash_key: value.payment_response_hash_key,
            redirect_to_merchant_with_http_post: value.redirect_to_merchant_with_http_post,
            webhook_details: value.webhook_details,
            metadata: value.metadata,
            routing_algorithm: value.routing_algorithm,
            intent_fulfillment_time: value.intent_fulfillment_time,
            frm_routing_algorithm: value.frm_routing_algorithm,
            payout_routing_algorithm: value.payout_routing_algorithm,
            is_recon_enabled: value.is_recon_enabled,
            applepay_verified_domains: value.applepay_verified_domains,
            payment_link_config: value.payment_link_config,
            session_expiry: value.session_expiry,
            authentication_connector_details: value.authentication_connector_details,
            payout_link_config: value.payout_link_config,
            is_extended_card_info_enabled: value.is_extended_card_info_enabled,
            extended_card_info_config: value.extended_card_info_config,
            is_connector_agnostic_mit_enabled: value.is_connector_agnostic_mit_enabled,
            use_billing_as_payment_method_billing: value.use_billing_as_payment_method_billing,
            collect_shipping_details_from_wallet_connector: value
                .collect_shipping_details_from_wallet_connector,
            collect_billing_details_from_wallet_connector: value
                .collect_billing_details_from_wallet_connector,
            outgoing_webhook_custom_http_headers: value.outgoing_webhook_custom_http_headers,
            always_collect_billing_details_from_wallet_connector: value
                .always_collect_billing_details_from_wallet_connector,
            always_collect_shipping_details_from_wallet_connector: value
                .always_collect_shipping_details_from_wallet_connector,
            tax_connector_id: value.tax_connector_id,
            is_tax_connector_enabled: value.is_tax_connector_enabled,
            version: consts::API_VERSION,
            dynamic_routing_algorithm: value.dynamic_routing_algorithm,
            is_network_tokenization_enabled: value.is_network_tokenization_enabled,
            is_auto_retries_enabled: value.is_auto_retries_enabled,
            max_auto_retries_enabled: value.max_auto_retries_enabled,
            always_request_extended_authorization: value.always_request_extended_authorization,
        }
    }
}

impl Profile {
    #[cfg(feature = "v1")]
    pub fn get_id(&self) -> &common_utils::id_type::ProfileId {
        &self.profile_id
    }

    #[cfg(feature = "v2")]
    pub fn get_id(&self) -> &common_utils::id_type::ProfileId {
        &self.id
    }
}

#[cfg(feature = "v1")]
#[derive(Debug)]
pub struct ProfileGeneralUpdate {
    pub profile_name: Option<String>,
    pub return_url: Option<String>,
    pub enable_payment_response_hash: Option<bool>,
    pub payment_response_hash_key: Option<String>,
    pub redirect_to_merchant_with_http_post: Option<bool>,
    pub webhook_details: Option<WebhookDetails>,
    pub metadata: Option<pii::SecretSerdeValue>,
    pub routing_algorithm: Option<serde_json::Value>,
    pub intent_fulfillment_time: Option<i64>,
    pub frm_routing_algorithm: Option<serde_json::Value>,
    pub payout_routing_algorithm: Option<serde_json::Value>,
    pub applepay_verified_domains: Option<Vec<String>>,
    pub payment_link_config: Option<BusinessPaymentLinkConfig>,
    pub session_expiry: Option<i64>,
    pub authentication_connector_details: Option<AuthenticationConnectorDetails>,
    pub payout_link_config: Option<BusinessPayoutLinkConfig>,
    pub extended_card_info_config: Option<pii::SecretSerdeValue>,
    pub use_billing_as_payment_method_billing: Option<bool>,
    pub collect_shipping_details_from_wallet_connector: Option<bool>,
    pub collect_billing_details_from_wallet_connector: Option<bool>,
    pub is_connector_agnostic_mit_enabled: Option<bool>,
    pub outgoing_webhook_custom_http_headers: OptionalEncryptableValue,
    pub always_collect_billing_details_from_wallet_connector: Option<bool>,
    pub always_collect_shipping_details_from_wallet_connector: Option<bool>,
    pub tax_connector_id: Option<common_utils::id_type::MerchantConnectorAccountId>,
    pub is_tax_connector_enabled: Option<bool>,
    pub dynamic_routing_algorithm: Option<serde_json::Value>,
    pub is_network_tokenization_enabled: Option<bool>,
    pub is_auto_retries_enabled: Option<bool>,
    pub max_auto_retries_enabled: Option<i16>,
}

#[cfg(feature = "v1")]
#[derive(Debug)]
pub enum ProfileUpdate {
    Update(Box<ProfileGeneralUpdate>),
    RoutingAlgorithmUpdate {
        routing_algorithm: Option<serde_json::Value>,
        payout_routing_algorithm: Option<serde_json::Value>,
    },
    DynamicRoutingAlgorithmUpdate {
        dynamic_routing_algorithm: Option<serde_json::Value>,
    },
    ExtendedCardInfoUpdate {
        is_extended_card_info_enabled: bool,
    },
    ConnectorAgnosticMitUpdate {
        is_connector_agnostic_mit_enabled: bool,
    },
    NetworkTokenizationUpdate {
        is_network_tokenization_enabled: bool,
    },
}

#[cfg(feature = "v1")]
impl From<ProfileUpdate> for ProfileUpdateInternal {
    fn from(profile_update: ProfileUpdate) -> Self {
        let now = date_time::now();

        match profile_update {
            ProfileUpdate::Update(update) => {
                let ProfileGeneralUpdate {
                    profile_name,
                    return_url,
                    enable_payment_response_hash,
                    payment_response_hash_key,
                    redirect_to_merchant_with_http_post,
                    webhook_details,
                    metadata,
                    routing_algorithm,
                    intent_fulfillment_time,
                    frm_routing_algorithm,
                    payout_routing_algorithm,
                    applepay_verified_domains,
                    payment_link_config,
                    session_expiry,
                    authentication_connector_details,
                    payout_link_config,
                    extended_card_info_config,
                    use_billing_as_payment_method_billing,
                    collect_shipping_details_from_wallet_connector,
                    collect_billing_details_from_wallet_connector,
                    is_connector_agnostic_mit_enabled,
                    outgoing_webhook_custom_http_headers,
                    always_collect_billing_details_from_wallet_connector,
                    always_collect_shipping_details_from_wallet_connector,
                    tax_connector_id,
                    is_tax_connector_enabled,
                    dynamic_routing_algorithm,
                    is_network_tokenization_enabled,
                    is_auto_retries_enabled,
                    max_auto_retries_enabled,
                } = *update;

                Self {
                    profile_name,
                    modified_at: now,
                    return_url,
                    enable_payment_response_hash,
                    payment_response_hash_key,
                    redirect_to_merchant_with_http_post,
                    webhook_details,
                    metadata,
                    routing_algorithm,
                    intent_fulfillment_time,
                    frm_routing_algorithm,
                    payout_routing_algorithm,
                    is_recon_enabled: None,
                    applepay_verified_domains,
                    payment_link_config,
                    session_expiry,
                    authentication_connector_details,
                    payout_link_config,
                    is_extended_card_info_enabled: None,
                    extended_card_info_config,
                    is_connector_agnostic_mit_enabled,
                    use_billing_as_payment_method_billing,
                    collect_shipping_details_from_wallet_connector,
                    collect_billing_details_from_wallet_connector,
                    outgoing_webhook_custom_http_headers: outgoing_webhook_custom_http_headers
                        .map(Encryption::from),
                    always_collect_billing_details_from_wallet_connector,
                    always_collect_shipping_details_from_wallet_connector,
                    tax_connector_id,
                    is_tax_connector_enabled,
                    dynamic_routing_algorithm,
                    is_network_tokenization_enabled,
                    is_auto_retries_enabled,
                    max_auto_retries_enabled,
                    always_request_extended_authorization: None,
                }
            }
            ProfileUpdate::RoutingAlgorithmUpdate {
                routing_algorithm,
                payout_routing_algorithm,
            } => Self {
                profile_name: None,
                modified_at: now,
                return_url: None,
                enable_payment_response_hash: None,
                payment_response_hash_key: None,
                redirect_to_merchant_with_http_post: None,
                webhook_details: None,
                metadata: None,
                routing_algorithm,
                intent_fulfillment_time: None,
                frm_routing_algorithm: None,
                payout_routing_algorithm,
                is_recon_enabled: None,
                applepay_verified_domains: None,
                payment_link_config: None,
                session_expiry: None,
                authentication_connector_details: None,
                payout_link_config: None,
                is_extended_card_info_enabled: None,
                extended_card_info_config: None,
                is_connector_agnostic_mit_enabled: None,
                use_billing_as_payment_method_billing: None,
                collect_shipping_details_from_wallet_connector: None,
                collect_billing_details_from_wallet_connector: None,
                outgoing_webhook_custom_http_headers: None,
                always_collect_billing_details_from_wallet_connector: None,
                always_collect_shipping_details_from_wallet_connector: None,
                tax_connector_id: None,
                is_tax_connector_enabled: None,
                dynamic_routing_algorithm: None,
                is_network_tokenization_enabled: None,
                is_auto_retries_enabled: None,
                max_auto_retries_enabled: None,
                always_request_extended_authorization: None,
            },
            ProfileUpdate::DynamicRoutingAlgorithmUpdate {
                dynamic_routing_algorithm,
            } => Self {
                profile_name: None,
                modified_at: now,
                return_url: None,
                enable_payment_response_hash: None,
                payment_response_hash_key: None,
                redirect_to_merchant_with_http_post: None,
                webhook_details: None,
                metadata: None,
                routing_algorithm: None,
                intent_fulfillment_time: None,
                frm_routing_algorithm: None,
                payout_routing_algorithm: None,
                is_recon_enabled: None,
                applepay_verified_domains: None,
                payment_link_config: None,
                session_expiry: None,
                authentication_connector_details: None,
                payout_link_config: None,
                is_extended_card_info_enabled: None,
                extended_card_info_config: None,
                is_connector_agnostic_mit_enabled: None,
                use_billing_as_payment_method_billing: None,
                collect_shipping_details_from_wallet_connector: None,
                collect_billing_details_from_wallet_connector: None,
                outgoing_webhook_custom_http_headers: None,
                always_collect_billing_details_from_wallet_connector: None,
                always_collect_shipping_details_from_wallet_connector: None,
                tax_connector_id: None,
                is_tax_connector_enabled: None,
                dynamic_routing_algorithm,
                is_network_tokenization_enabled: None,
                is_auto_retries_enabled: None,
                max_auto_retries_enabled: None,
                always_request_extended_authorization: None,
            },
            ProfileUpdate::ExtendedCardInfoUpdate {
                is_extended_card_info_enabled,
            } => Self {
                profile_name: None,
                modified_at: now,
                return_url: None,
                enable_payment_response_hash: None,
                payment_response_hash_key: None,
                redirect_to_merchant_with_http_post: None,
                webhook_details: None,
                metadata: None,
                routing_algorithm: None,
                intent_fulfillment_time: None,
                frm_routing_algorithm: None,
                payout_routing_algorithm: None,
                is_recon_enabled: None,
                applepay_verified_domains: None,
                payment_link_config: None,
                session_expiry: None,
                authentication_connector_details: None,
                payout_link_config: None,
                is_extended_card_info_enabled: Some(is_extended_card_info_enabled),
                extended_card_info_config: None,
                is_connector_agnostic_mit_enabled: None,
                use_billing_as_payment_method_billing: None,
                collect_shipping_details_from_wallet_connector: None,
                collect_billing_details_from_wallet_connector: None,
                outgoing_webhook_custom_http_headers: None,
                always_collect_billing_details_from_wallet_connector: None,
                always_collect_shipping_details_from_wallet_connector: None,
                tax_connector_id: None,
                is_tax_connector_enabled: None,
                dynamic_routing_algorithm: None,
                is_network_tokenization_enabled: None,
                is_auto_retries_enabled: None,
                max_auto_retries_enabled: None,
                always_request_extended_authorization: None,
            },
            ProfileUpdate::ConnectorAgnosticMitUpdate {
                is_connector_agnostic_mit_enabled,
            } => Self {
                profile_name: None,
                modified_at: now,
                return_url: None,
                enable_payment_response_hash: None,
                payment_response_hash_key: None,
                redirect_to_merchant_with_http_post: None,
                webhook_details: None,
                metadata: None,
                routing_algorithm: None,
                intent_fulfillment_time: None,
                frm_routing_algorithm: None,
                payout_routing_algorithm: None,
                is_recon_enabled: None,
                applepay_verified_domains: None,
                payment_link_config: None,
                session_expiry: None,
                authentication_connector_details: None,
                payout_link_config: None,
                is_extended_card_info_enabled: None,
                extended_card_info_config: None,
                is_connector_agnostic_mit_enabled: Some(is_connector_agnostic_mit_enabled),
                use_billing_as_payment_method_billing: None,
                collect_shipping_details_from_wallet_connector: None,
                collect_billing_details_from_wallet_connector: None,
                outgoing_webhook_custom_http_headers: None,
                always_collect_billing_details_from_wallet_connector: None,
                always_collect_shipping_details_from_wallet_connector: None,
                tax_connector_id: None,
                is_tax_connector_enabled: None,
                dynamic_routing_algorithm: None,
                is_network_tokenization_enabled: None,
                is_auto_retries_enabled: None,
                max_auto_retries_enabled: None,
                always_request_extended_authorization: None,
            },
            ProfileUpdate::NetworkTokenizationUpdate {
                is_network_tokenization_enabled,
            } => Self {
                profile_name: None,
                modified_at: now,
                return_url: None,
                enable_payment_response_hash: None,
                payment_response_hash_key: None,
                redirect_to_merchant_with_http_post: None,
                webhook_details: None,
                metadata: None,
                routing_algorithm: None,
                intent_fulfillment_time: None,
                frm_routing_algorithm: None,
                payout_routing_algorithm: None,
                is_recon_enabled: None,
                applepay_verified_domains: None,
                payment_link_config: None,
                session_expiry: None,
                authentication_connector_details: None,
                payout_link_config: None,
                is_extended_card_info_enabled: None,
                extended_card_info_config: None,
                is_connector_agnostic_mit_enabled: None,
                use_billing_as_payment_method_billing: None,
                collect_shipping_details_from_wallet_connector: None,
                collect_billing_details_from_wallet_connector: None,
                outgoing_webhook_custom_http_headers: None,
                always_collect_billing_details_from_wallet_connector: None,
                always_collect_shipping_details_from_wallet_connector: None,
                tax_connector_id: None,
                is_tax_connector_enabled: None,
                dynamic_routing_algorithm: None,
                is_network_tokenization_enabled: Some(is_network_tokenization_enabled),
                is_auto_retries_enabled: None,
                max_auto_retries_enabled: None,
                always_request_extended_authorization: None,
            },
        }
    }
}

#[cfg(feature = "v1")]
#[async_trait::async_trait]
impl super::behaviour::Conversion for Profile {
    type DstType = diesel_models::business_profile::Profile;
    type NewDstType = diesel_models::business_profile::ProfileNew;

    async fn convert(self) -> CustomResult<Self::DstType, ValidationError> {
        Ok(diesel_models::business_profile::Profile {
            profile_id: self.profile_id,
            merchant_id: self.merchant_id,
            profile_name: self.profile_name,
            created_at: self.created_at,
            modified_at: self.modified_at,
            return_url: self.return_url,
            enable_payment_response_hash: self.enable_payment_response_hash,
            payment_response_hash_key: self.payment_response_hash_key,
            redirect_to_merchant_with_http_post: self.redirect_to_merchant_with_http_post,
            webhook_details: self.webhook_details,
            metadata: self.metadata,
            routing_algorithm: self.routing_algorithm,
            intent_fulfillment_time: self.intent_fulfillment_time,
            frm_routing_algorithm: self.frm_routing_algorithm,
            payout_routing_algorithm: self.payout_routing_algorithm,
            is_recon_enabled: self.is_recon_enabled,
            applepay_verified_domains: self.applepay_verified_domains,
            payment_link_config: self.payment_link_config,
            session_expiry: self.session_expiry,
            authentication_connector_details: self.authentication_connector_details,
            payout_link_config: self.payout_link_config,
            is_extended_card_info_enabled: self.is_extended_card_info_enabled,
            extended_card_info_config: self.extended_card_info_config,
            is_connector_agnostic_mit_enabled: self.is_connector_agnostic_mit_enabled,
            use_billing_as_payment_method_billing: self.use_billing_as_payment_method_billing,
            collect_shipping_details_from_wallet_connector: self
                .collect_shipping_details_from_wallet_connector,
            collect_billing_details_from_wallet_connector: self
                .collect_billing_details_from_wallet_connector,
            outgoing_webhook_custom_http_headers: self
                .outgoing_webhook_custom_http_headers
                .map(Encryption::from),
            always_collect_billing_details_from_wallet_connector: self
                .always_collect_billing_details_from_wallet_connector,
            always_collect_shipping_details_from_wallet_connector: self
                .always_collect_shipping_details_from_wallet_connector,
            tax_connector_id: self.tax_connector_id,
            is_tax_connector_enabled: Some(self.is_tax_connector_enabled),
            version: self.version,
            dynamic_routing_algorithm: self.dynamic_routing_algorithm,
            is_network_tokenization_enabled: self.is_network_tokenization_enabled,
            is_auto_retries_enabled: Some(self.is_auto_retries_enabled),
            max_auto_retries_enabled: self.max_auto_retries_enabled,
            always_request_extended_authorization: self.always_request_extended_authorization,
        })
    }

    async fn convert_back(
        state: &keymanager::KeyManagerState,
        item: Self::DstType,
        key: &Secret<Vec<u8>>,
        key_manager_identifier: keymanager::Identifier,
    ) -> CustomResult<Self, ValidationError>
    where
        Self: Sized,
    {
        async {
            Ok::<Self, error_stack::Report<common_utils::errors::CryptoError>>(Self {
                profile_id: item.profile_id,
                merchant_id: item.merchant_id,
                profile_name: item.profile_name,
                created_at: item.created_at,
                modified_at: item.modified_at,
                return_url: item.return_url,
                enable_payment_response_hash: item.enable_payment_response_hash,
                payment_response_hash_key: item.payment_response_hash_key,
                redirect_to_merchant_with_http_post: item.redirect_to_merchant_with_http_post,
                webhook_details: item.webhook_details,
                metadata: item.metadata,
                routing_algorithm: item.routing_algorithm,
                intent_fulfillment_time: item.intent_fulfillment_time,
                frm_routing_algorithm: item.frm_routing_algorithm,
                payout_routing_algorithm: item.payout_routing_algorithm,
                is_recon_enabled: item.is_recon_enabled,
                applepay_verified_domains: item.applepay_verified_domains,
                payment_link_config: item.payment_link_config,
                session_expiry: item.session_expiry,
                authentication_connector_details: item.authentication_connector_details,
                payout_link_config: item.payout_link_config,
                is_extended_card_info_enabled: item.is_extended_card_info_enabled,
                extended_card_info_config: item.extended_card_info_config,
                is_connector_agnostic_mit_enabled: item.is_connector_agnostic_mit_enabled,
                use_billing_as_payment_method_billing: item.use_billing_as_payment_method_billing,
                collect_shipping_details_from_wallet_connector: item
                    .collect_shipping_details_from_wallet_connector,
                collect_billing_details_from_wallet_connector: item
                    .collect_billing_details_from_wallet_connector,
                always_collect_billing_details_from_wallet_connector: item
                    .always_collect_billing_details_from_wallet_connector,
                always_collect_shipping_details_from_wallet_connector: item
                    .always_collect_shipping_details_from_wallet_connector,
                outgoing_webhook_custom_http_headers: item
                    .outgoing_webhook_custom_http_headers
                    .async_lift(|inner| async {
                        crypto_operation(
                            state,
                            type_name!(Self::DstType),
                            CryptoOperation::DecryptOptional(inner),
                            key_manager_identifier.clone(),
                            key.peek(),
                        )
                        .await
                        .and_then(|val| val.try_into_optionaloperation())
                    })
                    .await?,
                tax_connector_id: item.tax_connector_id,
                is_tax_connector_enabled: item.is_tax_connector_enabled.unwrap_or(false),
                version: item.version,
                dynamic_routing_algorithm: item.dynamic_routing_algorithm,
                is_network_tokenization_enabled: item.is_network_tokenization_enabled,
                is_auto_retries_enabled: item.is_auto_retries_enabled.unwrap_or(false),
                max_auto_retries_enabled: item.max_auto_retries_enabled,
                always_request_extended_authorization: item.always_request_extended_authorization,
            })
        }
        .await
        .change_context(ValidationError::InvalidValue {
            message: "Failed while decrypting business profile data".to_string(),
        })
    }

    async fn construct_new(self) -> CustomResult<Self::NewDstType, ValidationError> {
        Ok(diesel_models::business_profile::ProfileNew {
            profile_id: self.profile_id,
            merchant_id: self.merchant_id,
            profile_name: self.profile_name,
            created_at: self.created_at,
            modified_at: self.modified_at,
            return_url: self.return_url,
            enable_payment_response_hash: self.enable_payment_response_hash,
            payment_response_hash_key: self.payment_response_hash_key,
            redirect_to_merchant_with_http_post: self.redirect_to_merchant_with_http_post,
            webhook_details: self.webhook_details,
            metadata: self.metadata,
            routing_algorithm: self.routing_algorithm,
            intent_fulfillment_time: self.intent_fulfillment_time,
            frm_routing_algorithm: self.frm_routing_algorithm,
            payout_routing_algorithm: self.payout_routing_algorithm,
            is_recon_enabled: self.is_recon_enabled,
            applepay_verified_domains: self.applepay_verified_domains,
            payment_link_config: self.payment_link_config,
            session_expiry: self.session_expiry,
            authentication_connector_details: self.authentication_connector_details,
            payout_link_config: self.payout_link_config,
            is_extended_card_info_enabled: self.is_extended_card_info_enabled,
            extended_card_info_config: self.extended_card_info_config,
            is_connector_agnostic_mit_enabled: self.is_connector_agnostic_mit_enabled,
            use_billing_as_payment_method_billing: self.use_billing_as_payment_method_billing,
            collect_shipping_details_from_wallet_connector: self
                .collect_shipping_details_from_wallet_connector,
            collect_billing_details_from_wallet_connector: self
                .collect_billing_details_from_wallet_connector,
            outgoing_webhook_custom_http_headers: self
                .outgoing_webhook_custom_http_headers
                .map(Encryption::from),
            always_collect_billing_details_from_wallet_connector: self
                .always_collect_billing_details_from_wallet_connector,
            always_collect_shipping_details_from_wallet_connector: self
                .always_collect_shipping_details_from_wallet_connector,
            tax_connector_id: self.tax_connector_id,
            is_tax_connector_enabled: Some(self.is_tax_connector_enabled),
            version: self.version,
            is_network_tokenization_enabled: self.is_network_tokenization_enabled,
            is_auto_retries_enabled: Some(self.is_auto_retries_enabled),
            max_auto_retries_enabled: self.max_auto_retries_enabled,
        })
    }
}

#[cfg(feature = "v2")]
#[derive(Clone, Debug)]
pub struct Profile {
    id: common_utils::id_type::ProfileId,
    pub merchant_id: common_utils::id_type::MerchantId,
    pub profile_name: String,
    pub created_at: time::PrimitiveDateTime,
    pub modified_at: time::PrimitiveDateTime,
    pub return_url: Option<common_utils::types::Url>,
    pub enable_payment_response_hash: bool,
    pub payment_response_hash_key: Option<String>,
    pub redirect_to_merchant_with_http_post: bool,
    pub webhook_details: Option<WebhookDetails>,
    pub metadata: Option<pii::SecretSerdeValue>,
    pub is_recon_enabled: bool,
    pub applepay_verified_domains: Option<Vec<String>>,
    pub payment_link_config: Option<BusinessPaymentLinkConfig>,
    pub session_expiry: Option<i64>,
    pub authentication_connector_details: Option<AuthenticationConnectorDetails>,
    pub payout_link_config: Option<BusinessPayoutLinkConfig>,
    pub is_extended_card_info_enabled: Option<bool>,
    pub extended_card_info_config: Option<pii::SecretSerdeValue>,
    pub is_connector_agnostic_mit_enabled: Option<bool>,
    pub use_billing_as_payment_method_billing: Option<bool>,
    pub collect_shipping_details_from_wallet_connector: Option<bool>,
    pub collect_billing_details_from_wallet_connector: Option<bool>,
    pub outgoing_webhook_custom_http_headers: OptionalEncryptableValue,
    pub always_collect_billing_details_from_wallet_connector: Option<bool>,
    pub always_collect_shipping_details_from_wallet_connector: Option<bool>,
    pub routing_algorithm_id: Option<common_utils::id_type::RoutingId>,
    pub order_fulfillment_time: Option<i64>,
    pub order_fulfillment_time_origin: Option<common_enums::OrderFulfillmentTimeOrigin>,
    pub frm_routing_algorithm_id: Option<String>,
    pub payout_routing_algorithm_id: Option<common_utils::id_type::RoutingId>,
    pub default_fallback_routing: Option<pii::SecretSerdeValue>,
    pub should_collect_cvv_during_payment: bool,
    pub tax_connector_id: Option<common_utils::id_type::MerchantConnectorAccountId>,
    pub is_tax_connector_enabled: bool,
    pub version: common_enums::ApiVersion,
    pub is_network_tokenization_enabled: bool,
}

#[cfg(feature = "v2")]
pub struct ProfileSetter {
    pub id: common_utils::id_type::ProfileId,
    pub merchant_id: common_utils::id_type::MerchantId,
    pub profile_name: String,
    pub created_at: time::PrimitiveDateTime,
    pub modified_at: time::PrimitiveDateTime,
    pub return_url: Option<common_utils::types::Url>,
    pub enable_payment_response_hash: bool,
    pub payment_response_hash_key: Option<String>,
    pub redirect_to_merchant_with_http_post: bool,
    pub webhook_details: Option<WebhookDetails>,
    pub metadata: Option<pii::SecretSerdeValue>,
    pub is_recon_enabled: bool,
    pub applepay_verified_domains: Option<Vec<String>>,
    pub payment_link_config: Option<BusinessPaymentLinkConfig>,
    pub session_expiry: Option<i64>,
    pub authentication_connector_details: Option<AuthenticationConnectorDetails>,
    pub payout_link_config: Option<BusinessPayoutLinkConfig>,
    pub is_extended_card_info_enabled: Option<bool>,
    pub extended_card_info_config: Option<pii::SecretSerdeValue>,
    pub is_connector_agnostic_mit_enabled: Option<bool>,
    pub use_billing_as_payment_method_billing: Option<bool>,
    pub collect_shipping_details_from_wallet_connector: Option<bool>,
    pub collect_billing_details_from_wallet_connector: Option<bool>,
    pub outgoing_webhook_custom_http_headers: OptionalEncryptableValue,
    pub always_collect_billing_details_from_wallet_connector: Option<bool>,
    pub always_collect_shipping_details_from_wallet_connector: Option<bool>,
    pub routing_algorithm_id: Option<common_utils::id_type::RoutingId>,
    pub order_fulfillment_time: Option<i64>,
    pub order_fulfillment_time_origin: Option<common_enums::OrderFulfillmentTimeOrigin>,
    pub frm_routing_algorithm_id: Option<String>,
    pub payout_routing_algorithm_id: Option<common_utils::id_type::RoutingId>,
    pub default_fallback_routing: Option<pii::SecretSerdeValue>,
    pub should_collect_cvv_during_payment: bool,
    pub tax_connector_id: Option<common_utils::id_type::MerchantConnectorAccountId>,
    pub is_tax_connector_enabled: bool,
    pub is_network_tokenization_enabled: bool,
}

#[cfg(feature = "v2")]
impl From<ProfileSetter> for Profile {
    fn from(value: ProfileSetter) -> Self {
        Self {
            id: value.id,
            merchant_id: value.merchant_id,
            profile_name: value.profile_name,
            created_at: value.created_at,
            modified_at: value.modified_at,
            return_url: value.return_url,
            enable_payment_response_hash: value.enable_payment_response_hash,
            payment_response_hash_key: value.payment_response_hash_key,
            redirect_to_merchant_with_http_post: value.redirect_to_merchant_with_http_post,
            webhook_details: value.webhook_details,
            metadata: value.metadata,
            is_recon_enabled: value.is_recon_enabled,
            applepay_verified_domains: value.applepay_verified_domains,
            payment_link_config: value.payment_link_config,
            session_expiry: value.session_expiry,
            authentication_connector_details: value.authentication_connector_details,
            payout_link_config: value.payout_link_config,
            is_extended_card_info_enabled: value.is_extended_card_info_enabled,
            extended_card_info_config: value.extended_card_info_config,
            is_connector_agnostic_mit_enabled: value.is_connector_agnostic_mit_enabled,
            use_billing_as_payment_method_billing: value.use_billing_as_payment_method_billing,
            collect_shipping_details_from_wallet_connector: value
                .collect_shipping_details_from_wallet_connector,
            collect_billing_details_from_wallet_connector: value
                .collect_billing_details_from_wallet_connector,
            outgoing_webhook_custom_http_headers: value.outgoing_webhook_custom_http_headers,
            always_collect_billing_details_from_wallet_connector: value
                .always_collect_billing_details_from_wallet_connector,
            always_collect_shipping_details_from_wallet_connector: value
                .always_collect_shipping_details_from_wallet_connector,
            routing_algorithm_id: value.routing_algorithm_id,
            order_fulfillment_time: value.order_fulfillment_time,
            order_fulfillment_time_origin: value.order_fulfillment_time_origin,
            frm_routing_algorithm_id: value.frm_routing_algorithm_id,
            payout_routing_algorithm_id: value.payout_routing_algorithm_id,
            default_fallback_routing: value.default_fallback_routing,
            should_collect_cvv_during_payment: value.should_collect_cvv_during_payment,
            tax_connector_id: value.tax_connector_id,
            is_tax_connector_enabled: value.is_tax_connector_enabled,
            version: consts::API_VERSION,
            is_network_tokenization_enabled: value.is_network_tokenization_enabled,
        }
    }
}

impl Profile {
    pub fn get_is_tax_connector_enabled(&self) -> bool {
        let is_tax_connector_enabled = self.is_tax_connector_enabled;
        match &self.tax_connector_id {
            Some(_id) => is_tax_connector_enabled,
            _ => false,
        }
    }

    #[cfg(feature = "v1")]
    pub fn get_order_fulfillment_time(&self) -> Option<i64> {
        self.intent_fulfillment_time
    }

    #[cfg(feature = "v2")]
    pub fn get_order_fulfillment_time(&self) -> Option<i64> {
        self.order_fulfillment_time
    }
}

#[cfg(feature = "v2")]
#[derive(Debug)]
pub struct ProfileGeneralUpdate {
    pub profile_name: Option<String>,
    pub return_url: Option<common_utils::types::Url>,
    pub enable_payment_response_hash: Option<bool>,
    pub payment_response_hash_key: Option<String>,
    pub redirect_to_merchant_with_http_post: Option<bool>,
    pub webhook_details: Option<WebhookDetails>,
    pub metadata: Option<pii::SecretSerdeValue>,
    pub applepay_verified_domains: Option<Vec<String>>,
    pub payment_link_config: Option<BusinessPaymentLinkConfig>,
    pub session_expiry: Option<i64>,
    pub authentication_connector_details: Option<AuthenticationConnectorDetails>,
    pub payout_link_config: Option<BusinessPayoutLinkConfig>,
    pub extended_card_info_config: Option<pii::SecretSerdeValue>,
    pub use_billing_as_payment_method_billing: Option<bool>,
    pub collect_shipping_details_from_wallet_connector: Option<bool>,
    pub collect_billing_details_from_wallet_connector: Option<bool>,
    pub is_connector_agnostic_mit_enabled: Option<bool>,
    pub outgoing_webhook_custom_http_headers: OptionalEncryptableValue,
    pub always_collect_billing_details_from_wallet_connector: Option<bool>,
    pub always_collect_shipping_details_from_wallet_connector: Option<bool>,
    pub order_fulfillment_time: Option<i64>,
    pub order_fulfillment_time_origin: Option<common_enums::OrderFulfillmentTimeOrigin>,
    pub is_network_tokenization_enabled: Option<bool>,
}

#[cfg(feature = "v2")]
#[derive(Debug)]
pub enum ProfileUpdate {
    Update(Box<ProfileGeneralUpdate>),
    RoutingAlgorithmUpdate {
        routing_algorithm_id: Option<common_utils::id_type::RoutingId>,
        payout_routing_algorithm_id: Option<common_utils::id_type::RoutingId>,
    },
    DefaultRoutingFallbackUpdate {
        default_fallback_routing: Option<pii::SecretSerdeValue>,
    },
    ExtendedCardInfoUpdate {
        is_extended_card_info_enabled: bool,
    },
    ConnectorAgnosticMitUpdate {
        is_connector_agnostic_mit_enabled: bool,
    },
    NetworkTokenizationUpdate {
        is_network_tokenization_enabled: bool,
    },
    CollectCvvDuringPaymentUpdate {
        should_collect_cvv_during_payment: bool,
    },
}

#[cfg(feature = "v2")]
impl From<ProfileUpdate> for ProfileUpdateInternal {
    fn from(profile_update: ProfileUpdate) -> Self {
        let now = date_time::now();

        match profile_update {
            ProfileUpdate::Update(update) => {
                let ProfileGeneralUpdate {
                    profile_name,
                    return_url,
                    enable_payment_response_hash,
                    payment_response_hash_key,
                    redirect_to_merchant_with_http_post,
                    webhook_details,
                    metadata,
                    applepay_verified_domains,
                    payment_link_config,
                    session_expiry,
                    authentication_connector_details,
                    payout_link_config,
                    extended_card_info_config,
                    use_billing_as_payment_method_billing,
                    collect_shipping_details_from_wallet_connector,
                    collect_billing_details_from_wallet_connector,
                    is_connector_agnostic_mit_enabled,
                    outgoing_webhook_custom_http_headers,
                    always_collect_billing_details_from_wallet_connector,
                    always_collect_shipping_details_from_wallet_connector,
                    order_fulfillment_time,
                    order_fulfillment_time_origin,
                    is_network_tokenization_enabled,
                } = *update;
                Self {
                    profile_name,
                    modified_at: now,
                    return_url,
                    enable_payment_response_hash,
                    payment_response_hash_key,
                    redirect_to_merchant_with_http_post,
                    webhook_details,
                    metadata,
                    is_recon_enabled: None,
                    applepay_verified_domains,
                    payment_link_config,
                    session_expiry,
                    authentication_connector_details,
                    payout_link_config,
                    is_extended_card_info_enabled: None,
                    extended_card_info_config,
                    is_connector_agnostic_mit_enabled,
                    use_billing_as_payment_method_billing,
                    collect_shipping_details_from_wallet_connector,
                    collect_billing_details_from_wallet_connector,
                    outgoing_webhook_custom_http_headers: outgoing_webhook_custom_http_headers
                        .map(Encryption::from),
                    routing_algorithm_id: None,
                    always_collect_billing_details_from_wallet_connector,
                    always_collect_shipping_details_from_wallet_connector,
                    order_fulfillment_time,
                    order_fulfillment_time_origin,
                    frm_routing_algorithm_id: None,
                    payout_routing_algorithm_id: None,
                    default_fallback_routing: None,
                    should_collect_cvv_during_payment: None,
                    tax_connector_id: None,
                    is_tax_connector_enabled: None,
                    is_network_tokenization_enabled,
                    is_auto_retries_enabled: None,
                    max_auto_retries_enabled: None,
                }
            }
            ProfileUpdate::RoutingAlgorithmUpdate {
                routing_algorithm_id,
                payout_routing_algorithm_id,
            } => Self {
                profile_name: None,
                modified_at: now,
                return_url: None,
                enable_payment_response_hash: None,
                payment_response_hash_key: None,
                redirect_to_merchant_with_http_post: None,
                webhook_details: None,
                metadata: None,
                is_recon_enabled: None,
                applepay_verified_domains: None,
                payment_link_config: None,
                session_expiry: None,
                authentication_connector_details: None,
                payout_link_config: None,
                is_extended_card_info_enabled: None,
                extended_card_info_config: None,
                is_connector_agnostic_mit_enabled: None,
                use_billing_as_payment_method_billing: None,
                collect_shipping_details_from_wallet_connector: None,
                collect_billing_details_from_wallet_connector: None,
                outgoing_webhook_custom_http_headers: None,
                routing_algorithm_id,
                always_collect_billing_details_from_wallet_connector: None,
                always_collect_shipping_details_from_wallet_connector: None,
                order_fulfillment_time: None,
                order_fulfillment_time_origin: None,
                frm_routing_algorithm_id: None,
                payout_routing_algorithm_id,
                default_fallback_routing: None,
                should_collect_cvv_during_payment: None,
                tax_connector_id: None,
                is_tax_connector_enabled: None,
                is_network_tokenization_enabled: None,
                is_auto_retries_enabled: None,
                max_auto_retries_enabled: None,
            },
            ProfileUpdate::ExtendedCardInfoUpdate {
                is_extended_card_info_enabled,
            } => Self {
                profile_name: None,
                modified_at: now,
                return_url: None,
                enable_payment_response_hash: None,
                payment_response_hash_key: None,
                redirect_to_merchant_with_http_post: None,
                webhook_details: None,
                metadata: None,
                is_recon_enabled: None,
                applepay_verified_domains: None,
                payment_link_config: None,
                session_expiry: None,
                authentication_connector_details: None,
                payout_link_config: None,
                is_extended_card_info_enabled: Some(is_extended_card_info_enabled),
                extended_card_info_config: None,
                is_connector_agnostic_mit_enabled: None,
                use_billing_as_payment_method_billing: None,
                collect_shipping_details_from_wallet_connector: None,
                collect_billing_details_from_wallet_connector: None,
                outgoing_webhook_custom_http_headers: None,
                always_collect_billing_details_from_wallet_connector: None,
                always_collect_shipping_details_from_wallet_connector: None,
                routing_algorithm_id: None,
                payout_routing_algorithm_id: None,
                order_fulfillment_time: None,
                order_fulfillment_time_origin: None,
                frm_routing_algorithm_id: None,
                default_fallback_routing: None,
                should_collect_cvv_during_payment: None,
                tax_connector_id: None,
                is_tax_connector_enabled: None,
                is_network_tokenization_enabled: None,
                is_auto_retries_enabled: None,
                max_auto_retries_enabled: None,
            },
            ProfileUpdate::ConnectorAgnosticMitUpdate {
                is_connector_agnostic_mit_enabled,
            } => Self {
                profile_name: None,
                modified_at: now,
                return_url: None,
                enable_payment_response_hash: None,
                payment_response_hash_key: None,
                redirect_to_merchant_with_http_post: None,
                webhook_details: None,
                metadata: None,
                is_recon_enabled: None,
                applepay_verified_domains: None,
                payment_link_config: None,
                session_expiry: None,
                authentication_connector_details: None,
                payout_link_config: None,
                is_extended_card_info_enabled: None,
                extended_card_info_config: None,
                is_connector_agnostic_mit_enabled: Some(is_connector_agnostic_mit_enabled),
                use_billing_as_payment_method_billing: None,
                collect_shipping_details_from_wallet_connector: None,
                collect_billing_details_from_wallet_connector: None,
                outgoing_webhook_custom_http_headers: None,
                always_collect_billing_details_from_wallet_connector: None,
                always_collect_shipping_details_from_wallet_connector: None,
                routing_algorithm_id: None,
                payout_routing_algorithm_id: None,
                order_fulfillment_time: None,
                order_fulfillment_time_origin: None,
                frm_routing_algorithm_id: None,
                default_fallback_routing: None,
                should_collect_cvv_during_payment: None,
                tax_connector_id: None,
                is_tax_connector_enabled: None,
                is_network_tokenization_enabled: None,
                is_auto_retries_enabled: None,
                max_auto_retries_enabled: None,
            },
            ProfileUpdate::DefaultRoutingFallbackUpdate {
                default_fallback_routing,
            } => Self {
                profile_name: None,
                modified_at: now,
                return_url: None,
                enable_payment_response_hash: None,
                payment_response_hash_key: None,
                redirect_to_merchant_with_http_post: None,
                webhook_details: None,
                metadata: None,
                is_recon_enabled: None,
                applepay_verified_domains: None,
                payment_link_config: None,
                session_expiry: None,
                authentication_connector_details: None,
                payout_link_config: None,
                is_extended_card_info_enabled: None,
                extended_card_info_config: None,
                is_connector_agnostic_mit_enabled: None,
                use_billing_as_payment_method_billing: None,
                collect_shipping_details_from_wallet_connector: None,
                collect_billing_details_from_wallet_connector: None,
                outgoing_webhook_custom_http_headers: None,
                always_collect_billing_details_from_wallet_connector: None,
                always_collect_shipping_details_from_wallet_connector: None,
                routing_algorithm_id: None,
                payout_routing_algorithm_id: None,
                order_fulfillment_time: None,
                order_fulfillment_time_origin: None,
                frm_routing_algorithm_id: None,
                default_fallback_routing,
                should_collect_cvv_during_payment: None,
                tax_connector_id: None,
                is_tax_connector_enabled: None,
                is_network_tokenization_enabled: None,
                is_auto_retries_enabled: None,
                max_auto_retries_enabled: None,
            },
            ProfileUpdate::NetworkTokenizationUpdate {
                is_network_tokenization_enabled,
            } => Self {
                profile_name: None,
                modified_at: now,
                return_url: None,
                enable_payment_response_hash: None,
                payment_response_hash_key: None,
                redirect_to_merchant_with_http_post: None,
                webhook_details: None,
                metadata: None,
                is_recon_enabled: None,
                applepay_verified_domains: None,
                payment_link_config: None,
                session_expiry: None,
                authentication_connector_details: None,
                payout_link_config: None,
                is_extended_card_info_enabled: None,
                extended_card_info_config: None,
                is_connector_agnostic_mit_enabled: None,
                use_billing_as_payment_method_billing: None,
                collect_shipping_details_from_wallet_connector: None,
                collect_billing_details_from_wallet_connector: None,
                outgoing_webhook_custom_http_headers: None,
                always_collect_billing_details_from_wallet_connector: None,
                always_collect_shipping_details_from_wallet_connector: None,
                routing_algorithm_id: None,
                payout_routing_algorithm_id: None,
                order_fulfillment_time: None,
                order_fulfillment_time_origin: None,
                frm_routing_algorithm_id: None,
                default_fallback_routing: None,
                should_collect_cvv_during_payment: None,
                tax_connector_id: None,
                is_tax_connector_enabled: None,
                is_network_tokenization_enabled: Some(is_network_tokenization_enabled),
                is_auto_retries_enabled: None,
                max_auto_retries_enabled: None,
            },
            ProfileUpdate::CollectCvvDuringPaymentUpdate {
                should_collect_cvv_during_payment,
            } => Self {
                profile_name: None,
                modified_at: now,
                return_url: None,
                enable_payment_response_hash: None,
                payment_response_hash_key: None,
                redirect_to_merchant_with_http_post: None,
                webhook_details: None,
                metadata: None,
                is_recon_enabled: None,
                applepay_verified_domains: None,
                payment_link_config: None,
                session_expiry: None,
                authentication_connector_details: None,
                payout_link_config: None,
                is_extended_card_info_enabled: None,
                extended_card_info_config: None,
                is_connector_agnostic_mit_enabled: None,
                use_billing_as_payment_method_billing: None,
                collect_shipping_details_from_wallet_connector: None,
                collect_billing_details_from_wallet_connector: None,
                outgoing_webhook_custom_http_headers: None,
                always_collect_billing_details_from_wallet_connector: None,
                always_collect_shipping_details_from_wallet_connector: None,
                routing_algorithm_id: None,
                payout_routing_algorithm_id: None,
                order_fulfillment_time: None,
                order_fulfillment_time_origin: None,
                frm_routing_algorithm_id: None,
                default_fallback_routing: None,
                should_collect_cvv_during_payment: Some(should_collect_cvv_during_payment),
                tax_connector_id: None,
                is_tax_connector_enabled: None,
                is_network_tokenization_enabled: None,
                is_auto_retries_enabled: None,
                max_auto_retries_enabled: None,
            },
        }
    }
}

#[cfg(feature = "v2")]
#[async_trait::async_trait]
impl super::behaviour::Conversion for Profile {
    type DstType = diesel_models::business_profile::Profile;
    type NewDstType = diesel_models::business_profile::ProfileNew;

    async fn convert(self) -> CustomResult<Self::DstType, ValidationError> {
        Ok(diesel_models::business_profile::Profile {
            id: self.id,
            merchant_id: self.merchant_id,
            profile_name: self.profile_name,
            created_at: self.created_at,
            modified_at: self.modified_at,
            return_url: self.return_url,
            enable_payment_response_hash: self.enable_payment_response_hash,
            payment_response_hash_key: self.payment_response_hash_key,
            redirect_to_merchant_with_http_post: self.redirect_to_merchant_with_http_post,
            webhook_details: self.webhook_details,
            metadata: self.metadata,
            is_recon_enabled: self.is_recon_enabled,
            applepay_verified_domains: self.applepay_verified_domains,
            payment_link_config: self.payment_link_config,
            session_expiry: self.session_expiry,
            authentication_connector_details: self.authentication_connector_details,
            payout_link_config: self.payout_link_config,
            is_extended_card_info_enabled: self.is_extended_card_info_enabled,
            extended_card_info_config: self.extended_card_info_config,
            is_connector_agnostic_mit_enabled: self.is_connector_agnostic_mit_enabled,
            use_billing_as_payment_method_billing: self.use_billing_as_payment_method_billing,
            collect_shipping_details_from_wallet_connector: self
                .collect_shipping_details_from_wallet_connector,
            collect_billing_details_from_wallet_connector: self
                .collect_billing_details_from_wallet_connector,
            outgoing_webhook_custom_http_headers: self
                .outgoing_webhook_custom_http_headers
                .map(Encryption::from),
            routing_algorithm_id: self.routing_algorithm_id,
            always_collect_billing_details_from_wallet_connector: self
                .always_collect_billing_details_from_wallet_connector,
            always_collect_shipping_details_from_wallet_connector: self
                .always_collect_shipping_details_from_wallet_connector,
            payout_routing_algorithm_id: self.payout_routing_algorithm_id,
            order_fulfillment_time: self.order_fulfillment_time,
            order_fulfillment_time_origin: self.order_fulfillment_time_origin,
            frm_routing_algorithm_id: self.frm_routing_algorithm_id,
            default_fallback_routing: self.default_fallback_routing,
            should_collect_cvv_during_payment: self.should_collect_cvv_during_payment,
            tax_connector_id: self.tax_connector_id,
            is_tax_connector_enabled: Some(self.is_tax_connector_enabled),
            version: self.version,
            dynamic_routing_algorithm: None,
            is_network_tokenization_enabled: self.is_network_tokenization_enabled,
            is_auto_retries_enabled: None,
            max_auto_retries_enabled: None,
        })
    }

    async fn convert_back(
        state: &keymanager::KeyManagerState,
        item: Self::DstType,
        key: &Secret<Vec<u8>>,
        key_manager_identifier: keymanager::Identifier,
    ) -> CustomResult<Self, ValidationError>
    where
        Self: Sized,
    {
        async {
            Ok::<Self, error_stack::Report<common_utils::errors::CryptoError>>(Self {
                id: item.id,
                merchant_id: item.merchant_id,
                profile_name: item.profile_name,
                created_at: item.created_at,
                modified_at: item.modified_at,
                return_url: item.return_url,
                enable_payment_response_hash: item.enable_payment_response_hash,
                payment_response_hash_key: item.payment_response_hash_key,
                redirect_to_merchant_with_http_post: item.redirect_to_merchant_with_http_post,
                webhook_details: item.webhook_details,
                metadata: item.metadata,
                is_recon_enabled: item.is_recon_enabled,
                applepay_verified_domains: item.applepay_verified_domains,
                payment_link_config: item.payment_link_config,
                session_expiry: item.session_expiry,
                authentication_connector_details: item.authentication_connector_details,
                payout_link_config: item.payout_link_config,
                is_extended_card_info_enabled: item.is_extended_card_info_enabled,
                extended_card_info_config: item.extended_card_info_config,
                is_connector_agnostic_mit_enabled: item.is_connector_agnostic_mit_enabled,
                use_billing_as_payment_method_billing: item.use_billing_as_payment_method_billing,
                collect_shipping_details_from_wallet_connector: item
                    .collect_shipping_details_from_wallet_connector,
                collect_billing_details_from_wallet_connector: item
                    .collect_billing_details_from_wallet_connector,
                outgoing_webhook_custom_http_headers: item
                    .outgoing_webhook_custom_http_headers
                    .async_lift(|inner| async {
                        crypto_operation(
                            state,
                            type_name!(Self::DstType),
                            CryptoOperation::DecryptOptional(inner),
                            key_manager_identifier.clone(),
                            key.peek(),
                        )
                        .await
                        .and_then(|val| val.try_into_optionaloperation())
                    })
                    .await?,
                routing_algorithm_id: item.routing_algorithm_id,
                always_collect_billing_details_from_wallet_connector: item
                    .always_collect_billing_details_from_wallet_connector,
                always_collect_shipping_details_from_wallet_connector: item
                    .always_collect_shipping_details_from_wallet_connector,
                order_fulfillment_time: item.order_fulfillment_time,
                order_fulfillment_time_origin: item.order_fulfillment_time_origin,
                frm_routing_algorithm_id: item.frm_routing_algorithm_id,
                payout_routing_algorithm_id: item.payout_routing_algorithm_id,
                default_fallback_routing: item.default_fallback_routing,
                should_collect_cvv_during_payment: item.should_collect_cvv_during_payment,
                tax_connector_id: item.tax_connector_id,
                is_tax_connector_enabled: item.is_tax_connector_enabled.unwrap_or(false),
                version: item.version,
                is_network_tokenization_enabled: item.is_network_tokenization_enabled,
            })
        }
        .await
        .change_context(ValidationError::InvalidValue {
            message: "Failed while decrypting business profile data".to_string(),
        })
    }

    async fn construct_new(self) -> CustomResult<Self::NewDstType, ValidationError> {
        Ok(diesel_models::business_profile::ProfileNew {
            id: self.id,
            merchant_id: self.merchant_id,
            profile_name: self.profile_name,
            created_at: self.created_at,
            modified_at: self.modified_at,
            return_url: self.return_url,
            enable_payment_response_hash: self.enable_payment_response_hash,
            payment_response_hash_key: self.payment_response_hash_key,
            redirect_to_merchant_with_http_post: self.redirect_to_merchant_with_http_post,
            webhook_details: self.webhook_details,
            metadata: self.metadata,
            is_recon_enabled: self.is_recon_enabled,
            applepay_verified_domains: self.applepay_verified_domains,
            payment_link_config: self.payment_link_config,
            session_expiry: self.session_expiry,
            authentication_connector_details: self.authentication_connector_details,
            payout_link_config: self.payout_link_config,
            is_extended_card_info_enabled: self.is_extended_card_info_enabled,
            extended_card_info_config: self.extended_card_info_config,
            is_connector_agnostic_mit_enabled: self.is_connector_agnostic_mit_enabled,
            use_billing_as_payment_method_billing: self.use_billing_as_payment_method_billing,
            collect_shipping_details_from_wallet_connector: self
                .collect_shipping_details_from_wallet_connector,
            collect_billing_details_from_wallet_connector: self
                .collect_billing_details_from_wallet_connector,
            outgoing_webhook_custom_http_headers: self
                .outgoing_webhook_custom_http_headers
                .map(Encryption::from),
            routing_algorithm_id: self.routing_algorithm_id,
            always_collect_billing_details_from_wallet_connector: self
                .always_collect_billing_details_from_wallet_connector,
            always_collect_shipping_details_from_wallet_connector: self
                .always_collect_shipping_details_from_wallet_connector,
            order_fulfillment_time: self.order_fulfillment_time,
            order_fulfillment_time_origin: self.order_fulfillment_time_origin,
            frm_routing_algorithm_id: self.frm_routing_algorithm_id,
            payout_routing_algorithm_id: self.payout_routing_algorithm_id,
            default_fallback_routing: self.default_fallback_routing,
            should_collect_cvv_during_payment: self.should_collect_cvv_during_payment,
            tax_connector_id: self.tax_connector_id,
            is_tax_connector_enabled: Some(self.is_tax_connector_enabled),
            version: self.version,
            is_network_tokenization_enabled: self.is_network_tokenization_enabled,
            is_auto_retries_enabled: None,
            max_auto_retries_enabled: None,
        })
    }
}
