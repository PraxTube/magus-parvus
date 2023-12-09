use std::collections::HashSet;

use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use bevy_rapier2d::geometry::Collider;

use super::CHUNK_SIZE;
use crate::player::Player;
use crate::{GameAssets, GameState};

use super::BACKGROUND_ZINDEX_ABS;

const CAMERA_SIZE_X: f32 = 400.0;
const CAMERA_SIZE_Y: f32 = 300.0;
const CHUNK_ROWS: usize = 5;
const IIDS: [&str; 25] = [
    "4561cae1-8990-11ee-bdb7-27b92e7f0bd1",
    "4c5c13d0-8990-11ee-bb97-5335be5f091d",
    "98936980-8990-11ee-bec7-fdf4bcea93a8",
    "09315e40-8990-11ee-bec7-1d8ee619301e",
    "0c0e6680-8990-11ee-bec7-c93ceae6c5ca",
    "30c12d00-8990-11ee-8c0e-1f466f38a0b0",
    "09bdb020-8990-11ee-8c0e-83df39a96f91",
    "1111d130-8990-11ee-bec7-e700d2272088",
    "124d1050-8990-11ee-bec7-6198e863d073",
    "1827a120-8990-11ee-bec7-4f3cacef695d",
    "39c4ea40-8990-11ee-8c0e-f5477a2dc37e",
    "54eaef30-8990-11ee-bb97-69638b6a5187",
    "1a354b70-8990-11ee-bec7-730615478666",
    "1b511bb0-8990-11ee-bec7-19cc658e1943",
    "1c70e390-8990-11ee-bec7-0502e0bb0762",
    "1df2a190-8990-11ee-bec7-83d9deca7e3c",
    "20749180-8990-11ee-bec7-23e99a0a4339",
    "22139b80-8990-11ee-bec7-b10470991053",
    "235961f0-8990-11ee-bec7-317e033999e9",
    "24889320-8990-11ee-bec7-cf378b66fb63",
    "25b134a0-8990-11ee-bec7-c55024c91577",
    "276fd490-8990-11ee-bec7-29fb36a20f8d",
    "28a2fd60-8990-11ee-bec7-0b57a23c555e",
    "29b72c80-8990-11ee-bec7-f753d14a721b",
    "2b786480-8990-11ee-bec7-97692d863dba",
];

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

fn iid_from_map_indices(x_index: i32, y_index: i32) -> String {
    let index = map_indices_to_index(x_index, y_index);
    if index >= IIDS.len() {
        return IIDS[0].to_string();
    }
    IIDS[index].to_string()
}

fn spawn_world_borders(mut commands: Commands) {
    commands.spawn((
        Collider::cuboid(10.0, 2600.0),
        TransformBundle::from_transform(Transform::from_translation(Vec3::new(0.0, 2600.0, 0.0))),
    ));
    commands.spawn((
        Collider::cuboid(10.0, 2600.0),
        TransformBundle::from_transform(Transform::from_translation(Vec3::new(
            5120.0, 2600.0, 0.0,
        ))),
    ));
    commands.spawn((
        Collider::cuboid(2600.0, 10.0),
        TransformBundle::from_transform(Transform::from_translation(Vec3::new(2610.0, 55.0, 0.0))),
    ));
    commands.spawn((
        Collider::cuboid(2600.0, 10.0),
        TransformBundle::from_transform(Transform::from_translation(Vec3::new(
            2610.0, 5120.0, 0.0,
        ))),
    ));
}

fn spawn_ldtk_world(mut commands: Commands, assets: Res<GameAssets>) {
    commands.spawn(LdtkWorldBundle {
        transform: Transform::from_translation(Vec3::new(0.0, 0.0, -BACKGROUND_ZINDEX_ABS)),
        ldtk_handle: assets.level.clone(),
        ..Default::default()
    });
}

fn adjust_chunks(mut level_set: Query<&mut LevelSet>, q_player: Query<&Transform, With<Player>>) {
    let player_pos = match q_player.get_single() {
        Ok(p) => p.translation,
        Err(_) => return,
    };
    let mut level_set = match level_set.get_single_mut() {
        Ok(l) => l,
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
    let mut iids: Vec<String> = Vec::new();

    for k in 0..indices.len() {
        let i = indices[k].0;
        let j = indices[k].1;
        let iid = iid_from_map_indices(i, j);
        let level = LevelIid::new(&iid);
        iids.push(iid);

        if !level_set.iids.contains(&level) {
            level_set.iids.insert(level);
        }
    }

    for level_iid in &level_set.iids.clone() {
        if !iids.contains(&level_iid.get()) {
            level_set.iids.remove(level_iid);
        }
    }
}

pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(LdtkPlugin)
            .insert_resource(LdtkSettings {
                level_spawn_behavior: LevelSpawnBehavior::UseWorldTranslation {
                    load_level_neighbors: false,
                },
                ..default()
            })
            .add_systems(
                OnEnter(GameState::Gaming),
                (spawn_world_borders, spawn_ldtk_world),
            )
            .add_systems(Update, (adjust_chunks).run_if(in_state(GameState::Gaming)));
    }
}
