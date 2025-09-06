use std::f32::consts::{PI, TAU};

use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use bevy_trickfilm::prelude::*;

use crate::{audio::PlaySound, player::Player, world::camera::YSort, GameAssets, GameState};

use super::{
    cast::{DemonSpell, SpawnDemonSpell},
    DemonBoss,
};

const WALL_PADDING: f32 = 10.0;
const COUNT: usize = 25;
const RADIUS: f32 = 100.0;
const OFFSET: f32 = 50.0;

#[derive(Component, Default)]
struct DemonBossEarthWall {
    despawning: bool,
}
#[derive(Component)]
struct DemonBossEarthWallCollider {
    timer: Timer,
}

fn spawn_wall(commands: &mut Commands, assets: &Res<GameAssets>, pos: Vec3, flip_x: bool) {
    let mut animator = AnimationPlayer2D::default();
    animator.play(assets.earth_wall_animations[0].clone());

    commands.spawn((
        DemonBossEarthWall::default(),
        animator,
        YSort(0.0),
        Sprite {
            image: assets.earth_wall_texture.clone(),
            texture_atlas: Some(TextureAtlas {
                layout: assets.earth_wall_layout.clone(),
                ..default()
            }),
            flip_x,
            ..default()
        },
        Transform::from_translation(pos),
    ));
}

fn spawn_collider(commands: &mut Commands, pos: Vec3, offset: f32, angle: f32) {
    let mut vertices = Vec::new();
    for i in 0..COUNT {
        let rot = Quat::from_rotation_z(angle + 3.0 / 2.0 * PI * i as f32 / COUNT as f32);
        let pos = rot.mul_vec3(Vec3::X * offset - WALL_PADDING);
        vertices.push(Vect::new(pos.x, pos.y));
    }

    commands.spawn((
        DemonBossEarthWallCollider {
            timer: Timer::from_seconds(7.0, TimerMode::Once),
        },
        Collider::polyline(vertices, None),
        CollisionGroups::default(),
        Transform::from_translation(pos),
    ));
}

fn spawn_earth_prison(
    mut commands: Commands,
    assets: Res<GameAssets>,
    q_player: Query<&Transform, With<Player>>,
    q_demon_boss: Query<&Transform, (With<DemonBoss>, Without<Player>)>,
    mut ev_spawn_demon_spells: EventReader<SpawnDemonSpell>,
    mut ev_play_sound: EventWriter<PlaySound>,
) {
    let player_pos = match q_player.get_single() {
        Ok(r) => r.translation,
        Err(_) => return,
    };
    let demon_boss_pos = match q_demon_boss.get_single() {
        Ok(r) => r.translation,
        Err(_) => return,
    };

    let dis = demon_boss_pos - player_pos;
    let pos = player_pos + dis.normalize_or_zero() * OFFSET;
    let angle = if dis.y < 0.0 {
        2.0 * PI - dis.angle_between(Vec3::X) + PI / 4.0
    } else {
        dis.angle_between(Vec3::X) + PI / 4.0
    };

    for ev in ev_spawn_demon_spells.read() {
        if ev.spell != DemonSpell::EarthPrison {
            continue;
        }

        ev_play_sound.send(PlaySound {
            clip: assets.earth_wall_sound.clone(),
            ..default()
        });

        spawn_collider(&mut commands, pos, RADIUS, angle);
        for i in 0..COUNT {
            let rot = Quat::from_rotation_z(angle + 3.0 / 2.0 * PI * i as f32 / COUNT as f32);
            let pos = pos + rot.mul_vec3(Vec3::X * RADIUS);
            let flip_x = rot.to_euler(EulerRot::ZYX).0.abs() < TAU / 4.0;
            spawn_wall(&mut commands, &assets, pos, flip_x);
        }
    }
}

fn despawn_earth_prison(
    mut commands: Commands,
    assets: Res<GameAssets>,
    time: Res<Time>,
    mut q_earth_walls: Query<(&mut DemonBossEarthWall, &mut AnimationPlayer2D)>,
    mut q_earth_wall_colliders: Query<(Entity, &mut DemonBossEarthWallCollider)>,
) {
    let mut despawn = false;
    for (_, mut collider) in &mut q_earth_wall_colliders {
        collider.timer.tick(time.delta());
        if collider.timer.just_finished() {
            despawn = true;
        }
    }

    if !despawn {
        return;
    }

    for (entity, _) in &q_earth_wall_colliders {
        commands.entity(entity).despawn_recursive();
    }

    for (mut earth_wall, mut animator) in &mut q_earth_walls {
        if !earth_wall.despawning {
            earth_wall.despawning = true;
            animator.play(assets.earth_wall_animations[1].clone());
        }
    }
}

fn despawn_walls(
    mut commands: Commands,
    q_earth_walls: Query<(Entity, &DemonBossEarthWall, &AnimationPlayer2D)>,
) {
    for (entity, earth_wall, animator) in &q_earth_walls {
        if !earth_wall.despawning {
            continue;
        }

        if animator.finished() {
            commands.entity(entity).despawn_recursive();
        }
    }
}

pub struct DemonBossEarthPrisonPlugin;

impl Plugin for DemonBossEarthPrisonPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (spawn_earth_prison, despawn_earth_prison, despawn_walls)
                .run_if(in_state(GameState::Gaming)),
        );
    }
}
