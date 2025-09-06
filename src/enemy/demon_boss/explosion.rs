use std::f32::consts::PI;

use rand::{thread_rng, Rng};

use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use bevy_trickfilm::prelude::*;

use crate::{player::Player, world::camera::YSort, GameAssets, GameState};

use super::{
    cast::{DemonSpell, SpawnDemonSpell},
    state::DemonBossState,
    strike::DemonBossStrike,
    DemonBoss,
};

const RAND_OFFSET_INTENSITY: f32 = 150.0;
const COUNT: usize = 10;

#[derive(Component)]
pub struct DemonBossExplosion {
    pub damage: f32,
    activation_timer: Timer,
}

#[derive(Component)]
pub struct DemonBossStrikeExplosion {
    pub damage: f32,
}

#[derive(Component, Deref, DerefMut)]
struct ColliderTimer(Timer);

#[derive(Component)]
struct ExplosionDelayTimer {
    timer: Timer,
    pos: Vec3,
    normal_explosion: bool,
}

fn spawn_normal_explosion(commands: &mut Commands, assets: &Res<GameAssets>, pos: Vec3) {
    let mut animator = AnimationPlayer2D::default();
    animator
        .play(assets.demon_boss_explosion_animations[0].clone())
        .repeat();

    commands.spawn((
        DemonBossExplosion {
            damage: 1.0,
            activation_timer: Timer::from_seconds(2.0, TimerMode::Once),
        },
        animator,
        YSort(1.0),
        Sprite::from_atlas_image(
            assets.demon_boss_explosion_texture.clone(),
            TextureAtlas {
                layout: assets.demon_boss_explosion_layout.clone(),
                ..default()
            },
        ),
        Transform::from_translation(pos).with_scale(Vec3::splat(2.0)),
    ));
}

fn spawn_strike_explosion(commands: &mut Commands, assets: &Res<GameAssets>, pos: Vec3) {
    let mut animator = AnimationPlayer2D::default();
    animator.play(assets.demon_boss_explosion2_animations[0].clone());

    let collider = commands
        .spawn((
            ColliderTimer(Timer::from_seconds(0.3, TimerMode::Once)),
            Sensor,
            Collider::ball(20.0),
            CollisionGroups::default(),
            Transform::default(),
        ))
        .id();

    commands
        .spawn((
            DemonBossStrikeExplosion { damage: 2.0 },
            animator,
            YSort(0.0),
            Sprite::from_atlas_image(
                assets.demon_boss_explosion2_texture.clone(),
                TextureAtlas {
                    layout: assets.demon_boss_explosion2_layout.clone(),
                    ..default()
                },
            ),
            Transform::from_translation(pos).with_scale(Vec3::splat(2.5)),
        ))
        .add_children(&[collider]);
}

fn spawn_explosions(
    mut commands: Commands,
    assets: Res<GameAssets>,
    time: Res<Time>,
    mut q_explosion_delays: Query<&mut ExplosionDelayTimer>,
) {
    for mut timer in &mut q_explosion_delays {
        timer.timer.tick(time.delta());
        if !timer.timer.just_finished() {
            continue;
        }

        if timer.normal_explosion {
            spawn_normal_explosion(&mut commands, &assets, timer.pos);
        } else {
            spawn_strike_explosion(&mut commands, &assets, timer.pos);
        }
    }
}

fn spawn_explosion_delays(
    mut commands: Commands,
    q_player: Query<&Transform, With<Player>>,
    mut ev_spawn_demon_spells: EventReader<SpawnDemonSpell>,
) {
    let player_pos = match q_player.get_single() {
        Ok(r) => r.translation,
        Err(_) => return,
    };

    let mut rng = thread_rng();

    for ev in ev_spawn_demon_spells.read() {
        if ev.spell != DemonSpell::Explosion {
            continue;
        }

        for i in 0..COUNT {
            let pos = player_pos
                + Vec3::new(
                    rng.gen_range(-1.0..1.0) * RAND_OFFSET_INTENSITY,
                    rng.gen_range(-1.0..1.0) * RAND_OFFSET_INTENSITY,
                    0.0,
                );

            commands.spawn(ExplosionDelayTimer {
                timer: Timer::from_seconds(0.1 * i as f32, TimerMode::Once),
                pos,
                normal_explosion: true,
            });
        }
    }
}

