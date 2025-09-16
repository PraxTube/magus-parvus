use bevy::prelude::*;

use super::{input::PlayerInput, Player};

#[derive(Debug, Default, PartialEq, Clone, Copy)]
pub enum PlayerState {
    #[default]
    Idling,
    Moving,
    Casting,
    SpellBook,
    Staggering,
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
    let player = match q_player.single() {
        Ok(p) => p,
        Err(_) => return,
    };

    if player.state != *old_state {
        ev_changed_state.write(PlayerChangedState {
            old_state: *old_state,
            new_state: player.state,
        });
        *old_state = player.state;
    }
}

fn switch_player_mode(player_input: Res<PlayerInput>, mut q_player: Query<&mut Player>) {
    let mut player = match q_player.single_mut() {
        Ok(p) => p,
        Err(_) => return,
    };

    match player.state {
        PlayerState::Idling => {
            if player_input.casting {
                player.state = PlayerState::Casting;
            } else if player_input.toggle_spell_book {
                player.state = PlayerState::SpellBook;
            }
        }
        PlayerState::Moving => {
            if player_input.casting {
                player.state = PlayerState::Casting;
            } else if player_input.toggle_spell_book {
                player.state = PlayerState::SpellBook;
            }
        }
        PlayerState::Casting => {
            if player_input.escape {
                player.state = PlayerState::Idling;
            }
        }
        PlayerState::SpellBook => {
            if player_input.escape || player_input.toggle_spell_book {
                player.state = PlayerState::Idling;
            }
        }
        PlayerState::Staggering => {
            if player.staggering_timer.just_finished() {
                player.staggering_timer.reset();
                player.state = PlayerState::Idling;
            }
        }
    }
}

fn tick_staggering_timer(time: Res<Time>, mut q_player: Query<&mut Player>) {
    let mut player = match q_player.single_mut() {
        Ok(p) => p,
        Err(_) => return,
    };

    if player.state == PlayerState::Staggering {
        player.staggering_timer.tick(time.delta());
    }
}

pub struct PlayerStatePlugin;

impl Plugin for PlayerStatePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            PostUpdate,
            (
                switch_player_mode,
                player_changed_state.after(switch_player_mode),
                tick_staggering_timer,
            ),
        )
        .add_event::<PlayerChangedState>();
    }
}
