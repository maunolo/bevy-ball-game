use bevy::{
    prelude::Resource,
    time::{Timer, TimerMode},
};

pub const STAR_SPAWN_TIME: f32 = 1.0; // once per second

#[derive(Resource)]
pub struct StarSpawnTimer {
    pub timer: Timer,
}

impl Default for StarSpawnTimer {
    fn default() -> Self {
        Self {
            timer: Timer::from_seconds(STAR_SPAWN_TIME, TimerMode::Repeating),
        }
    }
}
