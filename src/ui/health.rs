use bevy::prelude::*;

use crate::{player::Player, GameAssets, GameState};

use super::world_text::{SpawnWorldText, WorldText};

#[derive(Component, Clone)]
pub struct Health {
    pub health: f32,
    old_health: f32,
}

#[derive(Component)]
struct Heart {
    index: usize,
}

#[derive(Component)]
struct HeartsContainer;

#[derive(Event)]
pub struct SpawnPlayerHearts {
    pub count: usize,
}

#[derive(Event)]
struct HealthChanged {
    entity: Entity,
    health_change: f32,
}

impl Health {
    pub fn new(max_health: f32) -> Self {
        Self {
            health: max_health,
            old_health: max_health,
        }
    }
}

fn spawn_health_damage_text(
    q_transforms: Query<&Transform, With<Health>>,
    mut ev_health_changed: EventReader<HealthChanged>,
    mut ev_spawn_damage_text: EventWriter<SpawnWorldText>,
) {
    for ev in ev_health_changed.read() {
        let pos = match q_transforms.get(ev.entity) {
            Ok(t) => t.translation,
            Err(_) => continue,
        };

        ev_spawn_damage_text.send(SpawnWorldText {
            world_text: WorldText::default(),
            pos,
            content: ev.health_change.to_string(),
        });
    }
}

fn check_health_changed(
    mut q_healths: Query<(Entity, &mut Health)>,
    mut ev_health_changed: EventWriter<HealthChanged>,
) {
    for (entity, mut health) in &mut q_healths {
        if health.health != health.old_health {
            let health_change = (health.health - health.old_health).abs();
            ev_health_changed.send(HealthChanged {
                entity,
                health_change,
            });
            health.old_health = health.health;
        }
    }
}

fn spawn_heart(commands: &mut Commands, assets: &Res<GameAssets>, index: usize) -> Entity {
    commands
        .spawn((
            Heart { index },
            ImageBundle {
                image: UiImage {
                    texture: assets.heart_full.clone(),
                    ..default()
                },
                style: Style {
                    width: Val::Percent(4.0),
                    ..default()
                },

                ..default()
            },
        ))
        .id()
}

fn spawn_hearts(
    mut commands: Commands,
    assets: Res<GameAssets>,
    mut ev_spawn_player_hearts: EventReader<SpawnPlayerHearts>,
) {
    for ev in ev_spawn_player_hearts.read() {
        let root = commands
            .spawn((
                HeartsContainer,
                NodeBundle {
                    style: Style {
                        width: Val::Percent(90.0),
                        top: Val::Percent(5.0),
                        left: Val::Percent(5.0),
                        ..default()
                    },
                    ..default()
                },
            ))
            .id();

        for i in 0..ev.count {
            let heart_entity = spawn_heart(&mut commands, &assets, i);
            commands.entity(root).push_children(&[heart_entity]);
        }
    }
}

fn despawn_hearts(
    mut commands: Commands,
    q_hearts_containers: Query<Entity, With<HeartsContainer>>,
) {
    for entity in &q_hearts_containers {
        commands.entity(entity).despawn_recursive();
    }
}

fn update_player_hearts(
    assets: Res<GameAssets>,
    q_player: Query<&Health, With<Player>>,
    mut q_hearts: Query<(&mut UiImage, &Heart)>,
    mut ev_health_changed: EventReader<HealthChanged>,
) {
    for ev in ev_health_changed.read() {
        let player_health = match q_player.get(ev.entity) {
            Ok(p) => p,
            Err(_) => continue,
        };

        let threshold_index = player_health.health as usize;

        for (mut ui_image, heart) in &mut q_hearts {
            if heart.index >= threshold_index {
                ui_image.texture = assets.heart_empty.clone();
            }
        }
    }
}

pub struct HealthPlugin;

impl Plugin for HealthPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                spawn_health_damage_text,
                check_health_changed,
                spawn_hearts,
                update_player_hearts,
            )
                .run_if(in_state(GameState::Gaming)),
        )
        .add_event::<SpawnPlayerHearts>()
        .add_event::<HealthChanged>()
        .add_systems(OnEnter(GameState::GameOver), despawn_hearts);
    }
}
