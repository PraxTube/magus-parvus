pub mod camera;
pub mod game_entity;
mod map;

pub use camera::MainCamera;

use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            camera::CameraPlugin,
            map::MapPlugin,
            game_entity::GameEntityPlugin,
        ))
        .add_systems(Startup, configure_physics);
    }
}

fn configure_physics(mut rapier_config: ResMut<RapierConfiguration>) {
    rapier_config.gravity = Vec2::ZERO;
}
