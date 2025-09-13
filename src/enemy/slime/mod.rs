mod collision;
mod movement;
mod spawn;
mod sprite;

use bevy::prelude::*;

use super::Enemy;
use crate::audio::PlaySound;
use crate::{GameAssets, GameState};

const MAX_JUMP_SPEED: f32 = 200.0;
const RANDOM_OFFSET_INTENSITY: f32 = 0.25;
const JUMP_TIME: f32 = 0.5;

const STAGGERING_TIME: f32 = 0.2;
const STAGGERING_INTENSITY: f32 = 100.0;

pub struct EnemySlimePlugin;

impl Plugin for EnemySlimePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            spawn::SlimeSpawnPlugin,
            movement::SlimeMovementPlugin,
            sprite::SlimeSpritePlugin,
            collision::SlimeCollisionPlugin,
        ))
        .add_event::<SpawnSlimeEnemy>()
        .add_systems(
            Update,
            (change_slime_states)
                .run_if(in_state(GameState::Gaming).or(in_state(GameState::GameOver))),
        );
    }
}

#[derive(Default, PartialEq, Clone, Copy)]
pub enum SlimeState {
    #[default]
    Idling,
    Jumping,
    Staggering,
    Dying,
}

#[derive(Component)]
struct SlimeEnemy {
    state: SlimeState,
    jump_speed: f32,
    jump_direction: Vec2,
    jumping_timer: Timer,
    jump_cooldown_timer: Timer,
    death_timer: Timer,
    staggering_timer: Timer,
    disabled: bool,
}

#[derive(Event)]
pub struct SpawnSlimeEnemy {
    pub pos: Vec3,
}

impl Default for SlimeEnemy {
    fn default() -> Self {
        Self {
            state: SlimeState::Idling,
            jump_speed: MAX_JUMP_SPEED,
            jump_direction: Vec2::ZERO,
            jumping_timer: Timer::from_seconds(JUMP_TIME, TimerMode::Repeating),
            jump_cooldown_timer: Timer::from_seconds(3.5, TimerMode::Repeating),
            death_timer: Timer::from_seconds(0.070 * 6.0, TimerMode::Once),
            staggering_timer: Timer::from_seconds(STAGGERING_TIME, TimerMode::Repeating),
            disabled: false,
        }
    }
}

fn change_slime_states(
    assets: Res<GameAssets>,
    time: Res<Time>,
    mut q_slimes: Query<(Entity, &mut SlimeEnemy), With<Enemy>>,
    mut ev_play_sound: EventWriter<PlaySound>,
) {
    for (entity, mut slime) in &mut q_slimes {
        match slime.state {
            SlimeState::Idling => {
                slime.jump_cooldown_timer.tick(time.delta());
                if slime.jump_cooldown_timer.just_finished() {
                    ev_play_sound.write(PlaySound {
                        clip: assets.slime_jump_sound.clone(),
                        volume: 0.75,
                        rand_speed_intensity: 0.2,
                        parent: Some(entity),
                        ..default()
                    });
                    slime.state = SlimeState::Jumping;
                }
            }
            SlimeState::Jumping => {
                slime.jumping_timer.tick(time.delta());
                if slime.jumping_timer.just_finished() {
                    ev_play_sound.write(PlaySound {
                        clip: assets.slime_land_sound.clone(),
                        rand_speed_intensity: 0.2,
                        parent: Some(entity),
                        ..default()
                    });
                    slime.state = SlimeState::Idling;
                }
            }
            SlimeState::Staggering => {
                slime.staggering_timer.tick(time.delta());
                if slime.staggering_timer.just_finished() {
                    ev_play_sound.write(PlaySound {
                        clip: assets.slime_hit_sound.clone(),
                        parent: Some(entity),
                        ..default()
                    });
                    slime.state = SlimeState::Idling;
                }
            }
            SlimeState::Dying => {
                slime.death_timer.tick(time.delta());
                if slime.death_timer.just_finished() {
                    slime.disabled = true;
                }
            }
        };
    }
}
