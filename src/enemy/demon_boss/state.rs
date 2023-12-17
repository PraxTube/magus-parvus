use bevy::prelude::*;
use bevy_trickfilm::prelude::*;

use crate::{player::Player, GameAssets, GameState};

use super::DemonBoss;

#[derive(Debug, Default, PartialEq, Clone, Copy)]
pub enum DemonBossState {
    #[default]
    Idling,
    Moving,
    Striking,
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
        DemonBossState::Striking => (assets.enemy_boss_animations[3].clone(), false),
    };

    if repeat {
        animator.play(clip).repeat();
    } else {
        animator.play(clip);
    }
}

fn adjust_sprite_flip(
    mut q_demon_boss: Query<(&Transform, &mut TextureAtlasSprite, &DemonBoss)>,
    q_player: Query<&Transform, (With<Player>, Without<DemonBoss>)>,
) {
    let (demon_boss_transform, mut sprite, demon_boss) = match q_demon_boss.get_single_mut() {
        Ok(r) => r,
        Err(_) => return,
    };
    let player_transform = match q_player.get_single() {
        Ok(r) => r,
        Err(_) => return,
    };

    if demon_boss.state == DemonBossState::Striking {
        return;
    }

    sprite.flip_x = player_transform.translation.x - demon_boss_transform.translation.x > 0.0;
}

fn switch_to_idle(mut q_demon_boss: Query<(&AnimationPlayer2D, &mut DemonBoss)>) {
    let (animator, mut demon_boss) = match q_demon_boss.get_single_mut() {
        Ok(r) => r,
        Err(_) => return,
    };

    if demon_boss.state == DemonBossState::Striking && animator.is_finished() {
        demon_boss.state = DemonBossState::Idling;
    }
}

pub struct DemonBossStatePlugin;

impl Plugin for DemonBossStatePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (update_animation, adjust_sprite_flip, switch_to_idle)
                .run_if(in_state(GameState::Gaming)),
        );
    }
}
