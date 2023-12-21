pub mod input;
pub mod speed_timer;
pub mod state;
pub mod stats;

mod audio;
mod collision;
mod movement;
mod spawn;
mod sprite;

pub use state::PlayerChangedState;
pub use state::PlayerState;

use bevy::prelude::*;

use crate::world::CHUNK_SIZE;

pub const PLAYER_SPAWN_POS: Vec3 = Vec3::new(2.5 * CHUNK_SIZE, 16.0 + 2.5 * CHUNK_SIZE, 0.0);
pub const PLAYER_HEALTH: f32 = 10.0;
const STAGGERING_TIME: f32 = 0.25;
const STAGGERING_INTENSITY: f32 = 200.0;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            input::InputPlugin,
            state::PlayerStatePlugin,
            audio::PlayerAudioPlugin,
            sprite::PlayerSpritePlugin,
            collision::PlayerCollisionPlugin,
            movement::PlayerMovementPlugin,
            spawn::PlayerSpawnPlugin,
            speed_timer::SpeedTimerPlugin,
        ));
    }
}

#[derive(Component)]
pub struct Player {
    pub state: PlayerState,
    pub current_direction: Vec2,
    pub collider_entity: Entity,
    pub staggering_timer: Timer,
}

impl Player {
    fn new(collider_entity: Entity) -> Self {
        Self {
            state: PlayerState::default(),
            current_direction: Vec2::ZERO,
            collider_entity,
            staggering_timer: Timer::from_seconds(STAGGERING_TIME, TimerMode::Repeating),
        }
    }
}
