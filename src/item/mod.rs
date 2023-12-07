mod statue;

use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;

use crate::GameState;
use statue::{ActiveStatues, SpawnStatue, Statue};

pub struct ItemPlugin;

impl Plugin for ItemPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (check_spawn_statues).run_if(in_state(GameState::Gaming)),
        )
        .register_ldtk_entity::<ItemBundle>("Item")
        .add_plugins(statue::StatuePlugin);
    }
}

#[derive(Component)]
pub struct VisitedItem;

#[derive(Debug, Default, Component, Reflect, Clone, PartialEq)]
pub enum Item {
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
    worldly: Worldly,
}

fn check_spawn_statues(
    mut commands: Commands,
    q_items: Query<(Entity, &Parent, &Item, &GridCoords), Without<VisitedItem>>,
    q_transforms: Query<(&Parent, &GlobalTransform), Without<Item>>,
    mut statues: ResMut<ActiveStatues>,
    mut ev_spawn_statue: EventWriter<SpawnStatue>,
) {
    for (entity, parent, item, grid_coords) in &q_items {
        let parent_pos = match q_transforms.get(parent.get()) {
            Ok(p) => match q_transforms.get(p.0.get()) {
                Ok(p) => p.1.translation(),
                Err(err) => {
                    error!("no parent found, {}", err);
                    continue;
                }
            },
            Err(err) => {
                error!("no parent found, {}", err);
                continue;
            }
        };

        let statue = Statue::new(item.clone(), grid_coords);
        statues.push(statue);
        commands.entity(entity).insert(VisitedItem);

        let transform = Vec3::new(
            grid_coords.x as f32 * 32.0,
            grid_coords.y as f32 * 32.0,
            0.0,
        ) + parent_pos;
        ev_spawn_statue.send(SpawnStatue { pos: transform });
    }
}
