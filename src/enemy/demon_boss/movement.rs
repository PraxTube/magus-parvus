use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::{player::Player, GameState};

use super::{DemonBoss, DemonBossState, MOVE_SPEED};

#[derive(Component, Deref, DerefMut)]
pub struct MovementCooldownTimer(pub Timer);

fn movement(
    mut q_demon_boss: Query<(&Transform, &mut Velocity, &DemonBoss)>,
    q_player: Query<&Transform, (With<Player>, Without<DemonBoss>)>,
) {
    let (demon_boss_transform, mut velocity, demon_boss) = match q_demon_boss.get_single_mut() {
        Ok(p) => p,
        Err(_) => return,
    };
    let player_transform = match q_player.get_single() {
        Ok(p) => p,
        Err(_) => return,
    };

    if demon_boss.state == DemonBossState::Staggering {
        return;
    }
    if demon_boss.state != DemonBossState::Moving {
        velocity.linvel = Vec2::ZERO;
        return;
    }

    let direction = (player_transform.translation - demon_boss_transform.translation)
        .truncate()
        .normalize_or_zero();
    let mul = if demon_boss.rage.active { 2.0 } else { 1.0 };
    velocity.linvel = direction * mul * MOVE_SPEED;
}

fn despawn_cooldowns(
    mut commands: Commands,
    time: Res<Time>,
    mut q_timers: Query<(Entity, &mut MovementCooldownTimer)>,
) {
    for (entity, mut timer) in &mut q_timers {
        timer.tick(time.delta());
        if timer.just_finished() {
            commands.entity(entity).despawn_recursive();
        }
    }
}

pub struct DemonBossMovementPlugin;

impl Plugin for DemonBossMovementPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (movement, despawn_cooldowns).run_if(in_state(GameState::Gaming)),
        );
    }
}
