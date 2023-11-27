use bevy::prelude::*;

#[derive(Component)]
pub struct Stats {
    pub move_speed: f32,
}

impl Default for Stats {
    fn default() -> Self {
        Self { move_speed: 150.0 }
    }
}
