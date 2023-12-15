pub mod item_value;
pub mod statue;

mod enemy_spawner;
mod enemy_sub_spawner;

use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;

pub struct ItemPlugin;

impl Plugin for ItemPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            statue::StatuePlugin,
            enemy_spawner::EnemySpawnerPlugin,
            enemy_sub_spawner::EnemySubSpawnerPlugin,
        ))
        .init_resource::<ActiveItems>()
        .register_ldtk_entity::<ItemBundle>("Item");
    }
}

#[derive(Debug, Default, Component, Reflect, Clone, PartialEq)]
pub enum Item {
    #[default]
    NotImplemented,
    Tutorial,
    IgnisPila,
    InfernoPila,
    Fulgur,
    ScutumGlaciei,
    AerTracto,
    AerPello,
    FulgurAvis,
}

#[derive(Resource, Default, Deref, DerefMut)]
pub struct ActiveItems(pub Vec<Item>);

#[derive(Default, Bundle, LdtkEntity)]
struct ItemBundle {
    #[with(Item::from_field)]
    item: Item,
    #[grid_coords]
    grid_coords: GridCoords,
    #[worldly]
    worldly: Worldly,
}

impl Item {
    fn from_str(s: &str) -> Item {
        match s {
            "Tutorial" => Item::Tutorial,
            "IgnisPila" => Item::IgnisPila,
            "InfernoPila" => Item::InfernoPila,
            "Fulgur" => Item::Fulgur,
            "ScutumGlaciei" => Item::ScutumGlaciei,
            "AerTracto" => Item::AerTracto,
            "AerPello" => Item::AerPello,
            "FulgurAvis" => Item::FulgurAvis,
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