fn change_animations(
    mut commands: Commands,
    assets: Res<GameAssets>,
    time: Res<Time>,
    mut q_explosions: Query<(Entity, &mut AnimationPlayer2D, &mut DemonBossExplosion)>,
) {
    for (entity, mut animator, mut explosion) in &mut q_explosions {
        explosion.activation_timer.tick(time.delta());
        if explosion.activation_timer.just_finished() {
            let collider = commands
                .spawn((
                    ColliderTimer(Timer::from_seconds(0.2, TimerMode::Once)),
                    Sensor,
                    Collider::ball(15.0),
                    CollisionGroups::default(),
                    Transform::default(),
                ))
                .id();
            commands.entity(entity).add_children(&[collider]);
            animator.play(assets.demon_boss_explosion_animations[1].clone());
        }
    }
}

fn despawn_explosions(
    mut commands: Commands,
    q_explosions: Query<(Entity, &AnimationPlayer2D, &DemonBossExplosion)>,
) {
    for (entity, animator, explosion) in &q_explosions {
        if explosion.activation_timer.finished() && animator.finished() {
            commands.entity(entity).despawn_recursive();
        }
    }
}

fn despawn_strike_explosions(
    mut commands: Commands,
    q_explosions: Query<(Entity, &AnimationPlayer2D), With<DemonBossStrikeExplosion>>,
) {
    for (entity, animator) in &q_explosions {
        if animator.finished() {
            commands.entity(entity).despawn_recursive();
        }
    }
}

fn despawn_colliders(
    mut commands: Commands,
    time: Res<Time>,
    mut q_colliders: Query<(Entity, &mut ColliderTimer)>,
) {
    for (entity, mut timer) in &mut q_colliders {
        timer.tick(time.delta());
        if timer.just_finished() {
            commands.entity(entity).despawn_recursive();
        }
    }
}

fn spawn_strike_explosions(
    mut commands: Commands,
    q_demon_boss: Query<(&Transform, &DemonBoss, &Sprite)>,
    mut q_strike_hitbox: Query<&mut DemonBossStrike>,
) {
    let (transform, demon_boss, sprite) = match q_demon_boss.get_single() {
        Ok(r) => r,
        Err(_) => return,
    };
    let mut strike = match q_strike_hitbox.get_single_mut() {
        Ok(r) => r,
        Err(_) => return,
    };

    if demon_boss.state != DemonBossState::Striking {
        return;
    }

    if !strike.striked || strike.spawned_explosions {
        return;
    }
    strike.spawned_explosions = true;

    let num = 5;
    let dis = 100.0;
    let delay = 0.1;
    let offset = Vec3::new(0.0, -40.0, 0.0);
    for i in 0..num {
        let dir = if sprite.flip_x {
            Quat::from_rotation_z(PI * i as f32 / num as f32).mul_vec3(Vec3::X)
        } else {
            Quat::from_rotation_z(PI - PI * i as f32 / num as f32).mul_vec3(Vec3::X)
        };
        let pos = transform.translation + dis * dir + offset;

        commands.spawn(ExplosionDelayTimer {
            timer: Timer::from_seconds(delay * i as f32, TimerMode::Once),
            pos,
            normal_explosion: false,
        });

        let dir = if sprite.flip_x {
            Quat::from_rotation_z(2.0 * PI - PI * i as f32 / num as f32).mul_vec3(Vec3::X)
        } else {
            Quat::from_rotation_z(PI + PI * i as f32 / num as f32).mul_vec3(Vec3::X)
        };
        let pos = transform.translation + dis * dir + offset;

        commands.spawn(ExplosionDelayTimer {
            timer: Timer::from_seconds(delay * i as f32, TimerMode::Once),
            pos,
            normal_explosion: false,
        });
    }
}

pub struct DemonBossExplosionPlugin;

impl Plugin for DemonBossExplosionPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                spawn_explosions,
                spawn_explosion_delays,
                spawn_strike_explosions,
                despawn_explosions,
                despawn_strike_explosions,
                despawn_colliders,
                change_animations,
            )
                .run_if(in_state(GameState::Gaming)),
        );
    }
}
