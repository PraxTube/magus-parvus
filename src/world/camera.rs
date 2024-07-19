use bevy::prelude::*;
use bevy::render::camera::ScalingMode;
#[cfg(not(target_arch = "wasm32"))]
use bevy::render::view::screenshot::ScreenshotManager;
#[cfg(not(target_arch = "wasm32"))]
use bevy::window::{PrimaryWindow, WindowMode};
use bevy_kira_audio::prelude::AudioReceiver;
use bevy_rapier2d::plugin::PhysicsSet;

use super::camera_shake::{update_camera, CameraShake};
use crate::player::input::PlayerInput;
use crate::player::Player;
#[cfg(not(target_arch = "wasm32"))]
use crate::player::PlayerState;
use crate::spell::debug_spell::DebugSpell;

// How much `1.0` in bevy coordinates translates to the pixels of a sprite.
// Only relevant for the ysorting.
pub const TRANSLATION_TO_PIXEL: f32 = 0.0001;

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

fn update_camera_target(mut shake: ResMut<CameraShake>, q_player: Query<&Transform, With<Player>>) {
    let player_transform = match q_player.get_single() {
        Ok(r) => r,
        Err(_) => return,
    };
    shake.update_target(player_transform.translation.truncate());
}

fn zoom_camera(
    debug_spell: Res<DebugSpell>,
    mut q_projection: Query<&mut OrthographicProjection, With<MainCamera>>,
    player_input: Res<PlayerInput>,
) {
    if !debug_spell.active {
        return;
    }

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

#[cfg(not(target_arch = "wasm32"))]
fn take_screenshot(
    keys: Res<ButtonInput<KeyCode>>,
    main_window: Query<Entity, With<PrimaryWindow>>,
    mut screenshot_manager: ResMut<ScreenshotManager>,
    mut counter: Local<u32>,
) {
    if !keys.just_pressed(KeyCode::F12) {
        return;
    }

    let path = format!("./screenshot-{}.png", *counter);
    *counter += 1;
    match screenshot_manager.save_screenshot_to_disk(main_window.single(), path) {
        Ok(()) => {}
        Err(err) => error!("failed to take screenshot, {}", err),
    }
}

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                #[cfg(not(target_arch = "wasm32"))]
                toggle_full_screen,
                #[cfg(not(target_arch = "wasm32"))]
                take_screenshot,
                apply_y_sort,
                zoom_camera,
            ),
        )
        .add_systems(Startup, spawn_camera)
        .add_systems(
            PostUpdate,
            update_camera_target
                .after(PhysicsSet::Writeback)
                .before(TransformSystem::TransformPropagate)
                .before(update_camera),
        );
    }
}
