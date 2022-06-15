use crate::app::*;
use crate::routes::*;
use utoipa::OpenApi;

pub fn create_api_doc() -> utoipa::openapi::OpenApi {
    #[derive(OpenApi)]
    #[openapi(
    handlers(
        get_all_menus,
        get_menu_by_id,
        create_menu,
        update_menu,
        delete_menu
    ),
    components(MenuCategoryDto, MenuItemDto, IngredientDto, MenuCategoryDtoError),
    tags(
        (name = "MenuCategorys.API", description = "MenuCategory Management API")
    )
  )]
    struct ApiDoc;
    ApiDoc::openapi()
}
