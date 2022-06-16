use crate::domain::IngredientCategory;
use crate::{app::IngredientCategoryDto};
use crate::infra::SharedCosmosRepository;
use common::data::Repository;
use mediator::{AsyncRequestHandler, Request};

pub struct GetIngredientCategoryByIdRequest{
  pub id: String
}

impl Request<Vec<IngredientCategoryDto>> for GetIngredientCategoryByIdRequest {}

pub struct GetIngredientCategoryByIdRequestHandler(pub SharedCosmosRepository<IngredientCategory>);

#[mediator::async_trait]
impl AsyncRequestHandler<GetIngredientCategoryByIdRequest, Vec<IngredientCategoryDto>> for GetIngredientCategoryByIdRequestHandler {

    async fn handle(&mut self, req: GetIngredientCategoryByIdRequest) -> Vec<IngredientCategoryDto> {
        let lock = self.0.lock().await;
        let ingredient_categories = lock.find_by_id(req.id).await.expect("no Ingredient Category found");
        let mut ingredient_category_dtos: Vec<IngredientCategoryDto> = Vec::new();
        for ingredient_category in ingredient_categories {
          ingredient_category_dtos.push(ingredient_category.into())
        }
        ingredient_category_dtos
    }
}