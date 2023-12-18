use bevy::prelude::*;
use bevy_trickfilm::prelude::*;

use crate::{player::Player, GameAssets, GameState};

use super::{cast::SpawnDemonSpell, DemonBoss, INV_CAST_RANGE, STRIKE_RANGE};

#[derive(Debug, Default, PartialEq, Clone, Copy)]
pub enum DemonBossState {
    #[default]
    Idling,
    Casting,
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
        DemonBossState::Casting => (assets.enemy_boss_animations[1].clone(), true),
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

fn striking_to_idle(mut q_demon_boss: Query<(&AnimationPlayer2D, &mut DemonBoss)>) {
    let (animator, mut demon_boss) = match q_demon_boss.get_single_mut() {
        Ok(r) => r,
        Err(_) => return,
    };

    if demon_boss.state == DemonBossState::Striking && animator.is_finished() {
        demon_boss.state = DemonBossState::Idling;
    }
}

fn casting_to_idle(
    mut q_demon_boss: Query<&mut DemonBoss>,
    mut ev_spawn_demon_spell: EventReader<SpawnDemonSpell>,
) {
    let mut demon_boss = match q_demon_boss.get_single_mut() {
        Ok(r) => r,
        Err(_) => return,
    };

    if ev_spawn_demon_spell.is_empty() {
        return;
    }

    ev_spawn_demon_spell.clear();
    demon_boss.state = DemonBossState::Idling;
}

fn switch_state(
    mut q_demon_boss: Query<(&Transform, &mut DemonBoss)>,
    q_player: Query<&Transform, (With<Player>, Without<DemonBoss>)>,
) {
    let (demon_boss_transform, mut demon_boss) = match q_demon_boss.get_single_mut() {
        Ok(p) => p,
        Err(_) => return,
    };
    let player_pos = match q_player.get_single() {
        Ok(p) => p.translation,
        Err(_) => return,
    };

    if demon_boss.state == DemonBossState::Striking || demon_boss.state == DemonBossState::Casting {
        return;
    }

    let dis = player_pos
        .truncate()
        .distance_squared(demon_boss_transform.translation.truncate());
    let strike_range = STRIKE_RANGE.powi(2);
    let inv_cast_range = INV_CAST_RANGE.powi(2);

    if dis <= strike_range {
        demon_boss.state = DemonBossState::Striking;
    } else if dis >= inv_cast_range {
        demon_boss.state = DemonBossState::Casting;
    } else {
        demon_boss.state = DemonBossState::Moving;
    }
}

pub struct DemonBossStatePlugin;

impl Plugin for DemonBossStatePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                update_animation,
                adjust_sprite_flip,
                striking_to_idle,
                casting_to_idle,
                switch_state,
            )
                .run_if(in_state(GameState::Gaming)),
        );
    }
}
