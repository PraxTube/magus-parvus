use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;

pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(LdtkPlugin)
            .add_systems(Startup, spawn_map)
            .insert_resource(LevelSelection::index(0));
    }
}

fn spawn_map(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(LdtkWorldBundle {
        ldtk_handle: asset_server.load("level.ldtk"),
        transform: Transform::from_translation(Vec3::new(0.0, 0.0, -1000.0)),
        ..Default::default()
    });
}
