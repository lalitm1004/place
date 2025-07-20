use std::sync::{Arc, RwLock};
use time::OffsetDateTime;
use uuid::Uuid;

use crate::Color;

const CANVAS_SIZE: usize = 1024 * 1024;

#[derive(Debug)]
pub struct Canvas(Arc<[RwLock<Pixel>; CANVAS_SIZE]>);

impl Canvas {
    pub fn new() -> Self {
        let pixels: [RwLock<Pixel>; CANVAS_SIZE] =
            std::array::from_fn(|_| RwLock::new(Pixel::new()));

        Self(Arc::new(pixels))
    }
}

impl std::ops::Deref for Canvas {
    type Target = Arc<[RwLock<Pixel>; CANVAS_SIZE]>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Pixel {
    pub color: Color,
    pub updated_by: Option<Uuid>,
    pub updated_at_ms: u128,
}

impl Pixel {
    pub fn new() -> Self {
        let now = OffsetDateTime::now_utc();
        let millis = now.unix_timestamp() * 1000 + (now.nanosecond() as i64 / 1_000_000);

        Self {
            color: Color::White,
            updated_by: None,
            updated_at_ms: millis as u128,
        }
    }
}
