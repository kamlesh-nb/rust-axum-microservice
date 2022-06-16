use crate::app::*;
use crate::routes::*;
use utoipa::OpenApi;



pub fn create_api_doc() -> utoipa::openapi::OpenApi {
  #[derive(OpenApi)]
  #[openapi(
        handlers(
            get_all_ingredient_categories,
            get_ingredient_category_by_id,
            create_ingredient_category,
            update_ingredient_category,
            delete_ingredient_category
        ),
        components(IngredientCategoryDto, IngredientDto, IngredientError),
        tags(
            (name = "IngredientCategories.API", description = "IngredientCategory Management API")
        )
      )]
  struct ApiDoc;
  ApiDoc::openapi()
}
