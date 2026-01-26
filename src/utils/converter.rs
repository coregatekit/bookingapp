use std::error::Error;

use uuid::Uuid;

pub fn string_to_uuid(s: &str) -> Result<Uuid, Box<dyn Error>> {
    let uuid = Uuid::parse_str(s)?;
    Ok(uuid)
}