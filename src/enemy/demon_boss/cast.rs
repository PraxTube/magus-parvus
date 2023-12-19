use bevy::prelude::*;

use super::{state::DemonBossState, DemonBoss};
use crate::GameState;

#[derive(Clone, PartialEq)]
pub enum DemonSpell {
    Explosion,
}

#[derive(Component, Clone)]
pub struct DemonSpellCast {
    pub spell: DemonSpell,
    pub timer: Timer,
}

#[derive(Event, Deref, DerefMut)]
pub struct SpawnDemonSpell(pub DemonSpellCast);

#[derive(Component)]
pub struct DemonSpellCooldown {
    #[allow(dead_code)]
    spell: DemonSpell,
    timer: Timer,
}

fn spawn_demon_spell(
    mut commands: Commands,
    q_demon_boss: Query<&DemonBoss>,
    q_demon_spells: Query<&DemonSpellCast>,
    q_demon_spell_cooldowns: Query<&DemonSpellCooldown>,
) {
    let demon_boss = match q_demon_boss.get_single() {
        Ok(r) => r,
        Err(_) => return,
    };

    if demon_boss.state != DemonBossState::Casting {
        return;
    }
    if !q_demon_spells.is_empty() {
        return;
    }
    if !q_demon_spell_cooldowns.is_empty() {
        return;
    }

    commands.spawn(DemonSpellCooldown {
        spell: DemonSpell::Explosion,
        timer: Timer::from_seconds(5.0, TimerMode::Once),
    });
    commands.spawn(DemonSpellCast {
        spell: DemonSpell::Explosion,
        timer: Timer::from_seconds(1.5, TimerMode::Once),
    });
}

fn relay_demon_spells(
    mut commands: Commands,
    time: Res<Time>,
    mut q_demon_spells: Query<(Entity, &mut DemonSpellCast)>,
    mut ev_spawn_demon_spell: EventWriter<SpawnDemonSpell>,
) {
    for (entity, mut demon_spell) in &mut q_demon_spells {
        demon_spell.timer.tick(time.delta());
        if demon_spell.timer.just_finished() {
            ev_spawn_demon_spell.send(SpawnDemonSpell(demon_spell.clone()));
            commands.entity(entity).despawn_recursive();
        }
    }
}

fn despawn_spell_cooldowns(
    mut commands: Commands,
    time: Res<Time>,
    mut q_demon_spell_cooldowns: Query<(Entity, &mut DemonSpellCooldown)>,
) {
    for (entity, mut cooldown) in &mut q_demon_spell_cooldowns {
        cooldown.timer.tick(time.delta());
        if cooldown.timer.just_finished() {
            commands.entity(entity).despawn_recursive();
        }
    }
}

pub struct DemonBossCastPlugin;

impl Plugin for DemonBossCastPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                spawn_demon_spell,
                despawn_spell_cooldowns,
                relay_demon_spells,
            )
                .run_if(in_state(GameState::Gaming)),
        )
        .add_event::<SpawnDemonSpell>();
    }
}
