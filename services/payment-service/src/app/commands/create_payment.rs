use crate::{SharedCosmosRepository, app::PaymentDto, domain::Payment};
use common::data::Repository;
use mediator::{AsyncRequestHandler, DefaultAsyncMediator, Request};
use serde::{Deserialize, Serialize};
 

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreatePaymentCommand {
  pub payment: PaymentDto,
}

impl Request<PaymentDto> for CreatePaymentCommand {}

pub struct CreatePaymentCommandHandler(pub SharedCosmosRepository<Payment>, pub DefaultAsyncMediator);

#[mediator::async_trait]
impl AsyncRequestHandler<CreatePaymentCommand, PaymentDto> for CreatePaymentCommandHandler {
    async fn handle(&mut self, command: CreatePaymentCommand) -> PaymentDto {
      let lock = self.0.lock().await;
      let payment = lock.create(command.payment.into()).await.expect("Failed to create customer");
      payment.into()  
    }
}