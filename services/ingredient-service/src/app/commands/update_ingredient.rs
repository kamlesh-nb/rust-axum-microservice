use crate::{SharedCosmosRepository, app::IngredientCategoryDto, domain::IngredientCategory};
use common::data::Repository;
use mediator::{AsyncRequestHandler, DefaultAsyncMediator, Request};
use serde::{Deserialize, Serialize};
 

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateIngredientCategoryCommand {
  pub id: String,
  pub ingredient_category: IngredientCategoryDto,
}

impl Request<IngredientCategoryDto> for UpdateIngredientCategoryCommand {}

pub struct UpdateIngredientCategoryCommandHandler(pub SharedCosmosRepository<IngredientCategory>, pub DefaultAsyncMediator);

#[mediator::async_trait]
impl AsyncRequestHandler<UpdateIngredientCategoryCommand, IngredientCategoryDto> for UpdateIngredientCategoryCommandHandler {
    async fn handle(&mut self, command: UpdateIngredientCategoryCommand) -> IngredientCategoryDto {
      let lock = self.0.lock().await;
      let ingredient_category = lock.update(command.ingredient_category.into(), command.id.clone()).await.expect("Failed to update Ingredient Category");
      ingredient_category.into()
    }
}