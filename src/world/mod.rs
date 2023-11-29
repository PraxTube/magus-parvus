pub mod camera;
pub mod game_entity;
mod map;

pub use camera::MainCamera;

use bevy::prelude::*;

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            camera::CameraPlugin,
            map::MapPlugin,
            game_entity::GameEntityPlugin,
        ));
    }
}
