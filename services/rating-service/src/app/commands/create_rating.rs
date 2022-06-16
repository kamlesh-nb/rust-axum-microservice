use crate::{SharedCosmosRepository, app::RatingDto, domain::Rating};
use common::data::Repository;
use mediator::{AsyncRequestHandler, DefaultAsyncMediator, Request};
use serde::{Deserialize, Serialize};
 

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateRatingCommand {
  pub rating: RatingDto,
}

impl Request<RatingDto> for CreateRatingCommand {}

pub struct CreateRatingCommandHandler(pub SharedCosmosRepository<Rating>, pub DefaultAsyncMediator);

#[mediator::async_trait]
impl AsyncRequestHandler<CreateRatingCommand, RatingDto> for CreateRatingCommandHandler {
    async fn handle(&mut self, command: CreateRatingCommand) -> RatingDto {
      let lock = self.0.lock().await;
      let rating = lock.create(command.rating.into()).await.expect("Failed to create customer");
      rating.into()  
    }
}