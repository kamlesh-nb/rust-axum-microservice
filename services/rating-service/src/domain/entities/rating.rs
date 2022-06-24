 
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::app::RatingDto;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Rating {
    pub id: String,
    pub customer_id: String,
    pub menu_id: String,
    pub score: i16,
    pub remarks: String,
    pub date_recorded: DateTime<Utc>,
}

impl azure_data_cosmos::CosmosEntity for Rating {
    type Entity = String;

    fn partition_key(&self) -> Self::Entity {
        self.id.clone()
    }
}

impl From<RatingDto> for Rating {
    fn from(dto: RatingDto) -> Self {
        Self {
            id: dto.id,
            customer_id: dto.customer_id,
            menu_id: dto.menu_id,
            score: dto.score,
            remarks: dto.remarks,
            date_recorded: dto.date_recorded,
        }
    }
}
