use std::time::Duration;

use rand::{self, Rng};

use bevy::prelude::*;

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
}

#[derive(Component)]
struct SlimeEnemy {
    state: SlimeState,
    jump_speed: f32,
    jump_direction: Vec3,
    jumping_timer: Timer,
    jump_cooldown_timer: Timer,
}

impl Default for SlimeEnemy {
    fn default() -> Self {
        Self {
            state: SlimeState::Idling,
            jump_speed: MAX_JUMP_SPEED,
            jump_direction: Vec3::default(),
            jumping_timer: Timer::new(Duration::from_secs_f32(JUMP_TIME), TimerMode::Repeating),
            jump_cooldown_timer: Timer::new(Duration::from_secs_f32(3.5), TimerMode::Repeating),
        }
    }
}

fn spawn_slimes(mut commands: Commands, assets: Res<GameAssets>) {
    commands.spawn((
        Enemy,
        SlimeEnemy::default(),
        SpriteSheetBundle {
            transform: Transform::from_translation(Vec3::new(32.0 * 32.0, 32.0 * 32.0, 0.0))
                .with_scale(Vec3::splat(1.5)),
            texture_atlas: assets.enemy.clone(),
            ..default()
        },
    ));

    commands.spawn((
        Enemy,
        SlimeEnemy::default(),
        SpriteSheetBundle {
            transform: Transform::from_translation(Vec3::default()).with_scale(Vec3::splat(1.5)),
            texture_atlas: assets.enemy.clone(),
            ..default()
        },
    ));
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

fn move_slimes(time: Res<Time>, mut q_enemies: Query<(&mut Transform, &SlimeEnemy), With<Enemy>>) {
    for (mut enemy_transform, slime) in &mut q_enemies {
        if slime.state != SlimeState::Jumping {
            continue;
        }
        enemy_transform.translation +=
            slime.jump_direction * slime.jump_speed * time.delta_seconds();
    }
}

pub struct SlimeEnemyPlugin;

impl Plugin for SlimeEnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (tick_slime_timers, move_slimes, update_jump_position)
                .run_if(in_state(GameState::Gaming)),
        )
        .add_systems(OnEnter(GameState::Gaming), spawn_slimes);
    }
}
