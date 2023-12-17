use std::f32::consts::TAU;

use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use bevy_trickfilm::animation::AnimationPlayer2D;

use crate::{audio::PlaySound, world::camera::YSort, GameAssets, GameState};

use super::{
    item_value::item_wall_offset,
    statue::{StatueTriggered, StatueUnlocked},
};

const WALL_PADDING: f32 = 10.0;

#[derive(Component, Default)]
struct EarthWall {
    despawning: bool,
}
#[derive(Component)]
struct EarthWallCollider;

fn wall_count(offset: f32) -> usize {
    offset as usize / 4
}

fn spawn_wall(commands: &mut Commands, assets: &Res<GameAssets>, pos: Vec3, flip_x: bool) {
    let mut animator = AnimationPlayer2D::default();
    animator.play(assets.earth_wall_animations[0].clone());

    commands.spawn((
        EarthWall::default(),
        animator,
        YSort(0.0),
        SpriteSheetBundle {
            transform: Transform::from_translation(pos),
            texture_atlas: assets.earth_wall.clone(),
            sprite: TextureAtlasSprite {
                flip_x,
                ..default()
            },
            ..default()
        },
    ));
}

fn spawn_collider(commands: &mut Commands, pos: Vec3, offset: f32) {
    let mut vertices = Vec::new();
    for i in 0..wall_count(offset) {
        let rot = Quat::from_rotation_z(TAU * i as f32 / wall_count(offset) as f32);
        let pos = rot.mul_vec3(Vec3::X * offset - WALL_PADDING);
        vertices.push(Vect::new(pos.x, pos.y));
    }
    vertices.push(Vect::new(offset - WALL_PADDING, 0.0));

    commands.spawn((
        EarthWallCollider,
        Collider::polyline(vertices, None),
        CollisionGroups::default(),
        TransformBundle::from_transform(Transform::from_translation(pos)),
    ));
}

fn spawn_statue_walls(
    mut commands: Commands,
    assets: Res<GameAssets>,
    mut ev_statue_triggered: EventReader<StatueTriggered>,
    mut ev_play_sound: EventWriter<PlaySound>,
) {
    for ev in ev_statue_triggered.read() {
        let offset = item_wall_offset(&ev.statue.item);
        let count = wall_count(offset);

        if count > 0 {
            ev_play_sound.send(PlaySound {
                clip: assets.earth_wall_sound.clone(),
                ..default()
            });
        }

        spawn_collider(&mut commands, ev.statue.pos, offset);
        for i in 0..wall_count(offset) {
            let rot = Quat::from_rotation_z(TAU * i as f32 / wall_count(offset) as f32);
            let pos = ev.statue.pos + rot.mul_vec3(Vec3::X * offset);
            let flip_x = rot.to_euler(EulerRot::ZYX).0.abs() < TAU / 4.0;
            spawn_wall(&mut commands, &assets, pos, flip_x);
        }
    }
}

fn despawn_statue_walls(
    mut commands: Commands,
    q_earth_walls: Query<(Entity, &EarthWall, &AnimationPlayer2D)>,
) {
    for (entity, earth_wall, animator) in &q_earth_walls {
        if !earth_wall.despawning {
            continue;
        }

        if animator.is_finished() {
            commands.entity(entity).despawn_recursive();
        }
    }
}

fn despawn_earth_wall_colliders(
    mut commands: Commands,
    q_earth_wall_colliders: Query<Entity, With<EarthWallCollider>>,
    mut ev_statue_unlocked: EventReader<StatueUnlocked>,
) {
    if ev_statue_unlocked.is_empty() {
        return;
    }
    ev_statue_unlocked.clear();

    for entity in &q_earth_wall_colliders {
        commands.entity(entity).despawn_recursive();
    }
}

fn play_despawn_animation(
    assets: Res<GameAssets>,
    mut q_earth_walls: Query<(&mut EarthWall, &mut AnimationPlayer2D)>,
    mut ev_statue_unlocked: EventReader<StatueUnlocked>,
) {
    if ev_statue_unlocked.is_empty() {
        return;
    }
    ev_statue_unlocked.clear();

    for (mut earth_wall, mut animator) in &mut q_earth_walls {
        if earth_wall.despawning {
            continue;
        }

        earth_wall.despawning = true;
        animator.play(assets.earth_wall_animations[1].clone());
    }
}

pub struct StatueWallsPlugin;

impl Plugin for StatueWallsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                spawn_statue_walls,
                despawn_statue_walls,
                despawn_earth_wall_colliders,
                play_despawn_animation,
            )
                .run_if(in_state(GameState::Gaming)),
        );
    }
}
