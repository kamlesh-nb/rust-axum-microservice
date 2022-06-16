use crate::{app::RatingDto};
use crate::domain::Rating;
use crate::{SharedCosmosRepository};
use common::data::Repository;
use mediator::{AsyncRequestHandler, Request};

pub struct GetAllRatingsRequest{
}

impl Request<Vec<RatingDto>> for GetAllRatingsRequest {}

pub struct GetAllRatingsRequestHandler(pub SharedCosmosRepository<Rating>);

#[mediator::async_trait]
impl AsyncRequestHandler<GetAllRatingsRequest, Vec<RatingDto>> for GetAllRatingsRequestHandler {

    async fn handle(&mut self, _req: GetAllRatingsRequest) -> Vec<RatingDto> {
        let lock = self.0.lock().await;
        let ratings = lock.find_all().await.expect("no customer found");
        let mut rating_dtos: Vec<RatingDto> = Vec::new();
        for rating in ratings {
          rating_dtos.push(rating.into())
        }
        rating_dtos
    }
}