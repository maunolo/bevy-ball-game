use bevy::{
    prelude::Resource,
    time::{Timer, TimerMode},
};

pub const ENEMY_SPAWN_TIME: f32 = 5.0; // once per 5 seconds

#[derive(Resource)]
pub struct EnemySpawnTimer {
    pub timer: Timer,
}

impl Default for EnemySpawnTimer {
    fn default() -> Self {
        Self {
            timer: Timer::from_seconds(ENEMY_SPAWN_TIME, TimerMode::Repeating),
        }
    }
}
