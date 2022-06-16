use crate::app::*;
use crate::routes::*;
use utoipa::OpenApi;

pub fn create_api_doc() -> utoipa::openapi::OpenApi {
    #[derive(OpenApi)]
    #[openapi(
    handlers(
        get_all_payments,
        get_payment_by_id,
        create_payment,
        update_payment,
        delete_payment
    ),
    components(PaymentDto, PaymentDtoError),
    tags(
        (name = "Payments.API", description = "Payment Management API")
    )
  )]
    struct ApiDoc;
    ApiDoc::openapi()
}
