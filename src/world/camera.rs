use bevy::prelude::*;
use bevy::render::camera::ScalingMode;
use bevy::window::{PrimaryWindow, WindowMode};

use crate::player::{player_movement, Player, PlayerState};
use crate::GameState;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            move_camera
                .after(player_movement)
                .run_if(in_state(GameState::Gaming)),
        )
        .add_systems(OnEnter(GameState::Gaming), spawn_camera)
        .add_systems(Update, toggle_full_screen);
    }
}

#[derive(Component)]
pub struct MainCamera;

fn spawn_camera(mut commands: Commands) {
    let mut camera = Camera2dBundle::default();
    camera.projection.scaling_mode = ScalingMode::FixedVertical(400.0);
    commands.spawn((MainCamera, camera));
}

fn move_camera(
    mut q_camera: Query<&mut Transform, (With<MainCamera>, Without<Player>)>,
    q_player: Query<&Transform, With<Player>>,
) {
    let player_pos = match q_player.get_single() {
        Ok(player) => player.translation,
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
    camera_transform.translation = player_pos;
}

fn toggle_full_screen(
    mut main_window: Query<&mut Window, With<PrimaryWindow>>,
    keys: Res<Input<KeyCode>>,
    gamepads: Res<Gamepads>,
    button_inputs: Res<Input<GamepadButton>>,
    q_player: Query<&Player>,
) {
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

    let mut pressed = keys.just_pressed(KeyCode::B);
    for gamepad in gamepads.iter() {
        if button_inputs.just_pressed(GamepadButton::new(gamepad, GamepadButtonType::DPadUp)) {
            pressed = true;
        }
    }

    if !pressed {
        return;
    }

    window.mode = if window.mode == WindowMode::Windowed {
        WindowMode::Fullscreen
    } else {
        WindowMode::Windowed
    }
}
