use crate::domain::Rating;
use crate::{app::RatingDto};
use crate::infra::SharedCosmosRepository;
use common::data::Repository;
use mediator::{AsyncRequestHandler, Request};

pub struct GetRatingByIdRequest{
  pub id: String
}

impl Request<Vec<RatingDto>> for GetRatingByIdRequest {}

pub struct GetRatingByIdRequestHandler(pub SharedCosmosRepository<Rating>);

#[mediator::async_trait]
impl AsyncRequestHandler<GetRatingByIdRequest, Vec<RatingDto>> for GetRatingByIdRequestHandler {

    async fn handle(&mut self, req: GetRatingByIdRequest) -> Vec<RatingDto> {
        let lock = self.0.lock().await;
        let ratings = lock.find_by_id(req.id).await.expect("no customer found");
        let mut rating_dtos: Vec<RatingDto> = Vec::new();
        for rating in ratings {
          rating_dtos.push(rating.into())
        }
        rating_dtos
    }
}