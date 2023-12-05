use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;

use crate::world::camera::YSort;

pub struct ItemPlugin;

impl Plugin for ItemPlugin {
    fn build(&self, app: &mut App) {
        app.register_ldtk_entity::<ItemBundle>("Item")
            .add_systems(Update, (debug_items, add_item_ysort));
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
    #[sprite_sheet_bundle]
    sprite_sheet_bundle: SpriteSheetBundle,
}

fn add_item_ysort(mut commands: Commands, q_items: Query<Entity, (With<Item>, Without<YSort>)>) {
    for entity in &q_items {
        commands.entity(entity).insert(YSort(0.0));
    }
}

fn debug_items(mut q_items: Query<(&Item, &mut Transform)>) {
    for (item, mut transform) in &mut q_items {
        transform.translation += Vec3::new(0.1, 0.0, 0.0);
        info!("{:?}, {:?}", transform, item);
    }
}
