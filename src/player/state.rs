use bevy::prelude::*;

use crate::GameState;

use super::Player;

#[derive(Default, PartialEq, Clone, Copy)]
pub enum PlayerState {
    #[default]
    Idling,
    Moving,
    Casting,
}

#[derive(Event)]
pub struct PlayerChangedState {
    pub old_state: PlayerState,
    pub new_state: PlayerState,
}

fn player_changed_state(
    q_player: Query<&Player>,
    mut ev_changed_state: EventWriter<PlayerChangedState>,
    mut old_state: Local<PlayerState>,
) {
    let player = match q_player.get_single() {
        Ok(p) => p,
        Err(_) => return,
    };

    if player.state != *old_state {
        ev_changed_state.send(PlayerChangedState {
            old_state: *old_state,
            new_state: player.state,
        });
        *old_state = player.state;
    }
}

fn switch_player_mode(keys: Res<Input<KeyCode>>, mut q_player: Query<&mut Player>) {
    let mut player = match q_player.get_single_mut() {
        Ok(p) => p,
        Err(_) => return,
    };

    match player.state {
        PlayerState::Idling => {
            if keys.just_pressed(KeyCode::I) {
                player.state = PlayerState::Casting;
            }
        }
        PlayerState::Moving => {
            if keys.just_pressed(KeyCode::I) {
                player.state = PlayerState::Casting;
            }
        }
        PlayerState::Casting => {
            if keys.just_pressed(KeyCode::Escape) {
                player.state = PlayerState::Idling;
            }
        }
    }
}

pub struct PlayerStatePlugin;

impl Plugin for PlayerStatePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (switch_player_mode, player_changed_state).run_if(in_state(GameState::Gaming)),
        )
        .add_event::<PlayerChangedState>();
    }
}
