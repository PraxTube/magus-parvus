use bevy::prelude::*;
use bevy_ecs_ldtk::GridCoords;
use bevy_rapier2d::prelude::*;

use crate::utils::anim_sprite::{AnimSprite, AnimSpriteTimer};
use crate::world::camera::{YSort, TRANSLATION_TO_PIXEL};
use crate::{GameAssets, GameState};

use super::Item;

const OFFSET: Vec3 = Vec3::new(1.0, 45.0, 0.0);

#[derive(Resource, Deref, DerefMut, Default)]
pub struct ActiveStatues(Vec<Statue>);

#[derive(PartialEq)]
pub struct Statue {
    item: Item,
    pos: Vec2,
}

impl Statue {
    pub fn new(item: Item, grid_coords: &GridCoords) -> Self {
        Self {
            item,
            pos: Vec2::new(grid_coords.x as f32 * 32.0, grid_coords.y as f32 * 32.0),
        }
    }
}

#[derive(Event)]
pub struct SpawnStatue {
    pub pos: Vec3,
}

#[derive(Event)]
pub struct StatueUnlocked {
    pub pos: Vec3,
}

fn spawn_statue_beams(
    mut commands: Commands,
    assets: Res<GameAssets>,
    mut ev_statue_unlocked: EventReader<StatueUnlocked>,
) {
    for ev in ev_statue_unlocked.read() {
        commands.spawn((
            AnimSprite::new(4, true),
            AnimSpriteTimer::new(0.05),
            YSort((OFFSET.y - 1.0) * TRANSLATION_TO_PIXEL),
            SpriteSheetBundle {
                texture_atlas: assets.statue_beam.clone(),
                transform: Transform::from_translation(ev.pos + OFFSET),
                ..default()
            },
        ));
    }
}

fn spawn_statues(
    mut commands: Commands,
    assets: Res<GameAssets>,
    mut ev_spawn_statue: EventReader<SpawnStatue>,
) {
    for ev in ev_spawn_statue.read() {
        let collider = commands
            .spawn((
                Collider::cuboid(20.0, 10.0),
                TransformBundle::from_transform(Transform::from_translation(Vec3::new(
                    0.0, -20.0, 0.0,
                ))),
            ))
            .id();

        commands
            .spawn((
                YSort(0.0),
                SpriteBundle {
                    texture: assets.statue.clone(),
                    transform: Transform::from_translation(ev.pos),
                    ..default()
                },
            ))
            .push_children(&[collider]);
    }
}

pub struct StatuePlugin;

impl Plugin for StatuePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (spawn_statues, spawn_statue_beams).run_if(in_state(GameState::Gaming)),
        )
        .init_resource::<ActiveStatues>()
        .add_event::<SpawnStatue>()
        .add_event::<StatueUnlocked>();
    }
}
