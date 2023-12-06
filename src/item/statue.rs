use bevy::prelude::*;
use bevy_ecs_ldtk::GridCoords;

use crate::{
    utils::anim_sprite::{AnimSprite, AnimSpriteTimer},
    world::camera::{YSort, TRANSLATION_TO_PIXEL},
    GameAssets, GameState,
};

use super::Item;

const OFFSET: Vec3 = Vec3::new(1.0, 45.0, 0.0);

#[derive(Event)]
pub struct StatueUnlocked {
    pub pos: Vec3,
}

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

fn spawn_statue_beam(
    mut commands: Commands,
    assets: Res<GameAssets>,
    mut ev_statue_unlocked: EventReader<StatueUnlocked>,
) {
    for ev in ev_statue_unlocked.read() {
        info!("{:?}", ev.pos);
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

pub struct StatuePlugin;

impl Plugin for StatuePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (spawn_statue_beam).run_if(in_state(GameState::Gaming)),
        )
        .add_event::<StatueUnlocked>();
    }
}
