use std::fmt;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub enum EventStatuses {
    #[default]
    Scheduled,
    Cancelled,
    Ongoing,
    Finished,
}
impl fmt::Display for EventStatuses {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            EventStatuses::Scheduled => write!(f, "scheduled"),
            EventStatuses::Cancelled => write!(f, "cancelled"),
            EventStatuses::Ongoing => write!(f, "ongoing"),
            EventStatuses::Finished => write!(f, "finished"),
        }
    }
}
