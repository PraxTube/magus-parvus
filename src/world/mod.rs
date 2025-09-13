pub mod camera;
pub mod camera_shake;

mod map;
mod rapier_debug;

pub use camera::MainCamera;
pub use camera_shake::CameraShake;

use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

pub const BACKGROUND_ZINDEX_ABS: f32 = 1_000.0;
pub const CHUNK_SIZE: f32 = 32.0 * 32.0;

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            camera::CameraPlugin,
            camera_shake::CameraShakePlugin,
            map::MapPlugin,
            rapier_debug::RapierDebugPlugin,
        ))
        .add_systems(Startup, configure_physics);
    }
}

fn configure_physics(mut rapier_config: Query<&mut RapierConfiguration>) {
    let mut rapier_config = rapier_config.single_mut();
    rapier_config.gravity = Vec2::ZERO;
}
