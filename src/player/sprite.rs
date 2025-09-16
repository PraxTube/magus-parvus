use bevy::prelude::*;

use crate::utils::anim_sprite::AnimationIndices;
use crate::GameState;

use super::{Player, PlayerState};

fn player_sprite_indicies(state: &PlayerState) -> (usize, usize) {
    match state {
        PlayerState::Idling => (0, 5),
        PlayerState::Moving => (6, 11),
        PlayerState::Casting => (12, 17),
        PlayerState::SpellBook => (0, 5),
        PlayerState::Staggering => (18, 18),
    }
}

fn update_indicies(mut q_player: Query<(&mut AnimationIndices, &mut Sprite, &Player)>) {
    let (mut indices, mut image, player) = match q_player.single_mut() {
        Ok(p) => (p.0, p.1, p.2),
        Err(_) => return,
    };

    let new_indices = player_sprite_indicies(&player.state);

    if new_indices.0 != indices.first {
        indices.first = new_indices.0;
        indices.last = new_indices.1;
        if let Some(layout) = image.texture_atlas.as_mut() {
            layout.index = indices.first;
        }
    }
}

fn adjust_sprite_flip(mut q_player: Query<(&mut Sprite, &Player)>) {
    let (mut sprite, player) = match q_player.single_mut() {
        Ok(p) => (p.0, p.1),
        Err(_) => return,
    };
    if player.current_direction.x == 0.0 {
        return;
    }

    sprite.flip_x = player.current_direction.x < 0.0;
}

pub struct PlayerSpritePlugin;

impl Plugin for PlayerSpritePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (update_indicies, adjust_sprite_flip).run_if(in_state(GameState::Gaming)),
        );
    }
}
