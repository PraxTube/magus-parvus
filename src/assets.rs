use bevy::prelude::*;
use bevy_asset_loader::prelude::*;
use bevy_ecs_ldtk::prelude::LdtkProject;

#[derive(AssetCollection, Resource)]
pub struct GameAssets {
    #[asset(texture_atlas(tile_size_x = 70.0, tile_size_y = 11.0, columns = 10, rows = 6))]
    #[asset(path = "fireball.png")]
    pub fireball: Handle<TextureAtlas>,

    #[asset(path = "level.ldtk")]
    pub level: Handle<LdtkProject>,
}
