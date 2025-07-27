use std::{collections::HashMap, fmt, sync::Arc};
use tokio::{
    sync::RwLock,
    time::{self, Duration, Instant},
};
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct CooldownManager {
    cooldowns: Arc<RwLock<HashMap<Uuid, Instant>>>,
    cooldown_duration: Duration,
}

impl CooldownManager {
    pub fn new(cooldown_seconds: u64) -> Self {
        let manager = Self {
            cooldowns: Arc::new(RwLock::new(HashMap::new())),
            cooldown_duration: Duration::from_secs(cooldown_seconds),
        };

        let cooldowns_clone = manager.cooldowns.clone();
        let cooldown_duration_clone = manager.cooldown_duration;

        // spawn cleanup task
        tokio::spawn(async move {
            let mut interval = time::interval(Duration::from_secs(30));
            loop {
                interval.tick().await;
                let now = Instant::now();

                let mut cooldowns = cooldowns_clone.write().await;
                cooldowns.retain(|_, &mut last_access| {
                    now.duration_since(last_access) < cooldown_duration_clone * 2
                });
            }
        });

        manager
    }

    pub async fn check_and_update_cooldown(&self, user_id: Uuid) -> Result<(), CooldownError> {
        let now = Instant::now();

        let cooldowns = self.cooldowns.read().await;

        if let Some(&last_access) = cooldowns.get(&user_id) {
            let elapsed = now.duration_since(last_access);
            if elapsed < self.cooldown_duration {
                return Err(CooldownError::OnCooldown(self.cooldown_duration - elapsed));
            }
        }

        // drop read to acquire write
        std::mem::drop(cooldowns);

        let mut cooldowns = self.cooldowns.write().await;
        cooldowns.insert(user_id, now);

        Ok(())
    }
}

#[derive(Debug)]
pub enum CooldownError {
    OnCooldown(Duration),
}

impl std::error::Error for CooldownError {}

impl fmt::Display for CooldownError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CooldownError::OnCooldown(duration) => {
                write!(f, "action is on cooldown for {:.2?}", duration)
            }
        }
    }
}
