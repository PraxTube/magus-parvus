use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::{player::Player, GameState};

use super::{DemonBoss, DemonBossState, MOVE_SPEED};

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

    if demon_boss.state != DemonBossState::Moving {
        velocity.linvel = Vec2::ZERO;
        return;
    }

    let direction = (player_transform.translation - demon_boss_transform.translation)
        .truncate()
        .normalize_or_zero();
    velocity.linvel = direction * MOVE_SPEED;
}

pub struct DemonBossMovementPlugin;

impl Plugin for DemonBossMovementPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (movement,).run_if(in_state(GameState::Gaming)));
    }
}
