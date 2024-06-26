use error_stack::ResultExt;
use router_env::env;

use super::VerifyConnector;
use crate::{
    connector, core::errors, services, types,
    types::api::verify_connector::BoxedConnectorIntegrationInterface,
};

#[async_trait::async_trait]
impl VerifyConnector for connector::Stripe {
    async fn handle_payment_error_response<F, ResourceCommonData, Req, Resp>(
        connector: BoxedConnectorIntegrationInterface<F, ResourceCommonData, Req, Resp>,
        error_response: types::Response,
    ) -> errors::RouterResponse<()> {
        let error = connector
            .get_error_response(error_response, None)
            .change_context(errors::ApiErrorResponse::InternalServerError)?;
        match (env::which(), error.code.as_str()) {
            // In situations where an attempt is made to process a payment using a
            // Stripe production key along with a test card (which verify_connector is using),
            // Stripe will respond with a "card_declined" error. In production,
            // when this scenario occurs we will send back an "Ok" response.
            (env::Env::Production, "card_declined") => Ok(services::ApplicationResponse::StatusOk),
            _ => Err(errors::ApiErrorResponse::InvalidRequestData {
                message: error.reason.unwrap_or(error.message),
            }
            .into()),
        }
    }
}
