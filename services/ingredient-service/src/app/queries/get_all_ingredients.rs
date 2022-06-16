use crate::{app::IngredientCategoryDto};
use crate::domain::IngredientCategory;
use crate::{SharedCosmosRepository};
use common::data::Repository;
use mediator::{AsyncRequestHandler, Request};

pub struct GetAllIngredientCategorysRequest{
}

impl Request<Vec<IngredientCategoryDto>> for GetAllIngredientCategorysRequest {}

pub struct GetAllIngredientCategorysRequestHandler(pub SharedCosmosRepository<IngredientCategory>);

#[mediator::async_trait]
impl AsyncRequestHandler<GetAllIngredientCategorysRequest, Vec<IngredientCategoryDto>> for GetAllIngredientCategorysRequestHandler {

    async fn handle(&mut self, _req: GetAllIngredientCategorysRequest) -> Vec<IngredientCategoryDto> {
        let lock = self.0.lock().await;
        let ingredient_categories = lock.find_all().await.expect("no Ingredient Category found");
        let mut ingredient_category_dtos: Vec<IngredientCategoryDto> = Vec::new();
        for ingredient_category in ingredient_categories {
          ingredient_category_dtos.push(ingredient_category.into())
        }
        ingredient_category_dtos
    }
}