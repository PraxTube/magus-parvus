use std::{f32::consts::TAU, time::Duration};

use rand::{rngs::ThreadRng, thread_rng, Rng};

use bevy::prelude::*;

use super::statue::{Statue, StatueTriggered};
use super::Item;
use crate::{enemy::slime::SpawnSlimeEnemy, GameState};

const PADDED_TIME: f32 = 0.5;

#[derive(Clone)]
pub enum SpawnFormation {
    Circle,
    Group,
    Random,
}

#[derive(Component)]
struct EnemySpawner {
    statue: Statue,
    sub_spawners: Vec<(f32, EnemySubSpawner)>,
    sub_spawner_index: usize,
    timer: Timer,
    disabled: bool,
}

#[derive(Component, Clone)]
struct EnemySubSpawner {
    statue: Statue,
    count: usize,
    current_index: usize,
    offset: f32,
    angle: f32,
    spawn_formation: SpawnFormation,
    timer: Timer,
    disabled: bool,
}

fn statue_sub_spawners(statue: &Statue) -> Vec<(f32, EnemySubSpawner)> {
    match statue.item {
        Item::NotImplemented => Vec::new(),
        Item::Tutorial => Vec::new(),
        Item::Fulgur => vec![
            (
                5.0,
                EnemySubSpawner {
                    statue: statue.clone(),
                    count: 8,
                    spawn_formation: SpawnFormation::Random,
                    timer: Timer::from_seconds(0.1, TimerMode::Repeating),
                    ..default()
                },
            ),
            (
                0.0,
                EnemySubSpawner {
                    statue: statue.clone(),
                    count: 15,
                    spawn_formation: SpawnFormation::Circle,
                    ..default()
                },
            ),
            (
                5.0,
                EnemySubSpawner {
                    statue: statue.clone(),
                    count: 8,
                    spawn_formation: SpawnFormation::Random,
                    offset: 300.0,
                    timer: Timer::from_seconds(0.2, TimerMode::Repeating),
                    ..default()
                },
            ),
            (
                0.0,
                EnemySubSpawner {
                    statue: statue.clone(),
                    count: 10,
                    spawn_formation: SpawnFormation::Group,
                    offset: 300.0,
                    angle: -TAU / 3.0,
                    ..default()
                },
            ),
        ],
    }
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

impl Default for EnemySubSpawner {
    fn default() -> Self {
        Self {
            statue: Statue::default(),
            count: 0,
            current_index: 0,
            offset: 200.0,
            angle: 0.0,
            spawn_formation: SpawnFormation::Circle,
            timer: Timer::from_seconds(0.0, TimerMode::Once),
            disabled: false,
        }
    }
}

fn spawn_spawners(
    mut commands: Commands,
    mut ev_initiate_enemy_spawning: EventReader<StatueTriggered>,
) {
    for ev in ev_initiate_enemy_spawning.read() {
        commands.spawn(EnemySpawner::new(
            statue_sub_spawners(&ev.statue),
            ev.statue.clone(),
        ));
    }
}

fn tick_enemy_spawner_timers(
    time: Res<Time>,
    mut q_enemy_spawners: Query<&mut EnemySpawner>,
    mut q_enemey_sub_spawners: Query<&mut EnemySubSpawner>,
) {
    for mut enemy_spawner in &mut q_enemy_spawners {
        enemy_spawner.timer.tick(time.delta());
    }

    for mut enemy_sub_spawner in &mut q_enemey_sub_spawners {
        enemy_sub_spawner.timer.tick(time.delta());
    }
}

fn disable_enemy_spawners(
    mut q_enemy_spawners: Query<&mut EnemySpawner>,
    q_enemy_sub_spawners: Query<&EnemySubSpawner>,
) {
    if q_enemy_sub_spawners.iter().count() > 0 {
        return;
    }

    for mut enemy_spawner in &mut q_enemy_spawners {
        // There are still subspawners that need to be spawned.
        if enemy_spawner.sub_spawner_index < enemy_spawner.sub_spawners.len() {
            continue;
        }
        if !enemy_spawner.timer.finished() {
            continue;
        }

        enemy_spawner.disabled = true;
    }
}

fn disable_enemy_sub_spawners(mut q_enemey_sub_spawners: Query<&mut EnemySubSpawner>) {
    for mut enemy_sub_spawner in &mut q_enemey_sub_spawners {
        if enemy_sub_spawner.current_index == enemy_sub_spawner.count {
            enemy_sub_spawner.disabled = true;
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

        commands.entity(entity).despawn_recursive();
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

fn despawn_sub_spawners(
    mut commands: Commands,
    q_enemy_sub_spawners: Query<(Entity, &EnemySubSpawner)>,
) {
    for (entity, enemy_spawner) in &q_enemy_sub_spawners {
        if enemy_spawner.disabled {
            commands.entity(entity).despawn_recursive();
        }
    }
}

fn spawn_enemy(
    sub_spawner: &mut EnemySubSpawner,
    ev_spawn_slime_enemy: &mut EventWriter<SpawnSlimeEnemy>,
    rng: &mut ThreadRng,
) {
    let pos = match sub_spawner.spawn_formation {
        SpawnFormation::Circle => {
            sub_spawner.statue.pos
                + Quat::from_rotation_z(
                    sub_spawner.current_index as f32 / sub_spawner.count as f32 * TAU,
                )
                .mul_vec3(Vec3::X)
                    * sub_spawner.offset
        }
        SpawnFormation::Group => {
            sub_spawner.statue.pos
                + Quat::from_rotation_z(sub_spawner.angle).mul_vec3(Vec3::X) * sub_spawner.offset
        }
        SpawnFormation::Random => {
            sub_spawner.statue.pos
                + Quat::from_rotation_z(rng.gen_range(0.0..TAU)).mul_vec3(Vec3::X)
                    * sub_spawner.offset
        }
    };

    sub_spawner.current_index += 1;
    ev_spawn_slime_enemy.send(SpawnSlimeEnemy { pos });
}

fn spawn_enemies(
    mut q_sub_spawners: Query<&mut EnemySubSpawner>,
    mut ev_spawn_slime_enemy: EventWriter<SpawnSlimeEnemy>,
) {
    let mut rng = thread_rng();

    for mut sub_spawner in &mut q_sub_spawners {
        if sub_spawner.disabled {
            continue;
        }

        // Spawn all enemies at once.
        if sub_spawner.timer.duration().as_secs_f32() == 0.0 {
            for _ in 0..sub_spawner.count {
                spawn_enemy(&mut sub_spawner, &mut ev_spawn_slime_enemy, &mut rng);
            }
            continue;
        }

        // Spawn single enemy based on timer.
        if sub_spawner.timer.just_finished() {
            spawn_enemy(&mut sub_spawner, &mut ev_spawn_slime_enemy, &mut rng);
        }
    }
}

pub struct EnemySpawnerPlugin;

impl Plugin for EnemySpawnerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                spawn_spawners,
                tick_enemy_spawner_timers,
                disable_enemy_spawners,
                disable_enemy_sub_spawners,
                despawn_spawners,
                spawn_sub_spawners,
                despawn_sub_spawners,
                spawn_enemies.before(disable_enemy_sub_spawners),
            )
                .run_if(in_state(GameState::Gaming)),
        );
    }
}
