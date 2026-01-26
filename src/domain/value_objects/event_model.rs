use anyhow::Result;
use chrono::{DateTime, ParseError, Utc};
use serde::{Deserialize, Serialize};

use crate::domain::{
    entities::events::CreateEventEntity, value_objects::event_statuses::EventStatuses,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateEventModel {
    pub name: String,
    pub description: Option<String>,
    pub performer: String,
    pub date: String,
    pub location: String,
}

impl CreateEventModel {
    pub fn to_entity(&self) -> Result<CreateEventEntity, ParseError> {
        Ok(CreateEventEntity {
            name: self.name.clone(),
            description: self.description.clone(),
            performer: self.performer.clone(),
            date: DateTime::parse_from_rfc3339(&self.date)?
                .with_timezone(&Utc),
            location: self.location.clone(),
            status: EventStatuses::Scheduled.to_string(),
            created_at: Utc::now(),
            updated_at: Utc::now(),
        })
    }
}
