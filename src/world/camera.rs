use bevy::prelude::*;
use bevy::render::camera::ScalingMode;
use bevy::window::{PrimaryWindow, WindowMode};
use bevy_kira_audio::prelude::AudioReceiver;
use bevy_rapier2d::dynamics::Velocity;

use crate::player::input::PlayerInput;
use crate::player::{Player, PlayerState};
use crate::GameState;

// How much `1.0` in bevy coordinates translates to the pixels of a sprite.
// Only relevant for the ysorting.
pub const TRANSLATION_TO_PIXEL: f32 = 0.0001;
// This is not changing the actual timestep,
// it's just a way to reduce magic numbers in code.
const RAPIER_TIMESTEP: f32 = 60.0;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                #[cfg(not(target_arch = "wasm32"))]
                toggle_full_screen,
                apply_y_sort,
                zoom_camera,
            ),
        )
        .add_systems(OnEnter(GameState::Gaming), spawn_camera)
        .add_systems(PostUpdate, move_camera.run_if(in_state(GameState::Gaming)));
    }
}

#[derive(Component)]
pub struct MainCamera;

#[derive(Component)]
pub struct YSort(pub f32);

pub fn apply_y_sort(mut q_transforms: Query<(&mut Transform, &GlobalTransform, &YSort)>) {
    for (mut transform, global_transform, ysort) in &mut q_transforms {
        transform.translation.z = ysort.0 - global_transform.translation().y * TRANSLATION_TO_PIXEL;
    }
}

fn spawn_camera(mut commands: Commands) {
    let mut camera = Camera2dBundle::default();
    camera.projection.scaling_mode = ScalingMode::FixedVertical(400.0);
    commands.spawn((MainCamera, AudioReceiver, camera));
}

fn move_camera(
    mut q_camera: Query<&mut Transform, (With<MainCamera>, Without<Player>)>,
    q_player: Query<(&Transform, &Velocity), With<Player>>,
) {
    let (player_pos, player_vel) = match q_player.get_single() {
        Ok(p) => (p.0.translation, p.1),
        Err(err) => {
            error!("no player! cannot move camera, {}", err);
            return;
        }
    };

    let mut camera_transform = match q_camera.get_single_mut() {
        Ok(c) => c,
        Err(err) => {
            error!("there is not exactly one main camera, {}", err);
            return;
        }
    };
    camera_transform.translation = Vec3::new(
        player_pos.x + player_vel.linvel.x / RAPIER_TIMESTEP,
        player_pos.y + player_vel.linvel.y / RAPIER_TIMESTEP,
        camera_transform.translation.z,
    );
}

fn zoom_camera(
    mut q_projection: Query<&mut OrthographicProjection, With<MainCamera>>,
    player_input: Res<PlayerInput>,
) {
    let mut projection = match q_projection.get_single_mut() {
        Ok(p) => p,
        Err(_) => return,
    };

    projection.scale = (projection.scale + player_input.zoom).clamp(1.0, 10.0);
}

#[cfg(not(target_arch = "wasm32"))]
fn toggle_full_screen(
    mut main_window: Query<&mut Window, With<PrimaryWindow>>,
    q_player: Query<&Player>,
    player_input: Res<PlayerInput>,
) {
    if !player_input.toggle_fullscreen {
        return;
    }

    let mut window = match main_window.get_single_mut() {
        Ok(w) => w,
        Err(err) => {
            error!("there is not exactly one window, {}", err);
            return;
        }
    };

    let player_state = match q_player.get_single() {
        Ok(p) => p.state,
        Err(_) => return,
    };
    if player_state == PlayerState::Casting {
        return;
    }

    window.mode = if window.mode == WindowMode::Windowed {
        WindowMode::Fullscreen
    } else {
        WindowMode::Windowed
    }
}
