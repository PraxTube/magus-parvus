use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::enemy::demon_boss::DemonBoss;
use crate::ui::health::Health;
use crate::{enemy::Enemy, GameState};

use super::{Player, PlayerState, STAGGERING_INTENSITY};

fn slime_collisions(
    mut q_player: Query<(&mut Velocity, &mut Player, &mut Health, &Transform)>,
    q_enemies: Query<(&Transform, &Enemy), Without<Player>>,
    q_colliders: Query<&Parent, (With<Collider>, Without<Enemy>, Without<Player>)>,
    mut ev_collision_events: EventReader<CollisionEvent>,
) {
    let (mut velocity, mut player, mut health, player_transform) = match q_player.get_single_mut() {
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

        let (enemy_transform, enemy) = match q_enemies.get(enemy_parent.get()) {
            Ok(e) => (e.0, e.1),
            Err(_) => continue,
        };

        health.health -= enemy.damage;
        player.state = PlayerState::Staggering;

        let dir = (player_transform.translation - enemy_transform.translation)
            .truncate()
            .normalize_or_zero();
        // This makes the player look towards the impact
        player.current_direction = -dir;
        velocity.linvel = dir * STAGGERING_INTENSITY;
    }
}

fn demon_boss_collisions(
    mut q_player: Query<(&mut Velocity, &mut Player, &mut Health, &Transform)>,
    q_demon_boss: Query<(Entity, &Transform, &DemonBoss), Without<Player>>,
    q_colliders: Query<&Parent, (With<Collider>, Without<DemonBoss>, Without<Player>)>,
    mut ev_collision_events: EventReader<CollisionEvent>,
) {
    let (mut velocity, mut player, mut health, player_transform) = match q_player.get_single_mut() {
        Ok(r) => r,
        Err(_) => return,
    };
    let (demon_boss_entity, enemy_transform, demon_boss) = match q_demon_boss.get_single() {
        Ok(r) => r,
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

        if demon_boss_entity != enemy_parent.get() {
            continue;
        }

        health.health -= demon_boss.damage;
        player.state = PlayerState::Staggering;

        let dir = (player_transform.translation - enemy_transform.translation)
            .truncate()
            .normalize_or_zero();
        // This makes the player look towards the impact
        player.current_direction = -dir;
        velocity.linvel = dir * STAGGERING_INTENSITY;
    }
}

pub struct PlayerCollisionPlugin;

impl Plugin for PlayerCollisionPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (slime_collisions, demon_boss_collisions).run_if(in_state(GameState::Gaming)),
        );
    }
}
