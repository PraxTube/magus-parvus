use bevy::prelude::*;
use bevy_asset_loader::prelude::*;
use bevy_ecs_ldtk::prelude::LdtkProject;
use bevy_kira_audio::AudioSource;

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

    #[asset(texture_atlas(tile_size_x = 48.0, tile_size_y = 48.0, columns = 7, rows = 1))]
    #[asset(path = "aer_tracto.png")]
    pub aer_tracto: Handle<TextureAtlas>,

    #[asset(texture_atlas(tile_size_x = 64.0, tile_size_y = 64.0, columns = 30, rows = 1))]
    #[asset(path = "icicle.png")]
    pub icicle: Handle<TextureAtlas>,
    #[asset(texture_atlas(tile_size_x = 96.0, tile_size_y = 96.0, columns = 49, rows = 1))]
    #[asset(path = "icicle_shatter.png")]
    pub icicle_shatter: Handle<TextureAtlas>,

    #[asset(texture_atlas(tile_size_x = 80.0, tile_size_y = 96.0, columns = 18, rows = 1))]
    #[asset(path = "death.png")]
    pub death: Handle<TextureAtlas>,

    #[asset(path = "map/level.ldtk")]
    pub level: Handle<LdtkProject>,

    #[asset(path = "statue.png")]
    pub statue: Handle<Image>,
    #[asset(texture_atlas(tile_size_x = 32.0, tile_size_y = 32.0, columns = 5, rows = 1))]
    #[asset(path = "statue_blink.png")]
    pub statue_blink: Handle<TextureAtlas>,
    #[asset(texture_atlas(tile_size_x = 32.0, tile_size_y = 64.0, columns = 4, rows = 1))]
    #[asset(path = "statue_beam.png")]
    pub statue_beam: Handle<TextureAtlas>,

    #[asset(path = "white_pixel.png")]
    pub white_pixel: Handle<Image>,
    #[asset(path = "heart_full.png")]
    pub heart_full: Handle<Image>,
    #[asset(path = "heart_empty.png")]
    pub heart_empty: Handle<Image>,
    #[asset(texture_atlas(tile_size_x = 34.0, tile_size_y = 34.0, columns = 3, rows = 5))]
    #[asset(path = "keyboard_ui.png")]
    pub keyboard_ui: Handle<TextureAtlas>,

    #[asset(path = "fonts/PressStart2P.ttf")]
    pub font: Handle<Font>,

    #[asset(path = "sounds/player_footstep.ogg")]
    pub player_footstep: Handle<AudioSource>,
    #[asset(path = "music/bgm.ogg")]
    pub bgm: Handle<AudioSource>,

    #[asset(path = "sounds/slime_jump.ogg")]
    pub slime_jump_sound: Handle<AudioSource>,
    #[asset(path = "sounds/slime_land.ogg")]
    pub slime_land_sound: Handle<AudioSource>,
    #[asset(path = "sounds/slime_hit.ogg")]
    pub slime_hit_sound: Handle<AudioSource>,
    #[asset(path = "sounds/slime_death.ogg")]
    pub slime_death_sound: Handle<AudioSource>,
}
