use rand::{thread_rng, Rng};

use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use bevy_trickfilm::prelude::*;

use crate::{player::Player, world::camera::YSort, GameAssets, GameState};

use super::cast::{DemonSpell, SpawnDemonSpell};

const RAND_OFFSET_INTENSITY: f32 = 150.0;
const COUNT: usize = 10;

#[derive(Component)]
pub struct DemonBossExplosion {
    pub damage: f32,
    activation_timer: Timer,
}

#[derive(Component, Deref, DerefMut)]
struct ColliderTimer(Timer);

#[derive(Component)]
struct ExplosionDelayTimer {
    timer: Timer,
    pos: Vec3,
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
            SpriteSheetBundle {
                texture_atlas: assets.demon_boss_explosion.clone(),
                transform: Transform::from_translation(timer.pos).with_scale(Vec3::splat(2.0)),
                ..default()
            },
        ));
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
                    TransformBundle::default(),
                ))
                .id();
            commands.entity(entity).push_children(&[collider]);
            animator.play(assets.demon_boss_explosion_animations[1].clone());
        }
    }
}

fn despawn_explosions(
    mut commands: Commands,
    q_explosions: Query<(Entity, &AnimationPlayer2D, &DemonBossExplosion)>,
) {
    for (entity, animator, explosion) in &q_explosions {
        if explosion.activation_timer.finished() && animator.is_finished() {
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

pub struct DemonBossExplosionPlugin;

impl Plugin for DemonBossExplosionPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                spawn_explosions,
                spawn_explosion_delays,
                despawn_explosions,
                despawn_colliders,
                change_animations,
            )
                .run_if(in_state(GameState::Gaming)),
        );
    }
}
