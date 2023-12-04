use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;

use bevy_rapier2d::prelude::*;

#[derive(Copy, Clone, Eq, PartialEq, Debug, Default, Component)]
pub struct Pot;

#[derive(Clone, Debug, Default, Bundle, LdtkIntCell)]
pub struct PotBundle {
    pot: Pot,
}

pub fn insert_pot_collider(mut commands: Commands, pot_query: Query<Entity, Added<Pot>>) {
    for entity in &pot_query {
        let collider = commands
            .spawn((
                Collider::ball(10.0),
                TransformBundle::from_transform(Transform::from_translation(Vec3::new(
                    -8.0, 8.0, 0.0,
                ))),
            ))
            .id();
        commands.entity(entity).push_children(&[collider]);
    }
}

pub struct MapColliderPlugin;

impl Plugin for MapColliderPlugin {
    fn build(&self, app: &mut App) {
        app.register_ldtk_int_cell::<PotBundle>(1)
            .add_systems(Update, insert_pot_collider);
    }
}
