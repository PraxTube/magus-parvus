use rand::{self, Rng};

use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use super::Enemy;
use crate::audio::PlaySound;
use crate::spell::fireball::Fireball;
use crate::spell::icicle::Icicle;
use crate::spell::lightning::Lightning;
use crate::ui::health::Health;
use crate::utils::anim_sprite::{AnimationIndices, FrameTimer};
use crate::world::camera::YSort;
use crate::{player::Player, GameAssets, GameState};

const MAX_JUMP_SPEED: f32 = 200.0;
const RANDOM_OFFSET_INTENSITY: f32 = 0.25;
const JUMP_TIME: f32 = 0.5;

const STAGGERING_TIME: f32 = 0.2;
const STAGGERING_INTENSITY: f32 = 100.0;

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

fn slime_sprite_indices(state: &SlimeState) -> (usize, usize) {
    match state {
        SlimeState::Idling => (0, 5),
        SlimeState::Jumping => (6, 11),
        SlimeState::Staggering => (18, 18),
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

fn spawn_slime(
    commands: &mut Commands,
    assets: &Res<GameAssets>,
    spawn_pos: Vec3,
    ev_play_sound: &mut EventWriter<PlaySound>,
) {
    let entity = commands
        .spawn((
            RigidBody::Dynamic,
            LockedAxes::ROTATION_LOCKED,
            Velocity::zero(),
            Enemy { damage: 1.0 },
            SlimeEnemy::default(),
            AnimationIndices { first: 0, last: 5 },
            FrameTimer(Timer::from_seconds(0.085, TimerMode::Repeating)),
            YSort(0.0),
            SpriteSheetBundle {
                transform: Transform::from_translation(spawn_pos).with_scale(Vec3::splat(1.5)),
                texture_atlas: assets.enemy.clone(),
                ..default()
            },
        ))
        .id();

    let collider = commands
        .spawn((
            Collider::ball(6.0),
            ActiveEvents::COLLISION_EVENTS,
            // CollisionGroups::new(
            //     Group::from_bits(0b0100).unwrap(),
            //     Group::from_bits(0b1000).unwrap(),
            // ),
            TransformBundle::from_transform(Transform::from_translation(Vec3::new(
                0.0, -10.0, 0.0,
            ))),
        ))
        .id();

    commands
        .entity(entity)
        .insert(Health::new(entity, 10.0, 0.60))
        .push_children(&[collider]);

    ev_play_sound.send(PlaySound {
        clip: assets.slime_land_sound.clone(),
        rand_speed_intensity: 0.2,
        ..default()
    });
}

fn spawn_slimes(
    mut commands: Commands,
    assets: Res<GameAssets>,
    mut ev_spawn_slime_enemy: EventReader<SpawnSlimeEnemy>,
    mut ev_play_sound: EventWriter<PlaySound>,
) {
    for ev in ev_spawn_slime_enemy.read() {
        spawn_slime(&mut commands, &assets, ev.pos, &mut ev_play_sound);
    }
}

fn tick_slime_timers(
    assets: Res<GameAssets>,
    time: Res<Time>,
    mut q_slimes: Query<&mut SlimeEnemy, With<Enemy>>,
    mut ev_play_sound: EventWriter<PlaySound>,
) {
    for mut slime in &mut q_slimes {
        match slime.state {
            SlimeState::Idling => {
                slime.jump_cooldown_timer.tick(time.delta());
                if slime.jump_cooldown_timer.just_finished() {
                    ev_play_sound.send(PlaySound {
                        clip: assets.slime_jump_sound.clone(),
                        volume: 0.75,
                        rand_speed_intensity: 0.2,
                        ..default()
                    });
                    slime.state = SlimeState::Jumping;
                }
            }
            SlimeState::Jumping => {
                slime.jumping_timer.tick(time.delta());
                if slime.jumping_timer.just_finished() {
                    ev_play_sound.send(PlaySound {
                        clip: assets.slime_land_sound.clone(),
                        rand_speed_intensity: 0.2,
                        ..default()
                    });
                    slime.state = SlimeState::Idling;
                }
            }
            SlimeState::Staggering => {
                slime.staggering_timer.tick(time.delta());
                if slime.staggering_timer.just_finished() {
                    ev_play_sound.send(PlaySound {
                        clip: assets.slime_hit_sound.clone(),
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
            .truncate()
            .distance(enemy_transform.translation.truncate());
        let ratio = (distance / MAX_JUMP_SPEED / JUMP_TIME).min(1.0);

        let mut rng = rand::thread_rng();
        let dir = (player_transform.translation.truncate()
            - enemy_transform.translation.truncate())
        .normalize_or_zero();
        let random_offset =
            Vec2::new(rng.gen_range(-1.0..1.0), rng.gen_range(-1.0..1.0)) * RANDOM_OFFSET_INTENSITY;

        slime.jump_speed = ratio * MAX_JUMP_SPEED;
        slime.jump_direction = dir + random_offset;
    }
}

fn move_slimes(mut q_slimes: Query<(&mut Velocity, &SlimeEnemy)>) {
    for (mut velocity, slime) in &mut q_slimes {
        if slime.state == SlimeState::Staggering {
            continue;
        }
        if slime.state != SlimeState::Jumping {
            velocity.linvel = Vec2::ZERO;
            continue;
        }
        velocity.linvel = slime.jump_direction * slime.jump_speed;
    }
}

fn despawn_slimes(
    mut commands: Commands,
    assets: Res<GameAssets>,
    mut q_slimes: Query<(Entity, &Health, &mut SlimeEnemy)>,
    mut ev_play_sound: EventWriter<PlaySound>,
) {
    for (entity, health, mut slime) in &mut q_slimes {
        if health.health <= 0.0 && slime.state != SlimeState::Dying {
            ev_play_sound.send(PlaySound {
                clip: assets.slime_death_sound.clone(),
                ..default()
            });
            slime.state = SlimeState::Dying;
        }
        if slime.disabled {
            commands.entity(entity).despawn_recursive();
        }
    }
}

fn check_player_collision(
    q_player: Query<(&Transform, &Player), With<Player>>,
    mut q_enemies: Query<(&Transform, &mut SlimeEnemy, &mut Velocity), Without<Player>>,
    q_colliders: Query<&Parent, (With<Collider>, Without<Enemy>, Without<Player>)>,
    mut ev_collision_events: EventReader<CollisionEvent>,
) {
    let (player_transform, player) = match q_player.get_single() {
        Ok(p) => p,
        Err(_) => return,
    };

    for ev in ev_collision_events.read() {
        let (source, target) = match ev {
            CollisionEvent::Started(source, target, _) => (source, target),
            CollisionEvent::Stopped(_, _, _) => continue,
        };

        let enemy_parent = if &player.collider_entity == source {
            match q_colliders.get(*target) {
                Ok(parent) => parent,
                Err(_) => continue,
            }
        } else if &player.collider_entity == target {
            match q_colliders.get(*source) {
                Ok(parent) => parent,
                Err(_) => continue,
            }
        } else {
            continue;
        };

        let (slime_transform, mut slime, mut velocity) = match q_enemies.get_mut(enemy_parent.get())
        {
            Ok(e) => e,
            Err(_) => continue,
        };

        // Slime is jumping, don't apply any knockback
        if slime.state == SlimeState::Jumping {
            slime.jump_direction = Vec2::ZERO;
            continue;
        } else if slime.state == SlimeState::Dying {
            continue;
        }

        let dir = (slime_transform.translation - player_transform.translation)
            .truncate()
            .normalize_or_zero();
        velocity.linvel = dir * STAGGERING_INTENSITY;
        slime.jump_cooldown_timer.reset();
        slime.state = SlimeState::Staggering;
    }
}

fn check_fireball_collisions(
    mut q_enemies: Query<(&Transform, &mut SlimeEnemy, &mut Health, &mut Velocity)>,
    mut q_fireballs: Query<(&Transform, &mut Fireball)>,
    q_colliders: Query<&Parent, (With<Collider>, Without<Enemy>)>,
    mut ev_collision_events: EventReader<CollisionEvent>,
) {
    for ev in ev_collision_events.read() {
        let (source, target) = match ev {
            CollisionEvent::Started(source, target, _) => (source, target),
            CollisionEvent::Stopped(_, _, _) => continue,
        };

        let source_parent = match q_colliders.get(*source) {
            Ok(p) => p.get(),
            Err(_) => continue,
        };
        let target_parent = match q_colliders.get(*target) {
            Ok(p) => p.get(),
            Err(_) => continue,
        };

        let (slime_transform, mut slime, mut slime_health, mut velocity) =
            if let Ok(h) = q_enemies.get_mut(source_parent) {
                h
            } else if let Ok(h) = q_enemies.get_mut(target_parent) {
                h
            } else {
                continue;
            };

        if slime.state == SlimeState::Dying {
            continue;
        }

        let (fireball_transform, mut fireball) = if let Ok(f) = q_fireballs.get_mut(source_parent) {
            f
        } else if let Ok(f) = q_fireballs.get_mut(target_parent) {
            f
        } else {
            continue;
        };

        fireball.disabled = true;

        let dir = (slime_transform.translation - fireball_transform.translation)
            .truncate()
            .normalize_or_zero();
        velocity.linvel = dir * STAGGERING_INTENSITY;
        slime_health.health -= fireball.damage;
        slime.state = SlimeState::Staggering;
    }
}

fn check_lightning_collisions(
    mut q_enemies: Query<(&mut SlimeEnemy, &mut Health, &mut Velocity)>,
    q_lightnings: Query<&Lightning>,
    q_colliders: Query<&Parent, (With<Collider>, Without<Enemy>)>,
    mut ev_collision_events: EventReader<CollisionEvent>,
) {
    for ev in ev_collision_events.read() {
        let (source, target) = match ev {
            CollisionEvent::Started(source, target, _) => (source, target),
            CollisionEvent::Stopped(_, _, _) => continue,
        };

        let source_parent = match q_colliders.get(*source) {
            Ok(p) => p.get(),
            Err(_) => continue,
        };
        let target_parent = match q_colliders.get(*target) {
            Ok(p) => p.get(),
            Err(_) => continue,
        };

        let (mut slime, mut slime_health, mut velocity) =
            if let Ok(h) = q_enemies.get_mut(source_parent) {
                h
            } else if let Ok(h) = q_enemies.get_mut(target_parent) {
                h
            } else {
                continue;
            };

        if slime.state == SlimeState::Dying {
            continue;
        }

        let lightning = if let Ok(l) = q_lightnings.get(source_parent) {
            l
        } else if let Ok(l) = q_lightnings.get(target_parent) {
            l
        } else {
            continue;
        };

        velocity.linvel = Vec2::ZERO;
        slime_health.health -= lightning.damage;
        slime.state = SlimeState::Staggering;
    }
}

fn check_icicle_collisions(
    q_player: Query<&Transform, With<Player>>,
    mut q_enemies: Query<(&Transform, &mut SlimeEnemy, &mut Health, &mut Velocity)>,
    q_icicles: Query<&Icicle>,
    q_colliders: Query<&Parent, With<Collider>>,
    mut ev_collision_events: EventReader<CollisionEvent>,
) {
    let player_pos = match q_player.get_single() {
        Ok(p) => p.translation,
        Err(_) => return,
    };

    for ev in ev_collision_events.read() {
        let (source, target) = match ev {
            CollisionEvent::Started(source, target, _) => (source, target),
            CollisionEvent::Stopped(_, _, _) => continue,
        };

        let source_parent = match q_colliders.get(*source) {
            Ok(p) => p.get(),
            Err(_) => continue,
        };
        let target_parent = match q_colliders.get(*target) {
            Ok(p) => p.get(),
            Err(_) => continue,
        };

        let (slime_transform, mut slime, mut slime_health, mut velocity) =
            if let Ok(h) = q_enemies.get_mut(source_parent) {
                h
            } else if let Ok(h) = q_enemies.get_mut(target_parent) {
                h
            } else {
                continue;
            };

        if slime.state == SlimeState::Dying {
            continue;
        }

        let icicle = if let Ok(i) = q_icicles.get(source_parent) {
            i
        } else if let Ok(i) = q_icicles.get(target_parent) {
            i
        } else {
            continue;
        };

        let dir = (slime_transform.translation - player_pos)
            .truncate()
            .normalize_or_zero();
        velocity.linvel = dir * STAGGERING_INTENSITY;
        slime_health.health -= icicle.damage;
        slime.state = SlimeState::Staggering;
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
                spawn_slimes,
                despawn_slimes,
                check_player_collision,
                check_fireball_collisions,
                check_lightning_collisions,
                check_icicle_collisions,
            )
                .run_if(in_state(GameState::Gaming)),
        )
        .add_event::<SpawnSlimeEnemy>();
    }
}
