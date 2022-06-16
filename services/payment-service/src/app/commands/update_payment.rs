use crate::{SharedCosmosRepository, app::PaymentDto, domain::Payment};
use common::data::Repository;
use mediator::{AsyncRequestHandler, DefaultAsyncMediator, Request};
use serde::{Deserialize, Serialize};
 

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdatePaymentCommand {
  pub id: String,
  pub payment: PaymentDto,
}

impl Request<PaymentDto> for UpdatePaymentCommand {}

pub struct UpdatePaymentCommandHandler(pub SharedCosmosRepository<Payment>, pub DefaultAsyncMediator);

#[mediator::async_trait]
impl AsyncRequestHandler<UpdatePaymentCommand, PaymentDto> for UpdatePaymentCommandHandler {
    async fn handle(&mut self, command: UpdatePaymentCommand) -> PaymentDto {
      let lock = self.0.lock().await;
      let payment = lock.update(command.payment.into(), command.id.clone()).await.expect("Failed to update customer");
      payment.into()
    }
}