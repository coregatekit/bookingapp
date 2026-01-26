use bigdecimal::BigDecimal;
use chrono::Utc;
use serde::{Deserialize, Serialize};
use std::error::Error;
use uuid::Uuid;

use crate::domain::entities::zones::CreateZoneEntity;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateZoneModel {
    pub label: String,
    pub price: BigDecimal,
    pub total_seats: i32,
}

impl CreateZoneModel {
    pub fn to_entity(&self, event_id: Uuid) -> Result<CreateZoneEntity, Box<dyn Error>> {
        Ok(CreateZoneEntity {
            event_id,
            label: self.label.clone(),
            price: self.price.clone(),
            total_seats: self.total_seats,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        })
    }
}
