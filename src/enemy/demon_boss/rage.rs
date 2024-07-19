use bevy::prelude::*;

use crate::{world::camera::YSort, GameAssets, GameState};

use super::{state::DemonBossState, DemonBoss};

#[derive(Component)]
pub struct RageAura;

fn spawn_rage_aura(
    mut commands: Commands,
    assets: Res<GameAssets>,
    q_demon_boss: Query<(Entity, &DemonBoss)>,
    q_demon_boss_aura: Query<&RageAura>,
) {
    if !q_demon_boss_aura.is_empty() {
        return;
    }

    let (entity, demon_boss) = match q_demon_boss.get_single() {
        Ok(r) => r,
        Err(_) => return,
    };
    if !demon_boss.rage.active {
        return;
    }
    if demon_boss.state == DemonBossState::Dying {
        return;
    }

    let aura = commands
        .spawn((
            RageAura,
            YSort(-0.5),
            SpriteBundle {
                texture: assets.demon_boss_aura.clone(),
                ..default()
            },
        ))
        .id();

    commands.entity(entity).push_children(&[aura]);
}

fn despawn_rage_aura(
    mut commands: Commands,
    q_demon_boss: Query<&DemonBoss>,
    q_demon_boss_aura: Query<Entity, With<RageAura>>,
) {
    let demon_boss = match q_demon_boss.get_single() {
        Ok(r) => r,
        Err(_) => return,
    };

    if demon_boss.state != DemonBossState::Dying && demon_boss.rage.active {
        return;
    }

    for entity in &q_demon_boss_aura {
        commands.entity(entity).despawn_recursive();
    }
}

fn tick_timer(time: Res<Time>, mut q_demon_boss: Query<&mut DemonBoss>) {
    let mut demon_boss = match q_demon_boss.get_single_mut() {
        Ok(r) => r,
        Err(_) => return,
    };
    if !demon_boss.rage.active {
        return;
    }

    demon_boss.rage.timer.tick(time.delta());
    if demon_boss.rage.timer.just_finished() {
        demon_boss.rage.active = false;
    }
}

pub struct DemonBossRagePlugin;

impl Plugin for DemonBossRagePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (spawn_rage_aura, despawn_rage_aura, tick_timer).run_if(in_state(GameState::Gaming)),
        );
    }
}
