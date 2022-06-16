use crate::app::*;
use crate::routes::*;
use utoipa::OpenApi;

pub fn create_api_doc() -> utoipa::openapi::OpenApi {
    #[derive(OpenApi)]
    #[openapi(
    handlers(
        get_all_ratings,
        get_rating_by_id,
        create_rating,
        update_rating,
        delete_rating
    ),
    components(RatingDto, RatingDtoError),
    tags(
        (name = "Ratings.API", description = "Rating Management API")
    )
  )]
    struct ApiDoc;
    ApiDoc::openapi()
}
