use azure_data_cosmos::CosmosEntity;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::app::CustomerDto;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Customer {
    pub id: String,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub address: String,
    pub city: String,
    pub state: String,
    pub phone: String,
    pub birthday: DateTime<Utc>,
    pub favourite_dishes: Vec<String>,
    pub created_by: Option<String>,
    pub created_on: Option<DateTime<Utc>>,
    pub modified_by: Option<String>,
    pub modfied_on: Option<DateTime<Utc>>,
}

impl CosmosEntity for Customer {
    type Entity = String;

    fn partition_key(&self) -> Self::Entity {
        self.id.clone()
    }
}

impl From<CustomerDto> for Customer {
    fn from(dto: CustomerDto) -> Self {
        Self {
            id: dto.id,
            first_name: dto.first_name,
            last_name: dto.last_name,
            email: dto.email,
            address: dto.address,
            city: dto.city,
            state: dto.state,
            phone: dto.phone,
            birthday: dto.birthday,
            favourite_dishes: dto.favourite_dishes,
            created_by: None,
            created_on: None,
            modified_by: None,
            modfied_on: None,
        }
    }
}
