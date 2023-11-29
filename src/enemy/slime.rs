use std::time::Duration;

use rand::{self, Rng};

use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::ui::health::Health;
use crate::utils::anim_sprite::{AnimationIndices, FrameTimer};
use crate::world::game_entity::SpawnGameEntity;
use crate::{player::Player, GameAssets, GameState};

use super::Enemy;

const MAX_JUMP_SPEED: f32 = 200.0;
const RANDOM_OFFSET_INTENSITY: f32 = 0.25;
const JUMP_TIME: f32 = 0.5;

#[derive(Default, PartialEq, Clone, Copy)]
pub enum SlimeState {
    #[default]
    Idling,
    Jumping,
    Dying,
}

#[derive(Component)]
struct SlimeEnemy {
    state: SlimeState,
    jump_speed: f32,
    jump_direction: Vec3,
    jumping_timer: Timer,
    jump_cooldown_timer: Timer,
    death_timer: Timer,
}

impl Default for SlimeEnemy {
    fn default() -> Self {
        Self {
            state: SlimeState::Idling,
            jump_speed: MAX_JUMP_SPEED,
            jump_direction: Vec3::default(),
            jumping_timer: Timer::new(Duration::from_secs_f32(JUMP_TIME), TimerMode::Repeating),
            jump_cooldown_timer: Timer::new(Duration::from_secs_f32(3.5), TimerMode::Repeating),
            death_timer: Timer::new(Duration::from_secs_f32(0.070 * 6.0), TimerMode::Once),
        }
    }
}

fn slime_sprite_indices(state: &SlimeState) -> (usize, usize) {
    match state {
        SlimeState::Idling => (0, 5),
        SlimeState::Jumping => (6, 11),
        SlimeState::Dying => (12, 17),
    }
}

fn update_indicies(
    mut q_slimes: Query<(&mut AnimationIndices, &mut TextureAtlasSprite, &SlimeEnemy)>,
) {
    for (mut indices, mut sprite, slime) in &mut q_slimes {
        let new_indices = slime_sprite_indices(&slime.state);

        if new_indices.0 != indices.first {
            indices.first = new_indices.0;
            indices.last = new_indices.1;
            sprite.index = indices.first;
        }
    }
}

fn spawn_slimes(
    mut commands: Commands,
    assets: Res<GameAssets>,
    mut ev_spawn_game_entity: EventWriter<SpawnGameEntity>,
) {
    let entity = commands
        .spawn((
            Enemy,
            SlimeEnemy::default(),
            AnimationIndices { first: 0, last: 5 },
            FrameTimer(Timer::from_seconds(0.085, TimerMode::Repeating)),
            SpriteSheetBundle {
                transform: Transform::from_translation(Vec3::new(32.0 * 32.0, 32.0 * 32.0, 0.0))
                    .with_scale(Vec3::splat(1.5)),
                texture_atlas: assets.enemy.clone(),
                ..default()
            },
        ))
        .id();

    let health = Health::new(entity, 10.0, 0.60);
    ev_spawn_game_entity.send(SpawnGameEntity { entity, health });

    let collider = commands
        .spawn((
            Collider::ball(6.0),
            // CollisionGroups::new(
            //     Group::from_bits(0b0100).unwrap(),
            //     Group::from_bits(0b1000).unwrap(),
            // ),
            TransformBundle::from_transform(Transform::from_translation(Vec3::new(
                0.0, -10.0, 0.0,
            ))),
        ))
        .id();

    commands.entity(entity).push_children(&[collider]);
}

fn tick_slime_timers(time: Res<Time>, mut q_slimes: Query<&mut SlimeEnemy, With<Enemy>>) {
    for mut slime in &mut q_slimes {
        if slime.state == SlimeState::Idling {
            slime.jump_cooldown_timer.tick(time.delta());
            if slime.jump_cooldown_timer.just_finished() {
                slime.state = SlimeState::Jumping;
            }
        } else if slime.state == SlimeState::Jumping {
            slime.jumping_timer.tick(time.delta());
            if slime.jumping_timer.just_finished() {
                slime.state = SlimeState::Idling;
            }
        } else if slime.state == SlimeState::Dying {
            slime.death_timer.tick(time.delta());
        }
    }
}

fn update_jump_position(
    mut q_slimes: Query<(&Transform, &mut SlimeEnemy), (With<Enemy>, Without<Player>)>,
    q_player: Query<&Transform, With<Player>>,
) {
    let player_transform = match q_player.get_single() {
        Ok(p) => p,
        Err(_) => return,
    };
    for (enemy_transform, mut slime) in &mut q_slimes {
        if slime.state == SlimeState::Jumping {
            continue;
        }

        let distance = player_transform
            .translation
            .distance(enemy_transform.translation);
        let ratio = (distance / MAX_JUMP_SPEED / JUMP_TIME).min(1.0);

        let mut rng = rand::thread_rng();
        let dir = (player_transform.translation - enemy_transform.translation).normalize_or_zero();
        let random_offset = Vec3::new(rng.gen_range(-1.0..1.0), rng.gen_range(-1.0..1.0), 0.0)
            * RANDOM_OFFSET_INTENSITY;

        slime.jump_speed = ratio * MAX_JUMP_SPEED;
        slime.jump_direction = dir + random_offset;
    }
}

fn move_slimes(time: Res<Time>, mut q_enemies: Query<(&mut Transform, &SlimeEnemy)>) {
    for (mut enemy_transform, slime) in &mut q_enemies {
        if slime.state != SlimeState::Jumping {
            continue;
        }
        enemy_transform.translation +=
            slime.jump_direction * slime.jump_speed * time.delta_seconds();
    }
}

fn damage_slimes(time: Res<Time>, mut q_slimes: Query<&mut Health, With<SlimeEnemy>>) {
    for mut health in &mut q_slimes {
        health.health -= 0.2 * time.delta_seconds();
    }
}

fn despawn_slimes(mut commands: Commands, mut q_slimes: Query<(Entity, &Health, &mut SlimeEnemy)>) {
    for (entity, health, mut slime) in &mut q_slimes {
        if health.health <= 0.0 {
            slime.state = SlimeState::Dying;
        }
        if slime.death_timer.just_finished() {
            commands.entity(entity).despawn_recursive();
        }
    }
}

pub struct SlimeEnemyPlugin;

impl Plugin for SlimeEnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                update_indicies,
                tick_slime_timers,
                move_slimes,
                update_jump_position,
                damage_slimes,
                despawn_slimes,
            )
                .run_if(in_state(GameState::Gaming)),
        )
        .add_systems(OnEnter(GameState::Gaming), spawn_slimes);
    }
}
