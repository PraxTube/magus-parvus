use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use bevy_trickfilm::prelude::*;

use crate::{utils::COLLISION_GROUPS_NONE, GameState};

use super::{DemonBoss, DemonBossState};

pub const STRIKE_HITBOX_OFFSET: Vec3 = Vec3::new(100.0, -25.0, 0.0);
const STRIKE_HITBOX_START: f32 = 1.2;
const STRIKE_HITBOX_TIME: f32 = 0.2;
const ACTIVE_GROUPS: CollisionGroups = CollisionGroups::new(Group::ALL, Group::ALL);

#[derive(Component, Default)]
pub struct DemonBossStrike {
    pub striked: bool,
    pub spawned_explosions: bool,
}

#[derive(Component)]
pub struct StrikeCooldown {
    timer: Timer,
}

fn spawn_strike_cooldown(
    mut commands: Commands,
    q_demon_boss: Query<&DemonBoss>,
    q_strike_cooldowns: Query<&StrikeCooldown>,
) {
    if !q_strike_cooldowns.is_empty() {
        return;
    }
    let demon_boss = match q_demon_boss.get_single() {
        Ok(r) => r,
        Err(_) => return,
    };
    if demon_boss.state != DemonBossState::Striking {
        return;
    }

    commands.spawn(StrikeCooldown {
        timer: Timer::from_seconds(5.0, TimerMode::Once),
    });
}

fn despawn_strike_cooldown(
    mut commands: Commands,
    time: Res<Time>,
    mut q_strike_cooldowns: Query<(Entity, &mut StrikeCooldown)>,
) {
    for (entity, mut cooldown) in &mut q_strike_cooldowns {
        cooldown.timer.tick(time.delta());
        if cooldown.timer.just_finished() {
            commands.entity(entity).despawn_recursive();
        }
    }
}

fn toggle_hitbox(
    q_demon_boss: Query<(&DemonBoss, &AnimationPlayer2D)>,
    mut q_strike_hitbox: Query<(&mut CollisionGroups, &mut DemonBossStrike)>,
) {
    let (demon_boss, animator) = match q_demon_boss.get_single() {
        Ok(r) => r,
        Err(_) => return,
    };
    let (mut collision_groups, mut strike) = match q_strike_hitbox.get_single_mut() {
        Ok(r) => r,
        Err(_) => return,
    };

    if demon_boss.state != DemonBossState::Striking {
        strike.striked = false;
        strike.spawned_explosions = false;
        return;
    }

    if animator.elapsed() >= STRIKE_HITBOX_START
        && animator.elapsed() <= STRIKE_HITBOX_START + STRIKE_HITBOX_TIME
    {
        strike.striked = true;
        *collision_groups = ACTIVE_GROUPS;
    } else {
        strike.striked = false;
        *collision_groups = COLLISION_GROUPS_NONE;
    }
}

fn update_hitbox_position(
    q_demon_boss: Query<&TextureAtlasSprite, With<DemonBoss>>,
    mut q_strike_hitbox: Query<&mut Transform, With<DemonBossStrike>>,
) {
    let sprite = match q_demon_boss.get_single() {
        Ok(r) => r,
        Err(_) => return,
    };
    let mut transform = match q_strike_hitbox.get_single_mut() {
        Ok(r) => r,
        Err(_) => return,
    };

    let sign = if sprite.flip_x { 1.0 } else { -1.0 };
    transform.translation = Vec3::new(
        sign * STRIKE_HITBOX_OFFSET.x,
        STRIKE_HITBOX_OFFSET.y,
        STRIKE_HITBOX_OFFSET.z,
    );
}

pub struct DemonBossAttackPlugin;

impl Plugin for DemonBossAttackPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                spawn_strike_cooldown,
                despawn_strike_cooldown,
                toggle_hitbox,
                update_hitbox_position,
            )
                .run_if(in_state(GameState::Gaming)),
        );
    }
}
