use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use super::{Enemy, SlimeEnemy, SlimeState, STAGGERING_INTENSITY};
use crate::player::Player;
use crate::spell::aer_tracto::AerTracto;
use crate::spell::fireball::Fireball;
use crate::spell::icicle::Icicle;
use crate::spell::lightning::Lightning;
use crate::spell::lightning_bird::LightningStrike;
use crate::ui::health::Health;

fn player_collisions(
    q_player: Query<(&Transform, &Player), With<Player>>,
    mut q_enemies: Query<(&Transform, &mut SlimeEnemy, &mut Velocity), Without<Player>>,
    q_colliders: Query<&ChildOf, (With<Collider>, Without<Enemy>, Without<Player>)>,
    mut ev_collision_events: EventReader<CollisionEvent>,
) {
    let (player_transform, player) = match q_player.single() {
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
                Ok(c) => c.parent(),
                Err(_) => continue,
            }
        } else if &player.collider_entity == target {
            match q_colliders.get(*source) {
                Ok(c) => c.parent(),
                Err(_) => continue,
            }
        } else {
            continue;
        };

        let (slime_transform, mut slime, mut velocity) = match q_enemies.get_mut(enemy_parent) {
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

fn fireball_collisions(
    mut q_enemies: Query<(&Transform, &mut SlimeEnemy, &mut Health, &mut Velocity)>,
    mut q_fireballs: Query<(&Transform, &mut Fireball)>,
    q_colliders: Query<&ChildOf, (With<Collider>, Without<Enemy>)>,
    mut ev_collision_events: EventReader<CollisionEvent>,
) {
    for ev in ev_collision_events.read() {
        let (source, target) = match ev {
            CollisionEvent::Started(source, target, _) => (source, target),
            CollisionEvent::Stopped(_, _, _) => continue,
        };

        let source_parent = match q_colliders.get(*source) {
            Ok(c) => c.parent(),
            Err(_) => continue,
        };
        let target_parent = match q_colliders.get(*target) {
            Ok(c) => c.parent(),
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

        if fireball.disabled() {
            continue;
        }
        fireball.disable();

        let dir = (slime_transform.translation - fireball_transform.translation)
            .truncate()
            .normalize_or_zero();
        velocity.linvel = dir * STAGGERING_INTENSITY;
        slime_health.health -= fireball.damage;
        slime.state = SlimeState::Staggering;
    }
}

fn lightning_collisions(
    mut q_enemies: Query<(&mut SlimeEnemy, &mut Health, &mut Velocity)>,
    q_lightnings: Query<&Lightning>,
    q_lightning_strikes: Query<&LightningStrike>,
    q_colliders: Query<&ChildOf, (With<Collider>, Without<Enemy>)>,
    mut ev_collision_events: EventReader<CollisionEvent>,
) {
    for ev in ev_collision_events.read() {
        let (source, target) = match ev {
            CollisionEvent::Started(source, target, _) => (source, target),
            CollisionEvent::Stopped(_, _, _) => continue,
        };

        let source_parent = match q_colliders.get(*source) {
            Ok(c) => c.parent(),
            Err(_) => continue,
        };
        let target_parent = match q_colliders.get(*target) {
            Ok(c) => c.parent(),
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

        let damage = if let Ok(l) = q_lightnings.get(source_parent) {
            l.damage
        } else if let Ok(l) = q_lightnings.get(target_parent) {
            l.damage
        } else if let Ok(l) = q_lightning_strikes.get(source_parent) {
            l.damage
        } else if let Ok(l) = q_lightning_strikes.get(target_parent) {
            l.damage
        } else {
            continue;
        };

        slime_health.health -= damage;
        velocity.linvel = Vec2::ZERO;
        slime.state = SlimeState::Staggering;
    }
}

fn icicle_collisions(
    q_player: Query<&Transform, With<Player>>,
    mut q_enemies: Query<(&Transform, &mut SlimeEnemy, &mut Health, &mut Velocity)>,
    q_icicles: Query<&Icicle>,
    q_colliders: Query<&ChildOf, With<Collider>>,
    mut ev_collision_events: EventReader<CollisionEvent>,
) {
    let player_pos = match q_player.single() {
        Ok(p) => p.translation,
        Err(_) => return,
    };

    for ev in ev_collision_events.read() {
        let (source, target) = match ev {
            CollisionEvent::Started(source, target, _) => (source, target),
            CollisionEvent::Stopped(_, _, _) => continue,
        };

        let source_parent = match q_colliders.get(*source) {
            Ok(c) => c.parent(),
            Err(_) => continue,
        };
        let target_parent = match q_colliders.get(*target) {
            Ok(c) => c.parent(),
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

fn aer_tracto_collisions(
    mut q_enemies: Query<(&mut SlimeEnemy, &mut Health, &mut Velocity)>,
    q_aer_tractos: Query<(&Transform, &AerTracto)>,
    q_colliders: Query<&ChildOf, With<Collider>>,
    mut ev_collision_events: EventReader<CollisionEvent>,
) {
    for ev in ev_collision_events.read() {
        let (source, target) = match ev {
            CollisionEvent::Started(source, target, _) => (source, target),
            CollisionEvent::Stopped(_, _, _) => continue,
        };

        let source_parent = match q_colliders.get(*source) {
            Ok(c) => c.parent(),
            Err(_) => continue,
        };
        let target_parent = match q_colliders.get(*target) {
            Ok(c) => c.parent(),
            Err(_) => continue,
        };

        let (mut slime, mut slime_health, mut velocity) =
            if let Ok(s) = q_enemies.get_mut(source_parent) {
                s
            } else if let Ok(s) = q_enemies.get_mut(target_parent) {
                s
            } else {
                continue;
            };

        let (aer_tracto_transform, aer_tracto) = if let Ok(a) = q_aer_tractos.get(source_parent) {
            a
        } else if let Ok(a) = q_aer_tractos.get(target_parent) {
            a
        } else {
            continue;
        };

        let dir = -aer_tracto_transform.rotation.mul_vec3(Vec3::X).truncate();
        velocity.linvel = dir * aer_tracto.pull_intensity;
        slime_health.health -= aer_tracto.damage;
        slime.state = SlimeState::Staggering;
    }
}

pub struct SlimeCollisionPlugin;

impl Plugin for SlimeCollisionPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                player_collisions,
                fireball_collisions,
                lightning_collisions,
                icicle_collisions,
                aer_tracto_collisions,
            ),
        );
    }
}
