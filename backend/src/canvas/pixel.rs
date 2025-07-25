use time::OffsetDateTime;
use uuid::Uuid;

use super::Color;

#[derive(Debug, Clone, Copy)]
pub struct Pixel {
    pub color: Color,
    pub updated_by: Option<Uuid>,
    pub updated_at_ms: u64,
}

impl Pixel {
    pub fn new() -> Self {
        let now = OffsetDateTime::now_utc();
        let millis = now.unix_timestamp() * 1000 + (now.nanosecond() as i64 / 1_000_000);

        Self {
            color: Color::White,
            updated_by: None,
            updated_at_ms: millis as u64,
        }
    }
}
