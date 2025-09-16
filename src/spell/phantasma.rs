use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::GameState;
use crate::{player::Player, utils::COLLISION_GROUPS_NONE};

use super::{Spell, SpellCasted};

const DEFAULT_COLOR: Color = Color::srgba(1.0, 1.0, 1.0, 1.0);
const PHANTASMA_COLOR: Color = Color::srgba(1.0, 1.0, 1.0, 0.5);
const DEFAULT_COLLISION_GROUPS: CollisionGroups = CollisionGroups::new(Group::ALL, Group::ALL);

#[derive(Component)]
struct PhantasmaTimer {
    timer: Timer,
}

impl Default for PhantasmaTimer {
    fn default() -> Self {
        Self {
            timer: Timer::from_seconds(30.0, TimerMode::Once),
        }
    }
}

fn activate_phantasma(
    mut commands: Commands,
    mut q_player: Query<(&Children, &mut Sprite), With<Player>>,
    mut q_colliders: Query<&mut CollisionGroups>,
    mut ev_spell_casted: EventReader<SpellCasted>,
) {
    let (children, mut sprite) = match q_player.single_mut() {
        Ok(p) => p,
        Err(_) => return,
    };

    for ev in ev_spell_casted.read() {
        if ev.spell == Spell::Phantasma {
            sprite.color = PHANTASMA_COLOR;

            for child in children {
                if let Ok(mut c) = q_colliders.get_mut(*child) {
                    *c = COLLISION_GROUPS_NONE;
                    break;
                };
            }

            commands.spawn(PhantasmaTimer::default());
        };
    }
}

fn deactivate_phantasma(
    mut commands: Commands,
    time: Res<Time>,
    mut q_player: Query<(&Children, &mut Sprite), With<Player>>,
    mut q_colliders: Query<&mut CollisionGroups>,
    mut q_timers: Query<(Entity, &mut PhantasmaTimer)>,
) {
    let (children, mut sprite) = match q_player.single_mut() {
        Ok(p) => p,
        Err(_) => return,
    };

    for (entity, mut timer) in &mut q_timers {
        timer.timer.tick(time.delta());

        if timer.timer.just_finished() {
            sprite.color = DEFAULT_COLOR;

            for child in children {
                if let Ok(mut c) = q_colliders.get_mut(*child) {
                    *c = DEFAULT_COLLISION_GROUPS;
                    break;
                };
            }

            commands.entity(entity).despawn();
        }
    }
}

pub struct PhantasmaPlugin;

impl Plugin for PhantasmaPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (activate_phantasma, deactivate_phantasma).run_if(in_state(GameState::Gaming)),
        );
    }
}
