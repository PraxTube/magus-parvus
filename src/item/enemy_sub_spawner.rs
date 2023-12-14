use std::f32::consts::TAU;

use rand::{rngs::ThreadRng, thread_rng, Rng};

use bevy::prelude::*;

use super::statue::Statue;
use crate::{enemy::slime::SpawnSlimeEnemy, GameState};

#[derive(Clone)]
pub enum SpawnFormation {
    Circle,
    Group,
    Random,
}

#[derive(Component, Clone)]
pub struct EnemySubSpawner {
    pub statue: Statue,
    pub count: usize,
    pub current_index: usize,
    pub offset: f32,
    pub angle: f32,
    pub spawn_formation: SpawnFormation,
    pub timer: Timer,
    pub disabled: bool,
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

fn tick_timers(time: Res<Time>, mut q_enemey_sub_spawners: Query<&mut EnemySubSpawner>) {
    for mut enemy_sub_spawner in &mut q_enemey_sub_spawners {
        enemy_sub_spawner.timer.tick(time.delta());
    }
}

fn disable_enemy_sub_spawners(mut q_enemey_sub_spawners: Query<&mut EnemySubSpawner>) {
    for mut enemy_sub_spawner in &mut q_enemey_sub_spawners {
        if enemy_sub_spawner.current_index == enemy_sub_spawner.count {
            enemy_sub_spawner.disabled = true;
        }
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

pub struct EnemySubSpawnerPlugin;

impl Plugin for EnemySubSpawnerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                tick_timers,
                disable_enemy_sub_spawners,
                despawn_sub_spawners,
                spawn_enemies.before(disable_enemy_sub_spawners),
            )
                .run_if(in_state(GameState::Gaming)),
        );
    }
}
