use bevy::prelude::*;

use super::{state::DemonBossState, DemonBoss};
use crate::{audio::PlaySound, GameAssets, GameState};

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

fn spawn_demon_spell(
    mut commands: Commands,
    q_demon_boss: Query<&DemonBoss>,
    q_demon_spells: Query<&DemonSpellCast>,
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

    commands.spawn(DemonSpellCast {
        spell: DemonSpell::Explosion,
        timer: Timer::from_seconds(1.5, TimerMode::Once),
    });
}

fn play_cast_vocals(
    assets: Res<GameAssets>,
    q_demon_spells: Query<&DemonSpellCast, Added<DemonSpellCast>>,
    mut play_sound: EventWriter<PlaySound>,
) {
    for demon_spell in &q_demon_spells {
        let vocals = match demon_spell.spell {
            DemonSpell::Explosion => assets.demon_boss_vocal_explosion_sound.clone(),
        };

        play_sound.send(PlaySound {
            clip: vocals,
            ..default()
        });
    }
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

pub struct DemonBossCastPlugin;

impl Plugin for DemonBossCastPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (spawn_demon_spell, relay_demon_spells, play_cast_vocals)
                .run_if(in_state(GameState::Gaming)),
        )
        .add_event::<SpawnDemonSpell>();
    }
}
