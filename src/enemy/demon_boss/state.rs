use bevy::prelude::*;
use bevy_trickfilm::prelude::*;

use crate::{player::Player, GameAssets, GameState};

use super::DemonBoss;

#[derive(Debug, Default, PartialEq, Clone, Copy)]
pub enum DemonBossState {
    #[default]
    Idling,
    Moving,
}

fn update_animation(
    assets: Res<GameAssets>,
    mut q_demon_boss: Query<(&mut AnimationPlayer2D, &DemonBoss)>,
) {
    let (mut animator, demon_boss) = match q_demon_boss.get_single_mut() {
        Ok(r) => r,
        Err(_) => return,
    };

    let (clip, repeat) = match demon_boss.state {
        DemonBossState::Idling => (assets.enemy_boss_animations[0].clone(), true),
        DemonBossState::Moving => (assets.enemy_boss_animations[2].clone(), true),
    };

    if repeat {
        animator.play(clip).repeat();
    } else {
        animator.play(clip);
    }
}

fn adjust_sprite_flip(
    mut q_demon_boss: Query<(&Transform, &mut TextureAtlasSprite), With<DemonBoss>>,
    q_player: Query<&Transform, (With<Player>, Without<DemonBoss>)>,
) {
    let (demon_boss_transform, mut sprite) = match q_demon_boss.get_single_mut() {
        Ok(r) => r,
        Err(_) => return,
    };
    let player_transform = match q_player.get_single() {
        Ok(r) => r,
        Err(_) => return,
    };

    sprite.flip_x = player_transform.translation.x - demon_boss_transform.translation.x > 0.0;
}

pub struct DemonBossStatePlugin;

impl Plugin for DemonBossStatePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (update_animation, adjust_sprite_flip).run_if(in_state(GameState::Gaming)),
        );
    }
}
