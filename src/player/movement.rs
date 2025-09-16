use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::GameState;

use super::input::PlayerInput;
use super::stats::Stats;
use super::{Player, PlayerState};

fn player_movement(
    mut q_player: Query<(&mut Velocity, &mut Player, &Stats)>,
    player_input: Res<PlayerInput>,
) {
    let (mut velocity, mut player, stats) = match q_player.single_mut() {
        Ok(p) => (p.0, p.1, p.2),
        Err(_) => return,
    };

    if player.state == PlayerState::Casting {
        velocity.linvel = Vec2::ZERO;
    }
    if player.state == PlayerState::SpellBook {
        velocity.linvel = Vec2::ZERO;
    }
    if player.state != PlayerState::Moving && player.state != PlayerState::Idling {
        return;
    }

    let direction = player_input.move_direction;
    if direction == Vec2::default() {
        player.state = PlayerState::Idling;
        velocity.linvel = Vec2::ZERO;
        return;
    }

    player.state = PlayerState::Moving;
    player.current_direction = direction;
    velocity.linvel = direction * stats.move_speed;
}

pub struct PlayerMovementPlugin;

impl Plugin for PlayerMovementPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (player_movement,).run_if(in_state(GameState::Gaming)),
        );
    }
}
