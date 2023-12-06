use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;

use crate::world::camera::{YSort, TRANSLATION_TO_PIXEL};
use crate::world::BACKGROUND_ZINDEX_ABS;
use crate::GameState;

// The index of the layer the items reside in.
// If the LDtk layers are changed (rearanged or added/removed),
// then this will need to be changed accordingly.
const ITEM_LAYER_ZINDEX_ABS: f32 = 3.0;

pub struct ItemPlugin;

impl Plugin for ItemPlugin {
    fn build(&self, app: &mut App) {
        app.register_ldtk_entity::<ItemBundle>("Item")
            .add_systems(Update, add_item_ysort.run_if(in_state(GameState::Gaming)));
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
        let offset = -ITEM_LAYER_ZINDEX_ABS + BACKGROUND_ZINDEX_ABS;
        commands
            .entity(entity)
            .insert(YSort(-10.0 * TRANSLATION_TO_PIXEL + offset));
    }
}
