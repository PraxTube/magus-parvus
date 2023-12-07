use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::utils::anim_sprite::{AnimSprite, AnimSpriteTimer};
use crate::world::camera::{YSort, TRANSLATION_TO_PIXEL};
use crate::world::BACKGROUND_ZINDEX_ABS;
use crate::{GameAssets, GameState};

use super::Item;

const OFFSET: Vec3 = Vec3::new(1.0, 45.0, 0.0);

#[derive(Resource, Deref, DerefMut, Default)]
pub struct ActiveStatues(Vec<Statue>);

#[derive(Component)]
pub struct Statue {
    item: Item,
}

impl Statue {
    pub fn new(item: Item) -> Self {
        Self { item }
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
    q_items: Query<(Entity, &Item, &GridCoords), Without<Statue>>,
) {
    for (entity, item, grid_coords) in &q_items {
        let pos = Vec3::new(
            grid_coords.x as f32 * 32.0,
            grid_coords.y as f32 * 32.0,
            0.0,
        );

        let collider = commands
            .spawn((
                Collider::cuboid(20.0, 10.0),
                TransformBundle::from_transform(Transform::from_translation(Vec3::new(
                    0.0, -20.0, 0.0,
                ))),
            ))
            .id();

        commands
            .entity(entity)
            .insert(YSort(0.0 + BACKGROUND_ZINDEX_ABS))
            .insert(Statue::new(item.clone()))
            .insert(SpriteBundle {
                texture: assets.statue.clone(),
                transform: Transform::from_translation(pos),
                ..default()
            })
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
