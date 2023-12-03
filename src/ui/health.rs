use bevy::prelude::*;

use crate::world::game_entity::SpawnGameEntity;
use crate::{GameAssets, GameState};

use super::damage_number::SpawnDamageText;

#[derive(Component, Clone)]
pub struct Health {
    pub entity: Entity,
    pub health: f32,
    old_health: f32,
    pub max_health: f32,
    pub size: f32,
}

impl Health {
    pub fn new(entity: Entity, max_health: f32, size: f32) -> Self {
        Self {
            entity,
            health: max_health,
            old_health: max_health,
            max_health,
            size,
        }
    }

    pub fn health_bar_offset(&self) -> Vec3 {
        Vec3::new(0.0, -45.0, 0.0) * self.size
    }

    pub fn health_bar_scale(&self) -> Vec3 {
        Vec3::new(1.0, 1.0, 1.0) * self.size
    }
}

#[derive(Component)]
struct HealthBar {
    entity: Entity,
}

#[derive(Component)]
struct HealthBarFill;

fn move_health_bars(
    mut health_bars: Query<(&HealthBar, &mut Transform), (Without<Health>, Without<HealthBarFill>)>,
    healths: Query<(&Transform, &Health), Without<HealthBar>>,
) {
    for (health_transform, health) in &healths {
        for (health_bar, mut health_bar_transform) in &mut health_bars {
            if health.entity != health_bar.entity {
                continue;
            }

            health_bar_transform.translation =
                health_transform.translation + health.health_bar_offset();
        }
    }
}

fn fill_health_bar(
    health_bar_fills: &mut Query<
        (&mut Transform, &HealthBarFill),
        (Without<Health>, Without<HealthBar>),
    >,
    children: &Children,
    health: &Health,
) {
    for &child in children {
        let health_bar_fill = health_bar_fills.get_mut(child);
        if let Ok(mut fill) = health_bar_fill {
            let x_fill = (health.health / health.max_health).clamp(0.0, 1.0);
            fill.0.scale = Vec3::new(x_fill, fill.0.scale.y, fill.0.scale.z);
        }
    }
}

fn fill_health_bars(
    mut health_bars: Query<
        (&HealthBar, &Children, &mut Visibility),
        (Without<Health>, Without<HealthBarFill>),
    >,
    mut health_bar_fills: Query<
        (&mut Transform, &HealthBarFill),
        (Without<Health>, Without<HealthBar>),
    >,
    healths: Query<&Health, Without<HealthBar>>,
) {
    for (health_bar, children, mut health_bar_visibility) in &mut health_bars {
        *health_bar_visibility = Visibility::Hidden;
        for health in &healths {
            if health.entity != health_bar.entity {
                continue;
            }

            *health_bar_visibility = Visibility::Visible;
            fill_health_bar(&mut health_bar_fills, children, health);
        }
    }
}

fn spawn_container(
    commands: &mut Commands,
    spawn_position: Vec3,
    entity: Entity,
    health: &Health,
) -> Entity {
    commands
        .spawn((
            HealthBar { entity },
            SpatialBundle {
                transform: Transform::from_translation(spawn_position + health.health_bar_offset()),
                ..default()
            },
        ))
        .id()
}

fn spawn_background(commands: &mut Commands, assets: &Res<GameAssets>, health: &Health) -> Entity {
    let transform = Transform::from_scale(health.health_bar_scale()).with_translation(Vec3::new(
        health.health_bar_scale().x / 2.0,
        0.0,
        10.0,
    ));
    commands
        .spawn((SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(0.2, 0.2, 0.2),
                ..default()
            },
            texture: assets.health_background.clone(),
            transform,
            ..default()
        },))
        .id()
}

fn spawn_fill_container(commands: &mut Commands) -> Entity {
    commands
        .spawn((HealthBarFill, SpatialBundle::default()))
        .id()
}

fn spawn_fill(commands: &mut Commands, assets: &Res<GameAssets>, health: &Health) -> Entity {
    let transform = Transform::from_scale(health.health_bar_scale()).with_translation(Vec3::new(
        health.health_bar_scale().x / 2.0,
        0.0,
        20.0,
    ));
    commands
        .spawn((SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(0.8, 0.0, 0.0),
                ..default()
            },
            texture: assets.health_fill.clone(),
            transform,
            ..default()
        },))
        .id()
}

fn spawn_health_bar(commands: &mut Commands, assets: &Res<GameAssets>, ev: SpawnGameEntity) {
    let container = spawn_container(commands, Vec3::ZERO, ev.entity, &ev.health);
    let background = spawn_background(commands, assets, &ev.health);
    let fill_container = spawn_fill_container(commands);
    let fill = spawn_fill(commands, assets, &ev.health);

    commands.entity(fill_container).push_children(&[fill]);
    commands
        .entity(container)
        .push_children(&[fill_container, background]);
}

fn spawn_health_bars(
    mut commands: Commands,
    assets: Res<GameAssets>,
    mut ev_spawn_health: EventReader<SpawnGameEntity>,
) {
    for ev in ev_spawn_health.read() {
        if let Some(mut entity) = commands.get_entity(ev.entity) {
            entity.insert(ev.health.clone());
            spawn_health_bar(&mut commands, &assets, ev.clone());
        }
    }
}

fn despawn_health_bars(
    mut commands: Commands,
    q_health_bars: Query<(Entity, &HealthBar), Without<Health>>,
    healths: Query<&Health>,
) {
    for (entity, health_bar) in &q_health_bars {
        for health in &healths {
            if health.entity != health_bar.entity {
                continue;
            }

            if health.health <= 0.0 {
                commands.entity(entity).despawn_recursive();
            }
        }
    }
}

fn check_health_reduction(
    mut q_healths: Query<(&Transform, &mut Health)>,
    mut ev_spawn_damage_text: EventWriter<SpawnDamageText>,
) {
    for (transform, mut health) in &mut q_healths {
        if health.health != health.old_health {
            let damage = (health.health - health.old_health).abs() as u32;
            ev_spawn_damage_text.send(SpawnDamageText {
                pos: transform.translation,
                damage,
            });
            health.old_health = health.health;
        }
    }
}

pub struct HealthPlugin;

impl Plugin for HealthPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                spawn_health_bars,
                despawn_health_bars,
                check_health_reduction,
            )
                .run_if(in_state(GameState::Gaming)),
        )
        .add_systems(
            PostUpdate,
            (move_health_bars, fill_health_bars).run_if(in_state(GameState::Gaming)),
        );
    }
}
