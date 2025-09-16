use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use super::state::DemonBossState;
use super::DemonBoss;
use crate::enemy::Enemy;
use crate::spell::aer_tracto::AerTracto;
use crate::spell::fireball::Fireball;
use crate::spell::icicle::Icicle;
use crate::spell::lightning::Lightning;
use crate::spell::lightning_bird::LightningStrike;
use crate::ui::health::Health;

fn fireball_collisions(
    mut q_enemies: Query<(&DemonBoss, &mut Health)>,
    mut q_fireballs: Query<&mut Fireball>,
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

        let (demon_boss, mut health) = if let Ok(h) = q_enemies.get_mut(source_parent) {
            h
        } else if let Ok(h) = q_enemies.get_mut(target_parent) {
            h
        } else {
            continue;
        };

        if demon_boss.state == DemonBossState::Dying {
            continue;
        }

        let mut fireball = if let Ok(f) = q_fireballs.get_mut(source_parent) {
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

        health.health -= fireball.damage;
    }
}

fn lightning_collisions(
    mut q_enemies: Query<(&DemonBoss, &mut Health)>,
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

        let (demon_boss, mut health) = if let Ok(h) = q_enemies.get_mut(source_parent) {
            h
        } else if let Ok(h) = q_enemies.get_mut(target_parent) {
            h
        } else {
            continue;
        };

        if demon_boss.state == DemonBossState::Dying {
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

        health.health -= damage;
    }
}

fn icicle_collisions(
    mut q_enemies: Query<(&DemonBoss, &mut Health)>,
    q_icicles: Query<&Icicle>,
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

        let (demon_boss, mut health) = if let Ok(h) = q_enemies.get_mut(source_parent) {
            h
        } else if let Ok(h) = q_enemies.get_mut(target_parent) {
            h
        } else {
            continue;
        };

        if demon_boss.state == DemonBossState::Dying {
            continue;
        }

        let icicle = if let Ok(i) = q_icicles.get(source_parent) {
            i
        } else if let Ok(i) = q_icicles.get(target_parent) {
            i
        } else {
            continue;
        };

        health.health -= icicle.damage;
    }
}

fn aer_tracto_collisions(
    mut q_enemies: Query<(&mut DemonBoss, &mut Health, &mut Velocity)>,
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

        let (mut demon_boss, mut health, mut velocity) =
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
        let mul = if demon_boss.rage.active { 0.25 } else { 0.5 };
        velocity.linvel = mul * dir * aer_tracto.pull_intensity;
        health.health -= aer_tracto.damage;

        if !demon_boss.rage.active {
            demon_boss.state = DemonBossState::Staggering;
            demon_boss.rage.add();
        }
    }
}

pub struct DemonBossCollisionPlugin;

impl Plugin for DemonBossCollisionPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                fireball_collisions,
                lightning_collisions,
                icicle_collisions,
                aer_tracto_collisions,
            ),
        );
    }
}
