use std::f32::consts::TAU;

use bevy::prelude::*;

use super::statue::{Statue, StatueTriggered};
use crate::{enemy::slime::SpawnSlimeEnemy, GameState};

const OFFSET: f32 = 300.0;
const ENEMY_COUNT: usize = 100;

#[derive(Component)]
struct EnemySpawner {
    statue: Statue,
    pos: Vec3,
    timer: Timer,
    disabled: bool,
}

impl EnemySpawner {
    fn new(statue: Statue, pos: Vec3) -> Self {
        let timer = Timer::from_seconds(10.0, TimerMode::Once);
        Self {
            statue,
            pos,
            timer,
            disabled: false,
        }
    }
}

fn spawn_enemy_spawners(
    mut commands: Commands,
    mut ev_initiate_enemy_spawning: EventReader<StatueTriggered>,
) {
    for ev in ev_initiate_enemy_spawning.read() {
        commands.spawn(EnemySpawner::new(ev.statue.clone(), ev.statue.pos));
    }
}

fn tick_enemy_spawners(time: Res<Time>, mut q_enemy_spawners: Query<&mut EnemySpawner>) {
    for mut enemy_spawner in &mut q_enemy_spawners {
        enemy_spawner.timer.tick(time.delta());

        if enemy_spawner.timer.just_finished() {
            enemy_spawner.disabled = true;
        }
    }
}

fn despawn_enemy_spawners(
    mut commands: Commands,
    q_enemy_spawners: Query<(Entity, &EnemySpawner)>,
    mut q_statues: Query<&mut Statue>,
) {
    for (entity, enemy_spawner) in &q_enemy_spawners {
        if enemy_spawner.disabled {
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

            commands.entity(entity).despawn_recursive();
        }
    }
}

fn spawn_enemies(
    q_enemy_spawners: Query<&EnemySpawner>,
    mut ev_spawn_slime_enemy: EventWriter<SpawnSlimeEnemy>,
    mut e: Local<usize>,
) {
    for enemy_spawner in &q_enemy_spawners {
        let pos = enemy_spawner.pos
            + Quat::from_rotation_z(*e as f32 / ENEMY_COUNT as f32 * TAU).mul_vec3(Vec3::X)
                * OFFSET;
        ev_spawn_slime_enemy.send(SpawnSlimeEnemy { pos });
        *e += 1;
    }
}

pub struct EnemySpawnerPlugin;

impl Plugin for EnemySpawnerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                spawn_enemy_spawners,
                tick_enemy_spawners,
                despawn_enemy_spawners,
                spawn_enemies,
            )
                .run_if(in_state(GameState::Gaming)),
        );
    }
}
