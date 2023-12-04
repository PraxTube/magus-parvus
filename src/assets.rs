use bevy::prelude::*;
use bevy_asset_loader::prelude::*;
use bevy_ecs_ldtk::prelude::LdtkProject;

#[derive(AssetCollection, Resource)]
pub struct GameAssets {
    #[asset(texture_atlas(tile_size_x = 32.0, tile_size_y = 32.0, columns = 6, rows = 4))]
    #[asset(path = "mage.png")]
    pub player: Handle<TextureAtlas>,

    #[asset(texture_atlas(tile_size_x = 48.0, tile_size_y = 48.0, columns = 6, rows = 4))]
    #[asset(path = "enemy.png")]
    pub enemy: Handle<TextureAtlas>,

    #[asset(texture_atlas(tile_size_x = 70.0, tile_size_y = 11.0, columns = 10, rows = 6))]
    #[asset(path = "fireball.png")]
    pub fireball: Handle<TextureAtlas>,

    #[asset(texture_atlas(tile_size_x = 32.0, tile_size_y = 112.0, columns = 12, rows = 1))]
    #[asset(path = "lightning.png")]
    pub lightning: Handle<TextureAtlas>,

    #[asset(texture_atlas(tile_size_x = 64.0, tile_size_y = 64.0, columns = 30, rows = 1))]
    #[asset(path = "icicle.png")]
    pub icicle: Handle<TextureAtlas>,
    #[asset(texture_atlas(tile_size_x = 96.0, tile_size_y = 96.0, columns = 49, rows = 1))]
    #[asset(path = "icicle_shatter.png")]
    pub icicle_shatter: Handle<TextureAtlas>,

    #[asset(path = "map/level.ldtk")]
    pub level: Handle<LdtkProject>,

    #[asset(path = "health_fill.png")]
    pub health_fill: Handle<Image>,
    #[asset(path = "health_background.png")]
    pub health_background: Handle<Image>,

    #[asset(path = "fonts/PressStart2P.ttf")]
    pub font: Handle<Font>,
}
