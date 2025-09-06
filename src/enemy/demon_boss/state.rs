use bevy::prelude::*;
use bevy_trickfilm::prelude::*;

use crate::{player::Player, GameAssets, GameState};

use super::{
    cast::{DemonSpellCooldown, LastSpellTimer, SpawnDemonSpell},
    movement::MovementCooldownTimer,
    strike::StrikeCooldown,
    DemonBoss, INV_CAST_RANGE, STRIKE_RANGE,
};

#[derive(Debug, Default, PartialEq, Clone, Copy)]
pub enum DemonBossState {
    #[default]
    Idling,
    Casting,
    Moving,
    Striking,
    Staggering,
    Dying,
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
        DemonBossState::Idling => (assets.demon_boss_animations[0].clone(), true),
        DemonBossState::Casting => (assets.demon_boss_animations[1].clone(), true),
        DemonBossState::Moving => (assets.demon_boss_animations[2].clone(), true),
        DemonBossState::Striking => (assets.demon_boss_animations[3].clone(), false),
        DemonBossState::Staggering => (assets.demon_boss_animations[4].clone(), false),
        DemonBossState::Dying => (assets.demon_boss_animations[5].clone(), false),
    };

    if repeat {
        animator.play(clip).repeat();
    } else {
        animator.play(clip);
    }
}

fn adjust_sprite_flip(
    mut q_demon_boss: Query<(&Transform, &mut Sprite, &DemonBoss)>,
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

    if demon_boss.state == DemonBossState::Striking
        || demon_boss.state == DemonBossState::Staggering
    {
        return;
    }

    sprite.flip_x = player_transform.translation.x - demon_boss_transform.translation.x > 0.0;
}

fn striking_to_idle(
    mut commands: Commands,
    mut q_demon_boss: Query<(&AnimationPlayer2D, &mut DemonBoss)>,
) {
    let (animator, mut demon_boss) = match q_demon_boss.get_single_mut() {
        Ok(r) => r,
        Err(_) => return,
    };
    if demon_boss.state != DemonBossState::Striking {
        return;
    }

    if animator.finished() {
        demon_boss.state = DemonBossState::Idling;
    }

    commands.spawn(MovementCooldownTimer(Timer::from_seconds(
        1.0,
        TimerMode::Once,
    )));
}

fn casting_to_idle(
    mut q_demon_boss: Query<&mut DemonBoss>,
    mut ev_spawn_demon_spell: EventReader<SpawnDemonSpell>,
) {
    let mut demon_boss = match q_demon_boss.get_single_mut() {
        Ok(r) => r,
        Err(_) => return,
    };
    if demon_boss.state != DemonBossState::Casting {
        return;
    }
    if ev_spawn_demon_spell.is_empty() {
        return;
    }
    ev_spawn_demon_spell.clear();
    demon_boss.state = DemonBossState::Idling;
}

fn staggering_to_idle(
    mut commands: Commands,
    mut q_demon_boss: Query<(&mut DemonBoss, &AnimationPlayer2D)>,
) {
    let (mut demon_boss, animator) = match q_demon_boss.get_single_mut() {
        Ok(r) => r,
        Err(_) => return,
    };
    if demon_boss.state != DemonBossState::Staggering {
        return;
    }

    if animator.finished() {
        demon_boss.state = DemonBossState::Idling;
    }

    commands.spawn(MovementCooldownTimer(Timer::from_seconds(
        0.6,
        TimerMode::Once,
    )));
}

fn switch_to_striking(
    mut q_demon_boss: Query<(&Transform, &mut DemonBoss)>,
    q_player: Query<&Transform, (With<Player>, Without<DemonBoss>)>,
    q_strike_cooldowns: Query<&StrikeCooldown>,
) {
    if !q_strike_cooldowns.is_empty() {
        return;
    }
    let (demon_boss_transform, mut demon_boss) = match q_demon_boss.get_single_mut() {
        Ok(p) => p,
        Err(_) => return,
    };
    let player_pos = match q_player.get_single() {
        Ok(p) => p.translation,
        Err(_) => return,
    };

    if demon_boss.state == DemonBossState::Casting
        || demon_boss.state == DemonBossState::Striking
        || demon_boss.state == DemonBossState::Dying
    {
        return;
    }

    let dis = player_pos
        .truncate()
        .distance_squared(demon_boss_transform.translation.truncate());
    let strike_range = STRIKE_RANGE.powi(2);

    if dis <= strike_range {
        demon_boss.state = DemonBossState::Striking;
    }
}

fn switch_to_casting(
    mut q_demon_boss: Query<(&Transform, &mut DemonBoss)>,
    q_player: Query<&Transform, (With<Player>, Without<DemonBoss>)>,
    q_demon_spell_cooldown: Query<&DemonSpellCooldown>,
    q_last_spell_timer: Query<&LastSpellTimer>,
) {
    if !q_demon_spell_cooldown.is_empty() {
        return;
    }
    let (demon_boss_transform, mut demon_boss) = match q_demon_boss.get_single_mut() {
        Ok(p) => p,
        Err(_) => return,
    };
    let player_pos = match q_player.get_single() {
        Ok(p) => p.translation,
        Err(_) => return,
    };
    let last_spell_timer = match q_last_spell_timer.get_single() {
        Ok(r) => r,
        Err(_) => return,
    };

    if demon_boss.state == DemonBossState::Casting
        || demon_boss.state == DemonBossState::Striking
        || demon_boss.state == DemonBossState::Dying
    {
        return;
    }

    let dis = player_pos
        .truncate()
        .distance_squared(demon_boss_transform.translation.truncate());
    let inv_cast_range = INV_CAST_RANGE.powi(2);

    if dis >= inv_cast_range || last_spell_timer.finished() {
        demon_boss.state = DemonBossState::Casting;
    }
}

fn switch_to_moving(
    mut q_demon_boss: Query<&mut DemonBoss>,
    q_movement_cooldowns: Query<&MovementCooldownTimer>,
) {
    if !q_movement_cooldowns.is_empty() {
        return;
    }
    let mut demon_boss = match q_demon_boss.get_single_mut() {
        Ok(p) => p,
        Err(_) => return,
    };

    if demon_boss.state != DemonBossState::Idling {
        return;
    }

    demon_boss.state = DemonBossState::Moving;
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
                staggering_to_idle,
                switch_to_striking,
                switch_to_casting,
                switch_to_moving,
            )
                .chain()
                .run_if(in_state(GameState::Gaming)),
        );
    }
}
