use crate::{app::PaymentDto};
use crate::domain::Payment;
use crate::{SharedCosmosRepository};
use common::data::Repository;
use mediator::{AsyncRequestHandler, Request};

pub struct GetAllPaymentsRequest{
}

impl Request<Vec<PaymentDto>> for GetAllPaymentsRequest {}

pub struct GetAllPaymentsRequestHandler(pub SharedCosmosRepository<Payment>);

#[mediator::async_trait]
impl AsyncRequestHandler<GetAllPaymentsRequest, Vec<PaymentDto>> for GetAllPaymentsRequestHandler {

    async fn handle(&mut self, _req: GetAllPaymentsRequest) -> Vec<PaymentDto> {
        let lock = self.0.lock().await;
        let payments = lock.find_all().await.expect("no customer found");
        let mut payment_dtos: Vec<PaymentDto> = Vec::new();
        for payment in payments {
          payment_dtos.push(payment.into())
        }
        payment_dtos
    }
}