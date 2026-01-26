use bigdecimal::BigDecimal;
use chrono::Utc;
use serde::{Deserialize, Serialize};
use std::error::Error;

use crate::{domain::entities::zones::CreateZoneEntity, utils::converter::string_to_uuid};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateZoneModel {
    pub event_id: String,
    pub label: String,
    pub price: BigDecimal,
    pub total_seats: i32,
}

impl CreateZoneModel {
    pub fn to_entity(&self) -> Result<CreateZoneEntity, Box<dyn Error>> {
        Ok(CreateZoneEntity {
            event_id: string_to_uuid(&self.event_id)?,
            label: self.label.clone(),
            price: self.price.clone(),
            total_seats: self.total_seats,
            created_at: Utc::now().naive_utc(),
            updated_at: Utc::now().naive_utc(),
        })
    }
}
