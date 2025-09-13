use rand::{thread_rng, Rng};

use bevy::prelude::*;

use super::{state::DemonBossState, DemonBoss};
use crate::GameState;

#[derive(Clone, PartialEq, Copy)]
pub enum DemonSpell {
    Explosion,
    EarthPrison,
}

#[derive(Component, Clone)]
pub struct DemonSpellCast {
    pub spell: DemonSpell,
    pub timer: Timer,
}

#[derive(Component)]
pub struct DemonSpellCooldown {
    #[allow(dead_code)]
    spell: DemonSpell,
    timer: Timer,
}

#[derive(Component, Deref, DerefMut)]
pub struct LastSpellTimer(Timer);

#[derive(Event, Deref, DerefMut)]
pub struct SpawnDemonSpell(pub DemonSpellCast);

fn pick_random_spell() -> DemonSpell {
    let mut rng = thread_rng();

    match rng.gen_range(0..2) {
        0 => DemonSpell::Explosion,
        _ => DemonSpell::EarthPrison,
    }
}

fn spawn_demon_spell(
    mut commands: Commands,
    q_demon_boss: Query<&DemonBoss>,
    q_demon_spells: Query<&DemonSpellCast>,
    q_demon_spell_cooldowns: Query<&DemonSpellCooldown>,
) {
    let demon_boss = match q_demon_boss.single() {
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

    let spell = pick_random_spell();

    commands.spawn(DemonSpellCooldown {
        spell,
        timer: Timer::from_seconds(5.0, TimerMode::Once),
    });
    commands.spawn(DemonSpellCast {
        spell,
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
            ev_spawn_demon_spell.write(SpawnDemonSpell(demon_spell.clone()));
            commands.entity(entity).despawn();
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
            commands.entity(entity).despawn();
        }
    }
}

fn spawn_last_spell_timer(mut commands: Commands) {
    commands.spawn(LastSpellTimer(Timer::from_seconds(10.0, TimerMode::Once)));
}

fn reset_last_spell_timer(
    time: Res<Time>,
    mut q_last_spell_timer: Query<&mut LastSpellTimer>,
    mut ev_spawn_demon_spell: EventReader<SpawnDemonSpell>,
) {
    let mut timer = match q_last_spell_timer.single_mut() {
        Ok(r) => r,
        Err(_) => return,
    };
    timer.tick(time.delta());

    if ev_spawn_demon_spell.is_empty() {
        return;
    }
    ev_spawn_demon_spell.clear();

    timer.reset();
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
                reset_last_spell_timer,
            )
                .run_if(in_state(GameState::Gaming)),
        )
        .add_event::<SpawnDemonSpell>()
        .add_systems(OnEnter(GameState::Gaming), spawn_last_spell_timer);
    }
}
