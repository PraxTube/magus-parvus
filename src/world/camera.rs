use bevy::prelude::*;
use bevy::render::camera::ScalingMode;
#[cfg(not(target_arch = "wasm32"))]
use bevy::window::{PrimaryWindow, WindowMode};
use bevy_kira_audio::prelude::SpatialAudioReceiver;
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
    let camera = Camera2d {};
    let mut projection = OrthographicProjection::default_2d();
    projection.scaling_mode = ScalingMode::FixedVertical {
        viewport_height: 400.0,
    };
    commands.spawn((
        MainCamera,
        SpatialAudioReceiver,
        camera,
        Projection::Orthographic(projection),
    ));
}

fn update_camera_target(mut shake: ResMut<CameraShake>, q_player: Query<&Transform, With<Player>>) {
    let player_transform = match q_player.single() {
        Ok(r) => r,
        Err(_) => return,
    };
    shake.update_target(player_transform.translation.truncate());
}

fn zoom_camera(
    debug_spell: Res<DebugSpell>,
    mut q_projection: Query<&mut Projection, With<MainCamera>>,
    player_input: Res<PlayerInput>,
) {
    if !debug_spell.active {
        return;
    }

    let mut projection = match q_projection.single_mut() {
        Ok(p) => p,
        Err(_) => return,
    };

    let Projection::Orthographic(mut orth) = projection.clone() else {
        return;
    };
    orth.scale = (orth.scale + player_input.zoom).clamp(1.0, 10.0);
    *projection = Projection::Orthographic(orth)
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

    let mut window = match main_window.single_mut() {
        Ok(w) => w,
        Err(err) => {
            error!("there is not exactly one window, {}", err);
            return;
        }
    };

    let player_state = match q_player.single() {
        Ok(p) => p.state,
        Err(_) => return,
    };
    if player_state == PlayerState::Casting {
        return;
    }

    window.mode = if window.mode == WindowMode::Windowed {
        WindowMode::Fullscreen(MonitorSelection::Primary, VideoModeSelection::Current)
    } else {
        WindowMode::Windowed
    }
}

#[cfg(not(target_arch = "wasm32"))]
fn take_screenshot(
    mut commands: Commands,
    input: Res<ButtonInput<KeyCode>>,
    mut counter: Local<u32>,
) {
    use bevy::render::view::screenshot::{save_to_disk, Screenshot};

    if input.just_pressed(KeyCode::F12) {
        let path = format!("./screenshot-{}.png", *counter);
        *counter += 1;
        commands
            .spawn(Screenshot::primary_window())
            .observe(save_to_disk(path));
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
