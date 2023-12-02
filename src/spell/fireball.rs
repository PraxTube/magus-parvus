use std::f32::consts::TAU;

use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::enemy::Enemy;
use crate::player::Player;
use crate::utils::anim_sprite::{AnimSprite, AnimSpriteTimer};
use crate::utils::{quat_from_vec2, quat_from_vec3};
use crate::{GameAssets, GameState};

use super::{Spell, SpellCasted};

#[derive(Component)]
pub struct Fireball {
    pub disabled: bool,
    pub damage: f32,
}

impl Default for Fireball {
    fn default() -> Self {
        Self {
            disabled: false,
            damage: 1.0,
        }
    }
}

const SPEED: f32 = 300.0;
const SCALE: f32 = 1.5;
const SCALE_TIME: f32 = 0.35;
const DELTA_STEERING: f32 = 2.0;
const INFERNO_COUNT: usize = 50;

fn spawn_fireball(commands: &mut Commands, assets: &Res<GameAssets>, transform: Transform) {
    let entity = commands
        .spawn((
            Fireball::default(),
            SpriteSheetBundle {
                transform,
                texture_atlas: assets.fireball.clone(),
                ..default()
            },
            AnimSprite::new(60, true),
            AnimSpriteTimer::new(0.05),
        ))
        .id();

    let collider = commands
        .spawn((
            Collider::ball(5.0),
            Sensor,
            TransformBundle::from_transform(Transform::from_translation(Vec3::new(
                -25.0, 0.0, 0.0,
            ))),
        ))
        .id();

    commands.entity(entity).push_children(&[collider]);
}

fn spawn_fireballs(
    mut commands: Commands,
    assets: Res<GameAssets>,
    q_player: Query<(&Transform, &Player)>,
    mut ev_spell_casted: EventReader<SpellCasted>,
) {
    let (player_transform, player) = match q_player.get_single() {
        Ok(p) => (p.0, p.1),
        Err(_) => return,
    };

    for ev in ev_spell_casted.read() {
        if ev.spell == Spell::Fireball {
            let transform = Transform::from_translation(player_transform.translation)
                .with_scale(Vec3::splat(SCALE))
                .with_rotation(quat_from_vec2(-player.current_direction));
            spawn_fireball(&mut commands, &assets, transform);
        }
    }
}

fn spawn_ignis_pila(
    mut commands: Commands,
    assets: Res<GameAssets>,
    q_player: Query<&Transform, With<Player>>,
    mut ev_spell_casted: EventReader<SpellCasted>,
) {
    let player_pos = match q_player.get_single() {
        Ok(p) => p.translation,
        Err(_) => return,
    };

    for ev in ev_spell_casted.read() {
        if ev.spell == Spell::IgnisPila {
            for dir in [
                Vec2::new(1.0, 0.0),
                Vec2::new(0.0, 1.0),
                Vec2::new(1.0, 1.0),
                Vec2::new(-1.0, 1.0),
            ] {
                let transform = Transform::from_translation(player_pos)
                    .with_scale(Vec3::ZERO)
                    .with_rotation(quat_from_vec2(dir));
                spawn_fireball(&mut commands, &assets, transform);
                let transform = Transform::from_translation(player_pos)
                    .with_scale(Vec3::ZERO)
                    .with_rotation(quat_from_vec2(-dir));
                spawn_fireball(&mut commands, &assets, transform);
            }
        }
    }
}

fn spawn_inferno_pila(
    mut commands: Commands,
    assets: Res<GameAssets>,
    q_player: Query<&Transform, With<Player>>,
    mut ev_spell_casted: EventReader<SpellCasted>,
) {
    let player_pos = match q_player.get_single() {
        Ok(p) => p.translation,
        Err(_) => return,
    };

    for ev in ev_spell_casted.read() {
        if ev.spell == Spell::InfernoPila {
            for i in 0..INFERNO_COUNT {
                let transform = Transform::from_translation(player_pos)
                    .with_scale(Vec3::ZERO)
                    .with_rotation(Quat::from_rotation_z(TAU * i as f32 / INFERNO_COUNT as f32));
                spawn_fireball(&mut commands, &assets, transform);
            }
        }
    }
}

fn scale_fireballs(time: Res<Time>, mut q_fireballs: Query<&mut Transform, With<Fireball>>) {
    for mut transform in &mut q_fireballs {
        if transform.scale.x < SCALE {
            transform.scale += Vec3::ONE * SCALE / SCALE_TIME * time.delta_seconds();
        }
    }
}

fn move_fireballs(time: Res<Time>, mut q_fireballs: Query<&mut Transform, With<Fireball>>) {
    for mut transform in &mut q_fireballs {
        let dir = -transform.local_x();
        transform.translation += dir * SPEED * time.delta_seconds();
    }
}

fn steer_fireballs(
    time: Res<Time>,
    mut q_fireballs: Query<&mut Transform, (With<Fireball>, Without<Enemy>)>,
    q_enemies: Query<&Transform, With<Enemy>>,
) {
    for mut transform in &mut q_fireballs {
        let point_far_away = transform.translation - transform.local_x() * 100_000_000.0;
        let mut closest_enemy = Transform::from_translation(point_far_away);

        for enemey_transform in &q_enemies {
            if enemey_transform
                .translation
                .distance_squared(transform.translation)
                < closest_enemy
                    .translation
                    .distance_squared(transform.translation)
            {
                closest_enemy = enemey_transform.clone();
            }
        }

        let dir = transform.rotation.mul_vec3(Vec3::X);
        let target_dir = -closest_enemy.translation + transform.translation;
        let angle = dir.truncate().angle_between(target_dir.truncate());

        if angle.abs() < DELTA_STEERING * time.delta_seconds() {
            transform.rotation = quat_from_vec3(target_dir);
        } else {
            let sign = angle / angle.abs();
            transform.rotate_z(sign * time.delta_seconds() * DELTA_STEERING);
        }
    }
}

fn despawn_fireballs(mut commands: Commands, q_fireballs: Query<(Entity, &Fireball)>) {
    for (entity, fireball) in &q_fireballs {
        if fireball.disabled {
            commands.entity(entity).despawn_recursive();
        }
    }
}

pub struct FireballPlugin;

impl Plugin for FireballPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                spawn_fireballs,
                spawn_ignis_pila,
                spawn_inferno_pila,
                despawn_fireballs,
                scale_fireballs,
                move_fireballs,
                steer_fireballs,
            )
                .run_if(in_state(GameState::Gaming)),
        );
    }
}
