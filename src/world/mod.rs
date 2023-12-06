pub mod camera;
pub mod game_entity;
mod map;
mod map_colliders;

pub use camera::MainCamera;

use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

pub const BACKGROUND_ZINDEX_ABS: f32 = 1_000.0;
pub const CHUNK_SIZE: f32 = 32.0 * 32.0;

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            camera::CameraPlugin,
            map::MapPlugin,
            map_colliders::MapColliderPlugin,
            game_entity::GameEntityPlugin,
        ))
        .add_systems(Startup, configure_physics);
    }
}

fn configure_physics(mut rapier_config: ResMut<RapierConfiguration>) {
    rapier_config.gravity = Vec2::ZERO;
}
