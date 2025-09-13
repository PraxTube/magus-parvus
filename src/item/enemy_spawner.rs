use std::time::Duration;

use bevy::prelude::*;

use super::enemy_sub_spawner::EnemySubSpawner;
use super::item_value::statue_sub_spawner;
use super::statue::{Statue, StatueTriggered};
use crate::GameState;

const PADDED_TIME: f32 = 0.5;

#[derive(Component)]
pub struct EnemySpawner {
    statue: Statue,
    sub_spawners: Vec<(f32, EnemySubSpawner)>,
    sub_spawner_index: usize,
    timer: Timer,
    disabled: bool,
}

impl EnemySpawner {
    fn new(sub_spawners: Vec<(f32, EnemySubSpawner)>, statue: Statue) -> Self {
        let timer = Timer::from_seconds(1.0, TimerMode::Once);
        Self {
            statue,
            sub_spawners,
            sub_spawner_index: 0,
            timer,
            disabled: false,
        }
    }
}

fn spawn_spawners(mut commands: Commands, mut ev_statue_triggered: EventReader<StatueTriggered>) {
    for ev in ev_statue_triggered.read() {
        commands.spawn(EnemySpawner::new(
            statue_sub_spawner(&ev.statue),
            ev.statue.clone(),
        ));
    }
}

fn spawn_sub_spawners(mut commands: Commands, mut q_enemy_spawners: Query<&mut EnemySpawner>) {
    for mut enemy_spawner in &mut q_enemy_spawners {
        if !enemy_spawner.timer.just_finished() {
            continue;
        }
        if enemy_spawner.sub_spawner_index >= enemy_spawner.sub_spawners.len() {
            continue;
        }

        let (time, sub_spawner) =
            enemy_spawner.sub_spawners[enemy_spawner.sub_spawner_index].clone();

        enemy_spawner.sub_spawner_index += 1;
        enemy_spawner
            .timer
            .set_duration(Duration::from_secs_f32(time + PADDED_TIME));
        enemy_spawner.timer.reset();
        commands.spawn(sub_spawner);
    }
}

fn tick_timers(time: Res<Time>, mut q_enemy_spawners: Query<&mut EnemySpawner>) {
    for mut enemy_spawner in &mut q_enemy_spawners {
        enemy_spawner.timer.tick(time.delta());
    }
}

fn disable_enemy_spawners(
    mut q_enemy_spawners: Query<&mut EnemySpawner>,
    q_enemy_sub_spawners: Query<&EnemySubSpawner>,
) {
    if !q_enemy_sub_spawners.is_empty() {
        return;
    }

    for mut enemy_spawner in &mut q_enemy_spawners {
        // There are still subspawners that need to be spawned.
        if enemy_spawner.sub_spawner_index < enemy_spawner.sub_spawners.len() {
            continue;
        }

        if enemy_spawner.timer.finished() {
            enemy_spawner.disabled = true;
        }
    }
}

fn despawn_spawners(
    mut commands: Commands,
    q_enemy_spawners: Query<(Entity, &EnemySpawner)>,
    mut q_statues: Query<&mut Statue>,
) {
    for (entity, enemy_spawner) in &q_enemy_spawners {
        if !enemy_spawner.disabled {
            continue;
        }

        // Get the statue corresponding to the enmey spawner and set `all_enemies_spawned` flag
        // to true, indicating that it should check if all enemies are killed and unlock the
        // statue if that is the case.
        for mut statue in &mut q_statues {
            if enemy_spawner.statue.item == statue.item {
                if statue.all_enemies_spawned {
                    error!("this statue is already unlockable,
                    most likely due to the fact that there are multiple statues with the same enum (item) type");
                }
                statue.all_enemies_spawned = true;
                break;
            }
        }

        commands.entity(entity).despawn();
    }
}

pub struct EnemySpawnerPlugin;

impl Plugin for EnemySpawnerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                spawn_spawners,
                spawn_sub_spawners,
                tick_timers,
                disable_enemy_spawners,
                despawn_spawners,
            )
                .run_if(in_state(GameState::Gaming)),
        );
    }
}
