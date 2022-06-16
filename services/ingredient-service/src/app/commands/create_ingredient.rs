use crate::{SharedCosmosRepository, app::IngredientCategoryDto, domain::IngredientCategory};
use common::data::Repository;
use mediator::{AsyncRequestHandler, DefaultAsyncMediator, Request};
use serde::{Deserialize, Serialize};
 

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateIngredientCategoryCommand {
  pub ingredient_category: IngredientCategoryDto,
}

impl Request<IngredientCategoryDto> for CreateIngredientCategoryCommand {}

pub struct CreateIngredientCategoryCommandHandler(pub SharedCosmosRepository<IngredientCategory>, pub DefaultAsyncMediator);

#[mediator::async_trait]
impl AsyncRequestHandler<CreateIngredientCategoryCommand, IngredientCategoryDto> for CreateIngredientCategoryCommandHandler {
    async fn handle(&mut self, command: CreateIngredientCategoryCommand) -> IngredientCategoryDto {
      let lock = self.0.lock().await;
      let ingredient_category = lock.create(command.ingredient_category.into()).await.expect("Failed to create Ingredient Category");
      ingredient_category.into()  
    }
}