use utoipa::OpenApi;
use crate::app::*;
use crate::routes::*;



pub fn create_api_doc() -> utoipa::openapi::OpenApi {
  #[derive(OpenApi)]
  #[openapi(
        handlers(
            get_all_orders,
            get_order_by_id,
            create_order,
            update_order,
            delete_order
        ),
        components(Order, OrderDetail, OrderError),
        tags(
            (name = "Orders.API", description = "Order Management API")
        )
      )]
  struct ApiDoc;
  ApiDoc::openapi()
}
