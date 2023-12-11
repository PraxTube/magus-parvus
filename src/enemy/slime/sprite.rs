use bevy::prelude::*;

use super::{SlimeEnemy, SlimeState};
use crate::utils::anim_sprite::AnimationIndices;

fn slime_sprite_indices(state: &SlimeState) -> (usize, usize) {
    match state {
        SlimeState::Idling => (0, 5),
        SlimeState::Jumping => (6, 11),
        SlimeState::Staggering => (18, 18),
        SlimeState::Dying => (12, 17),
    }
}

fn update_indicies(
    mut q_slimes: Query<(&mut AnimationIndices, &mut TextureAtlasSprite, &SlimeEnemy)>,
) {
    for (mut indices, mut sprite, slime) in &mut q_slimes {
        let new_indices = slime_sprite_indices(&slime.state);

        if new_indices.0 != indices.first {
            indices.first = new_indices.0;
            indices.last = new_indices.1;
            sprite.index = indices.first;
        }
    }
}

pub struct SlimeSpritePlugin;

impl Plugin for SlimeSpritePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (update_indicies,));
    }
}
