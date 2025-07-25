use sqlx::{PgPool, postgres::PgPoolOptions};
use std::sync::Arc;

use crate::{CONFIG, Canvas, CooldownManager};

#[derive(Debug, Clone)]
pub struct AppState {
    pub db: Arc<PgPool>,
    pub canvas: Canvas,
    pub cooldown_manager: CooldownManager,
}

impl AppState {
    pub async fn new() -> Result<Self, String> {
        let db = Arc::new(
            PgPoolOptions::new()
                .max_connections(5)
                .connect(&CONFIG.database_url)
                .await
                .map_err(|e| format!("Database connection failed: {}", e))?,
        );

        let canvas = Canvas::new();

        let cooldown_manager = CooldownManager::new(10);

        Ok(AppState {
            db,
            canvas,
            cooldown_manager,
        })
    }
}
