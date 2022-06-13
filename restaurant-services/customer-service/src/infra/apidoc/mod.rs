use utoipa::OpenApi;
use crate::app::*;
use crate::routes::*;



pub fn create_api_doc() -> utoipa::openapi::OpenApi {
  #[derive(OpenApi)]
  #[openapi(
        handlers(
            get_all_customers,
            get_customer_by_id,
            create_customer,
            update_customer,
            delete_customer
        ),
        components(CustomerDto, CustomerDtoError),
        tags(
            (name = "Customers.API", description = "Customer Management API")
        )
      )]
  struct ApiDoc;
  ApiDoc::openapi()
}
