use bevy::prelude::*;
use bevy_asset_loader::prelude::*;

#[derive(AssetCollection, Resource)]
pub struct GameAssets {
    #[asset(texture_atlas(tile_size_x = 68.0, tile_size_y = 9.0, columns = 10, rows = 6))]
    #[asset(path = "fireball.png")]
    pub fireball: Handle<TextureAtlas>,
}
