use bevy::input::mouse::{MouseScrollUnit, MouseWheel};
use bevy::prelude::*;
use bevy::window::{PrimaryWindow, Window};

use crate::world::MainCamera;
use crate::GameState;

#[derive(Resource, Default)]
pub struct MouseWorldCoords(pub Vec2);

pub fn fetch_mouse_world_coords(
    mut mouse_coords: ResMut<MouseWorldCoords>,
    q_window: Query<&Window, With<PrimaryWindow>>,
    q_camera: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
) {
    let (camera, camera_transform) = q_camera.single();
    let window = q_window.single();

    if let Some(world_position) = window
        .cursor_position()
        .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor))
        .map(|ray| ray.origin.truncate())
    {
        mouse_coords.0 = world_position;
    }
}

fn fetch_scroll_events(
    mut scroll_evr: EventReader<MouseWheel>,
    mut q_projection: Query<&mut OrthographicProjection, With<MainCamera>>,
) {
    for ev in scroll_evr.read() {
        let mut projection = q_projection.single_mut();
        match ev.unit {
            MouseScrollUnit::Line => {
                let scroll = if ev.y > 0.0 { -1.0 } else { 1.0 };
                projection.scale = (projection.scale + scroll).clamp(1.0, 10.0);
            }
            MouseScrollUnit::Pixel => {
                let scroll = if ev.y > 0.0 { -1.0 } else { 1.0 };
                projection.scale = (projection.scale + scroll).clamp(1.0, 10.0);
            }
        }
    }
}

pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (fetch_scroll_events, fetch_mouse_world_coords)
                .chain()
                .run_if(in_state(GameState::Gaming)),
        )
        .init_resource::<MouseWorldCoords>();
    }
}
