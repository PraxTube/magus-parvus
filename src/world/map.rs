use std::collections::HashSet;

use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;

use crate::player::Player;
use crate::{GameAssets, GameState};

const CHUNK_SIZE: f32 = 32.0 * 32.0;
const CAMERA_SIZE_X: f32 = 400.0;
const CAMERA_SIZE_Y: f32 = 300.0;
const CHUNK_ROWS: usize = 2;
const IIDS: [&str; 6] = [
    "4561cae1-8990-11ee-bdb7-27b92e7f0bd1",
    "4c5c13d0-8990-11ee-bb97-5335be5f091d",
    "30c12d00-8990-11ee-8c0e-1f466f38a0b0",
    "09bdb020-8990-11ee-8c0e-83df39a96f91",
    "39c4ea40-8990-11ee-8c0e-f5477a2dc37e",
    "54eaef30-8990-11ee-bb97-69638b6a5187",
];

#[derive(Component)]
pub struct Chunk {
    x_index: i32,
    y_index: i32,
}

fn map_indices_to_world_coords(x_index: i32, y_index: i32) -> Vec3 {
    Vec3::new(
        x_index as f32 * CHUNK_SIZE,
        y_index as f32 * CHUNK_SIZE,
        -1000.0,
    )
}

fn world_coords_to_map_indices(position: Vec3) -> (i32, i32) {
    let x_index = (position.x / CHUNK_SIZE) as i32 + if position.x < 0.0 { -1 } else { 0 };
    let y_index = (position.y / CHUNK_SIZE) as i32 + if position.y < 0.0 { -1 } else { 0 };
    (x_index, y_index)
}

fn map_indices_to_index(x_index: i32, y_index: i32) -> usize {
    let m = x_index.unsigned_abs() as usize;
    let n = y_index.unsigned_abs() as usize;

    m + n * CHUNK_ROWS
}

fn level_set_from_map_indices(x_index: i32, y_index: i32) -> LevelSet {
    let index = map_indices_to_index(x_index, y_index);
    if index >= IIDS.len() {
        return LevelSet::from_iids([IIDS[0]]);
    }
    LevelSet::from_iids([IIDS[index]])
}

fn adjust_chunks(
    mut commands: Commands,
    assets: Res<GameAssets>,
    q_player: Query<&Transform, With<Player>>,
    q_chunks: Query<(Entity, &Chunk)>,
) {
    let player_pos = match q_player.get_single() {
        Ok(p) => p.translation,
        Err(_) => return,
    };

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
                transform: Transform::from_translation(map_indices_to_world_coords(i, j)),
                ldtk_handle: assets.level.clone(),
                level_set: LevelSet::from_iids(level_set_from_map_indices(i, j)),
                ..Default::default()
            },
        ));
    }
}

pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(LdtkPlugin)
            .insert_resource(LdtkSettings {
                level_spawn_behavior: LevelSpawnBehavior::UseZeroTranslation,
                ..default()
            })
            .add_systems(Update, (adjust_chunks).run_if(in_state(GameState::Gaming)));
    }
}
