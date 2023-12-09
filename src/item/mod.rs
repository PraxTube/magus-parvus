mod enemy_spawner;
pub mod item_value;
pub mod statue;

use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;

pub struct ItemPlugin;

impl Plugin for ItemPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((statue::StatuePlugin, enemy_spawner::EnemySpawnerPlugin))
            .register_ldtk_entity::<ItemBundle>("Item");
    }
}

#[derive(Debug, Default, Component, Reflect, Clone, PartialEq)]
pub enum Item {
    #[default]
    NotImplemented,
    Tutorial,
    Fulgur,
}

impl Item {
    fn from_str(s: &str) -> Item {
        match s {
            "Tutorial" => Item::Tutorial,
            "Fulgur" => Item::Fulgur,
            _ => Item::NotImplemented,
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
    worldly: Worldly,
}
