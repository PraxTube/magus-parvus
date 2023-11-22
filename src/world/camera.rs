use bevy::prelude::*;
use bevy::render::camera::ScalingMode;
use bevy::window::{PrimaryWindow, WindowMode};

use crate::GameState;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, move_camera.run_if(in_state(GameState::Gaming)))
            .add_systems(OnEnter(GameState::Gaming), spawn_camera)
            .add_systems(Update, toggle_full_screen);
    }
}

#[derive(Component)]
pub struct MainCamera;

fn spawn_camera(mut commands: Commands) {
    let mut camera = Camera2dBundle::default();
    camera.projection.scaling_mode = ScalingMode::FixedVertical(750.0);
    commands.spawn((MainCamera, camera));
}

fn move_camera() {}

fn toggle_full_screen(
    mut main_window: Query<&mut Window, With<PrimaryWindow>>,
    keys: Res<Input<KeyCode>>,
    gamepads: Res<Gamepads>,
    button_inputs: Res<Input<GamepadButton>>,
) {
    let mut window = match main_window.get_single_mut() {
        Ok(w) => w,
        Err(err) => {
            error!("there is not exactly one window, {}", err);
            return;
        }
    };

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
