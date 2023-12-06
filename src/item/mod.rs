mod statue;

use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::world::camera::YSort;
use crate::world::BACKGROUND_ZINDEX_ABS;
use crate::{GameAssets, GameState};

use self::statue::{Statue, StatueUnlocked};

pub struct ItemPlugin;

impl Plugin for ItemPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (add_item_ysort, add_item_sprite_bundle).run_if(in_state(GameState::Gaming)),
        )
        .init_resource::<ActiveStatues>()
        .register_ldtk_entity::<ItemBundle>("Item")
        .add_plugins(statue::StatuePlugin);
    }
}

#[derive(Resource, Deref, DerefMut, Default)]
pub struct ActiveStatues(Vec<Statue>);

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
    q_items: Query<(Entity, &Parent, &Item, &GridCoords), Without<VisitedItem>>,
    q_transforms: Query<(&Parent, &GlobalTransform), Without<Item>>,
    mut statues: ResMut<ActiveStatues>,
    mut ev_statue_unlocked: EventWriter<StatueUnlocked>,
) {
    for (entity, parent, item, grid_coords) in &q_items {
        let statue = Statue::new(item.clone(), grid_coords);
        if statues.contains(&statue) {
            continue;
        };

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

        statues.push(statue);
        commands.entity(entity).insert(VisitedItem);

        let collider = commands
            .spawn((
                Collider::cuboid(20.0, 10.0),
                TransformBundle::from_transform(Transform::from_translation(Vec3::new(
                    0.0, -20.0, 0.0,
                ))),
            ))
            .id();

        info!("{:?}", parent_pos);

        let transform = Transform::from_translation(
            Vec3::new(
                grid_coords.x as f32 * 32.0,
                grid_coords.y as f32 * 32.0,
                0.0,
            ) + parent_pos,
        );
        ev_statue_unlocked.send(StatueUnlocked {
            pos: transform.translation,
        });

        commands
            .spawn((
                YSort(0.0),
                SpriteBundle {
                    texture: assets.statue.clone(),
                    transform,
                    ..default()
                },
            ))
            .push_children(&[collider]);
    }
}
