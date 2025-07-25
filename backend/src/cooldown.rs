use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
    time::{Duration, Instant},
};

use tokio::time;
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct CooldownManager {
    cooldowns: Arc<RwLock<HashMap<Uuid, Instant>>>,
    pub cooldown_duration: Duration,
}

impl CooldownManager {
    pub fn new(cooldown_seconds: u64) -> Self {
        let manager = Self {
            cooldowns: Arc::new(RwLock::new(HashMap::new())),
            cooldown_duration: Duration::from_secs(cooldown_seconds),
        };

        let cleanup_cooldowns = manager.cooldowns.clone();
        let cleanup_duration = manager.cooldown_duration;
        tokio::spawn(async move {
            let mut interval = time::interval(Duration::from_secs(30));
            loop {
                interval.tick().await;
                let now = Instant::now();
                if let Ok(mut cooldowns) = cleanup_cooldowns.write() {
                    cooldowns.retain(|_, &mut last_access| {
                        now.duration_since(last_access) < cleanup_duration * 2
                    });
                }
            }
        });

        manager
    }

    pub fn check_and_update_cooldown(&self, user_id: Uuid) -> Result<(), CooldownError> {
        let now = Instant::now();

        if let Ok(cooldowns) = self.cooldowns.read() {
            if let Some(&last_access) = cooldowns.get(&user_id) {
                let elapsed = now.duration_since(last_access);
                if elapsed < self.cooldown_duration {
                    return Err(CooldownError::OnCooldown(self.cooldown_duration - elapsed));
                }
            }
        }

        if let Ok(mut cooldowns) = self.cooldowns.write() {
            cooldowns.insert(user_id, now);
            Ok(())
        } else {
            Err(CooldownError::PosionedLock)
        }
    }
}

#[derive(Debug)]
pub enum CooldownError {
    OnCooldown(Duration),
    PosionedLock,
}

impl std::error::Error for CooldownError {}

impl std::fmt::Display for CooldownError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CooldownError::OnCooldown(duration) => {
                write!(f, "Action is on cooldown for {:.2?}", duration)
            }
            CooldownError::PosionedLock => {
                write!(f, "The cooldown manager lock was poisoned")
            }
        }
    }
}
