use std::collections::HashSet;

use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;

use crate::{GameAssets, GameState, Player};

const MAP_SIZE: f32 = 32.0 * 32.0;
const CAMERA_SIZE_X: f32 = 400.0;
const CAMERA_SIZE_Y: f32 = 300.0;

#[derive(Component)]
pub struct Chunk {
    x_index: i32,
    y_index: i32,
}

fn map_indices_to_world_coords(x_index: i32, y_index: i32) -> Vec3 {
    Vec3::new(
        x_index as f32 * MAP_SIZE,
        y_index as f32 * MAP_SIZE,
        -1000.0,
    )
}

fn world_coords_to_map_indices(position: Vec3) -> (i32, i32) {
    let x_index = (position.x / MAP_SIZE) as i32 + if position.x < 0.0 { -1 } else { 0 };
    let y_index = (position.y / MAP_SIZE) as i32 + if position.y < 0.0 { -1 } else { 0 };
    (x_index, y_index)
}

fn adjust_chunks(
    mut commands: Commands,
    assets: Res<GameAssets>,
    q_player: Query<&Transform, With<Player>>,
    q_chunks: Query<(Entity, &Chunk)>,
) {
    let player_pos = q_player.single().translation;

    let unique_indices: HashSet<_> = [
        world_coords_to_map_indices(player_pos + Vec3::new(CAMERA_SIZE_X, CAMERA_SIZE_Y, 0.0)),
        world_coords_to_map_indices(player_pos + Vec3::new(-CAMERA_SIZE_X, CAMERA_SIZE_Y, 0.0)),
        world_coords_to_map_indices(player_pos + Vec3::new(CAMERA_SIZE_X, -CAMERA_SIZE_Y, 0.0)),
        world_coords_to_map_indices(player_pos + Vec3::new(-CAMERA_SIZE_X, -CAMERA_SIZE_Y, 0.0)),
    ]
    .into_iter()
    .collect();
    let indices: Vec<_> = unique_indices.into_iter().collect();
    let mut chunk_exist_flags: Vec<bool> = vec![false; indices.len()];

    for (entity, chunk) in &q_chunks {
        let mut despawn = true;
        for i in 0..indices.len() {
            if indices[i].0 == chunk.x_index && indices[i].1 == chunk.y_index {
                chunk_exist_flags[i] = true;
                despawn = false;
                break;
            }
        }
        if despawn {
            commands.entity(entity).despawn_recursive();
        }
    }

    for k in 0..indices.len() {
        if chunk_exist_flags[k] {
            continue;
        }

        let i = indices[k].0;
        let j = indices[k].1;

        commands.spawn((
            Chunk {
                x_index: i,
                y_index: j,
            },
            LdtkWorldBundle {
                ldtk_handle: assets.level.clone(),
                transform: Transform::from_translation(map_indices_to_world_coords(i, j)),
                ..Default::default()
            },
        ));
    }
}

pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(LdtkPlugin)
            .insert_resource(LevelSelection::index(0))
            .add_systems(Update, (adjust_chunks).run_if(in_state(GameState::Gaming)));
    }
}
