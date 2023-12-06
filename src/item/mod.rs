use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;

use crate::world::camera::YSort;
use crate::world::BACKGROUND_ZINDEX_ABS;
use crate::{GameAssets, GameState};

pub struct ItemPlugin;

impl Plugin for ItemPlugin {
    fn build(&self, app: &mut App) {
        app.register_ldtk_entity::<ItemBundle>("Item").add_systems(
            Update,
            (add_item_ysort, add_item_sprite_bundle).run_if(in_state(GameState::Gaming)),
        );
    }
}

#[derive(Debug, Default, Component, Reflect)]
enum Item {
    #[default]
    Test,
    Fulgur,
}

impl Item {
    fn from_str(s: &str) -> Item {
        match s {
            "Fulgur" => Item::Fulgur,
            _ => Item::Test,
        }
    }

    fn from_field(entity_instance: &EntityInstance) -> Item {
        match entity_instance.get_enum_field("item") {
            Ok(s) => Item::from_str(s),
            Err(_) => Item::default(),
        }
    }
}

#[derive(Default, Bundle, LdtkEntity)]
struct ItemBundle {
    #[with(Item::from_field)]
    item: Item,
    #[grid_coords]
    grid_coords: GridCoords,
    #[worldly]
    worldy: Worldly,
}

fn add_item_ysort(mut commands: Commands, q_items: Query<Entity, (With<Item>, Without<YSort>)>) {
    for entity in &q_items {
        commands
            .entity(entity)
            .insert(YSort(0.0 + BACKGROUND_ZINDEX_ABS));
    }
}

fn add_item_sprite_bundle(
    mut commands: Commands,
    assets: Res<GameAssets>,
    q_items: Query<(Entity, &GridCoords), (With<Item>, Without<Sprite>)>,
) {
    for (entity, grid_coords) in &q_items {
        commands.entity(entity).insert(SpriteBundle {
            texture: assets.statue.clone(),
            transform: Transform::from_translation(Vec3::new(
                grid_coords.x as f32 * 32.0,
                grid_coords.y as f32 * 32.0,
                0.0,
            )),
            ..default()
        });
    }
}
