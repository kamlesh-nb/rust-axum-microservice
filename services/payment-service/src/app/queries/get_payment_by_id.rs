use crate::domain::Payment;
use crate::{app::PaymentDto};
use crate::infra::SharedCosmosRepository;
use common::data::Repository;
use mediator::{AsyncRequestHandler, Request};

pub struct GetPaymentByIdRequest{
  pub id: String
}

impl Request<Vec<PaymentDto>> for GetPaymentByIdRequest {}

pub struct GetPaymentByIdRequestHandler(pub SharedCosmosRepository<Payment>);

#[mediator::async_trait]
impl AsyncRequestHandler<GetPaymentByIdRequest, Vec<PaymentDto>> for GetPaymentByIdRequestHandler {

    async fn handle(&mut self, req: GetPaymentByIdRequest) -> Vec<PaymentDto> {
        let lock = self.0.lock().await;
        let payments = lock.find_by_id(req.id).await.expect("no customer found");
        let mut payment_dtos: Vec<PaymentDto> = Vec::new();
        for payment in payments {
          payment_dtos.push(payment.into())
        }
        payment_dtos
    }
}