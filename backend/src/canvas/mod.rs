mod color;
mod pixel;

pub use color::Color;
pub use pixel::Pixel;
use time::OffsetDateTime;
use uuid::Uuid;

use std::sync::{Arc, RwLock};

pub const CANVAS_SIZE: usize = 1024 * 1024;

#[derive(Debug, Clone)]
pub struct Canvas(Arc<[RwLock<Pixel>; CANVAS_SIZE]>);

impl Canvas {
    pub fn new() -> Self {
        let pixels: [RwLock<Pixel>; CANVAS_SIZE] =
            std::array::from_fn(|_| RwLock::new(Pixel::new()));

        Self(Arc::new(pixels))
    }
}

impl Canvas {
    pub fn update_pixel(
        &self,
        index: usize,
        color: Color,
        user_id: Uuid,
    ) -> Result<(), UpdatePixelError> {
        if index >= CANVAS_SIZE {
            return Err(UpdatePixelError::OutOfBounds);
        }

        let now = OffsetDateTime::now_utc();
        let millis = now.unix_timestamp() * 1000 + (now.nanosecond() as i64 / 1_000_000);

        let pixel_lock = &self[index];
        let mut pixel = pixel_lock
            .write()
            .map_err(|_| UpdatePixelError::PoisonedLock)?;

        pixel.color = color;
        pixel.updated_at_ms = millis as u64;
        pixel.updated_by = Some(user_id);

        Ok(())
    }
}

impl std::ops::Deref for Canvas {
    type Target = Arc<[RwLock<Pixel>; CANVAS_SIZE]>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Debug)]
pub enum UpdatePixelError {
    OutOfBounds,
    PoisonedLock,
}

impl std::error::Error for UpdatePixelError {}

impl std::fmt::Display for UpdatePixelError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            UpdatePixelError::OutOfBounds => write!(f, "Attempted to update a pixel out of bounds"),
            UpdatePixelError::PoisonedLock => write!(f, "The pixel buffer lock was poisoned"),
        }
    }
}
