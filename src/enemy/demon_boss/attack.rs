use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use bevy_trickfilm::prelude::*;

use crate::GameState;

use super::{DemonBoss, DemonBossState};

pub const STRIKE_HITBOX_OFFSET: Vec3 = Vec3::new(100.0, -25.0, 0.0);
const STRIKE_HITBOX_START: f32 = 1.2;
const STRIKE_HITBOX_TIME: f32 = 0.2;
const ACTIVE_GROUPS: CollisionGroups = CollisionGroups::new(Group::ALL, Group::ALL);
const INACTIVE_GROUPS: CollisionGroups = CollisionGroups::new(Group::NONE, Group::NONE);

#[derive(Component)]
pub struct StrikeHitbox;

fn toggle_hitbox(
    q_demon_boss: Query<(&DemonBoss, &AnimationPlayer2D)>,
    mut q_strike_hitbox: Query<&mut CollisionGroups, With<StrikeHitbox>>,
) {
    let (demon_boss, animator) = match q_demon_boss.get_single() {
        Ok(r) => r,
        Err(_) => return,
    };
    let mut collision_groups = match q_strike_hitbox.get_single_mut() {
        Ok(r) => r,
        Err(_) => return,
    };

    if demon_boss.state != DemonBossState::Striking {
        return;
    }

    if animator.elapsed() >= STRIKE_HITBOX_START
        && animator.elapsed() <= STRIKE_HITBOX_START + STRIKE_HITBOX_TIME
    {
        *collision_groups = ACTIVE_GROUPS;
    } else {
        *collision_groups = INACTIVE_GROUPS;
    }
}

fn update_hitbox_position(
    q_demon_boss: Query<&TextureAtlasSprite, With<DemonBoss>>,
    mut q_strike_hitbox: Query<&mut Transform, With<StrikeHitbox>>,
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
            (toggle_hitbox, update_hitbox_position).run_if(in_state(GameState::Gaming)),
        );
    }
}
