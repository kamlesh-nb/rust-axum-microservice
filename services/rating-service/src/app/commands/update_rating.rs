use crate::{SharedCosmosRepository, app::RatingDto, domain::Rating};
use common::data::Repository;
use mediator::{AsyncRequestHandler, DefaultAsyncMediator, Request};
use serde::{Deserialize, Serialize};
 

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateRatingCommand {
  pub id: String,
  pub rating: RatingDto,
}

impl Request<RatingDto> for UpdateRatingCommand {}

pub struct UpdateRatingCommandHandler(pub SharedCosmosRepository<Rating>, pub DefaultAsyncMediator);

#[mediator::async_trait]
impl AsyncRequestHandler<UpdateRatingCommand, RatingDto> for UpdateRatingCommandHandler {
    async fn handle(&mut self, command: UpdateRatingCommand) -> RatingDto {
      let lock = self.0.lock().await;
      let rating = lock.update(command.rating.into(), command.id.clone()).await.expect("Failed to update customer");
      rating.into()
    }
}