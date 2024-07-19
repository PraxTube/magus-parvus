use bevy::prelude::*;
use bevy_asset_loader::prelude::*;
use bevy_ecs_ldtk::prelude::LdtkProject;
use bevy_kira_audio::AudioSource;
use bevy_trickfilm::prelude::*;

#[derive(AssetCollection, Resource)]
pub struct GameAssets {
    #[asset(path = "mage.png")]
    pub player_texture: Handle<Image>,
    #[asset(texture_atlas(tile_size_x = 32, tile_size_y = 32, columns = 6, rows = 4))]
    pub player_layout: Handle<TextureAtlasLayout>,

    #[asset(path = "enemy/enemy.png")]
    pub slime_texture: Handle<Image>,
    #[asset(texture_atlas(tile_size_x = 48, tile_size_y = 48, columns = 6, rows = 4))]
    pub slime_layout: Handle<TextureAtlasLayout>,

    #[asset(path = "enemy/enemy_boss.png")]
    pub demon_boss_texture: Handle<Image>,
    #[asset(texture_atlas(tile_size_x = 288, tile_size_y = 160, columns = 22, rows = 5))]
    pub demon_boss_layout: Handle<TextureAtlasLayout>,
    #[asset(
        paths(
            "enemy/enemy_boss.trickfilm#idle",
            "enemy/enemy_boss.trickfilm#casting",
            "enemy/enemy_boss.trickfilm#walking",
            "enemy/enemy_boss.trickfilm#striking",
            "enemy/enemy_boss.trickfilm#staggering",
            "enemy/enemy_boss.trickfilm#dying",
        ),
        collection(typed)
    )]
    pub demon_boss_animations: Vec<Handle<AnimationClip2D>>,
    #[asset(path = "enemy/enemy_boss_shadow.png")]
    pub demon_boss_shadow: Handle<Image>,
    #[asset(path = "enemy/enemy_boss_aura.png")]
    pub demon_boss_aura: Handle<Image>,

    // --- SPELL ---
    #[asset(path = "spell/fireball.png")]
    pub fireball_texture: Handle<Image>,
    #[asset(texture_atlas(tile_size_x = 70, tile_size_y = 11, columns = 10, rows = 6))]
    pub fireball_layout: Handle<TextureAtlasLayout>,

    #[asset(path = "spell/lightning.png")]
    pub lightning_texture: Handle<Image>,
    #[asset(texture_atlas(tile_size_x = 32, tile_size_y = 112, columns = 12, rows = 1))]
    pub lightning_layout: Handle<TextureAtlasLayout>,

    #[asset(path = "spell/lightning_strike.png")]
    pub lightning_strike_texture: Handle<Image>,
    #[asset(texture_atlas(tile_size_x = 66, tile_size_y = 66, columns = 13, rows = 1))]
    pub lightning_strike_layout: Handle<TextureAtlasLayout>,
    #[asset(paths("spell/lightning_strike.trickfilm#main"), collection(typed))]
    pub lightning_strike_animations: Vec<Handle<AnimationClip2D>>,

    #[asset(path = "spell/lightning_bird.png")]
    pub lightning_bird_texture: Handle<Image>,
    #[asset(texture_atlas(tile_size_x = 50, tile_size_y = 50, columns = 16, rows = 1))]
    pub lightning_bird_layout: Handle<TextureAtlasLayout>,
    #[asset(
        paths(
            "spell/lightning_bird.trickfilm#spawn",
            "spell/lightning_bird.trickfilm#fly"
        ),
        collection(typed)
    )]
    pub lightning_bird_animations: Vec<Handle<AnimationClip2D>>,
    #[asset(path = "spell/lightning_bird_death.png")]
    pub lightning_bird_death_texture: Handle<Image>,
    #[asset(texture_atlas(tile_size_x = 34, tile_size_y = 34, columns = 6, rows = 1))]
    pub lightning_bird_death_layout: Handle<TextureAtlasLayout>,
    #[asset(paths("spell/lightning_bird_death.trickfilm#main"), collection(typed))]
    pub lightning_bird_death_animations: Vec<Handle<AnimationClip2D>>,
    #[asset(path = "spell/lightning_bird_shadow.png")]
    pub lightning_bird_shadow: Handle<Image>,

    #[asset(path = "spell/aer_tracto.png")]
    pub aer_tracto_texture: Handle<Image>,
    #[asset(texture_atlas(tile_size_x = 48, tile_size_y = 48, columns = 7, rows = 1))]
    pub aer_tracto_layout: Handle<TextureAtlasLayout>,

    #[asset(path = "spell/icicle.png")]
    pub icicle_texture: Handle<Image>,
    #[asset(texture_atlas(tile_size_x = 64, tile_size_y = 64, columns = 30, rows = 1))]
    pub icicle_layout: Handle<TextureAtlasLayout>,
    #[asset(path = "spell/icicle_shatter.png")]
    pub icicle_shatter_texture: Handle<Image>,
    #[asset(texture_atlas(tile_size_x = 96, tile_size_y = 96, columns = 49, rows = 1))]
    pub icicle_shatter_layout: Handle<TextureAtlasLayout>,

    #[asset(path = "spell/death.png")]
    pub death_texture: Handle<Image>,
    #[asset(texture_atlas(tile_size_x = 80, tile_size_y = 96, columns = 18, rows = 1))]
    pub death_layout: Handle<TextureAtlasLayout>,

    #[asset(path = "spell/earth_wall.png")]
    pub earth_wall_texture: Handle<Image>,
    #[asset(texture_atlas(tile_size_x = 48, tile_size_y = 48, columns = 4, rows = 4))]
    pub earth_wall_layout: Handle<TextureAtlasLayout>,
    #[asset(
        paths(
            "spell/earth_wall.trickfilm#spawn",
            "spell/earth_wall.trickfilm#despawn"
        ),
        collection(typed)
    )]
    pub earth_wall_animations: Vec<Handle<AnimationClip2D>>,

    #[asset(path = "spell/explosion.png")]
    pub demon_boss_explosion_texture: Handle<Image>,
    #[asset(texture_atlas(tile_size_x = 64, tile_size_y = 64, columns = 4, rows = 4))]
    pub demon_boss_explosion_layout: Handle<TextureAtlasLayout>,
    #[asset(
        paths("spell/explosion.trickfilm#blink", "spell/explosion.trickfilm#explode"),
        collection(typed)
    )]
    pub demon_boss_explosion_animations: Vec<Handle<AnimationClip2D>>,
    #[asset(path = "spell/explosion2.png")]
    pub demon_boss_explosion2_texture: Handle<Image>,
    #[asset(texture_atlas(tile_size_x = 50, tile_size_y = 50, columns = 18, rows = 1))]
    pub demon_boss_explosion2_layout: Handle<TextureAtlasLayout>,
    #[asset(paths("spell/explosion2.trickfilm#main"), collection(typed))]
    pub demon_boss_explosion2_animations: Vec<Handle<AnimationClip2D>>,
    #[asset(path = "spell/smoke.png")]
    pub demon_boss_smoke_texture: Handle<Image>,
    #[asset(texture_atlas(tile_size_x = 64, tile_size_y = 64, columns = 12, rows = 1))]
    pub demon_boss_smoke_layout: Handle<TextureAtlasLayout>,
    #[asset(paths("spell/smoke.trickfilm#main"), collection(typed))]
    pub demon_boss_smoke_animations: Vec<Handle<AnimationClip2D>>,

    #[asset(path = "spell/portal.png")]
    pub portal_texture: Handle<Image>,
    #[asset(texture_atlas(tile_size_x = 64, tile_size_y = 64, columns = 8, rows = 2))]
    pub portal_layout: Handle<TextureAtlasLayout>,
    #[asset(
        paths("spell/portal.trickfilm#spawn", "spell/portal.trickfilm#idle"),
        collection(typed)
    )]
    pub portal_animations: Vec<Handle<AnimationClip2D>>,

    // --- MAP ---
    #[asset(path = "map/level.ldtk")]
    pub level: Handle<LdtkProject>,
    #[asset(path = "map/statue.png")]
    pub statue: Handle<Image>,
    #[asset(path = "map/statue_blink.png")]
    pub statue_blink_texture: Handle<Image>,
    #[asset(texture_atlas(tile_size_x = 32, tile_size_y = 32, columns = 5, rows = 1))]
    pub statue_blink_layout: Handle<TextureAtlasLayout>,
    #[asset(path = "map/statue_beam.png")]
    pub statue_beam_texture: Handle<Image>,
    #[asset(texture_atlas(tile_size_x = 32, tile_size_y = 64, columns = 4, rows = 1))]
    pub statue_beam_layout: Handle<TextureAtlasLayout>,
    #[asset(path = "map/platform.png")]
    pub platform_texture: Handle<Image>,
    #[asset(texture_atlas(tile_size_x = 112, tile_size_y = 80, columns = 3, rows = 1))]
    pub platform_layout: Handle<TextureAtlasLayout>,
    #[asset(
        paths("map/platform.trickfilm#idle", "map/platform.trickfilm#trigger",),
        collection(typed)
    )]
    pub platform_animations: Vec<Handle<AnimationClip2D>>,
    #[asset(path = "map/platform_item.png")]
    pub platform_item: Handle<Image>,
    #[asset(path = "map/platform_item_shadow.png")]
    pub platform_item_shadow: Handle<Image>,
    #[asset(path = "map/platform_item_highlight.png")]
    pub platform_item_highlight: Handle<Image>,

    // --- UI ---
    #[asset(path = "ui/white_pixel.png")]
    pub white_pixel: Handle<Image>,
    #[asset(path = "ui/heart_full.png")]
    pub heart_full: Handle<Image>,
    #[asset(path = "ui/heart_empty.png")]
    pub heart_empty: Handle<Image>,
    #[asset(path = "ui/statue_icon.png")]
    pub statue_ui_icon: Handle<Image>,
    #[asset(path = "ui/keys.png")]
    pub keyboard_ui_texture: Handle<Image>,
    #[asset(texture_atlas(tile_size_x = 34, tile_size_y = 34, columns = 3, rows = 6))]
    pub keyboard_ui_layout: Handle<TextureAtlasLayout>,
    #[asset(path = "ui/vignette.png")]
    pub vignette: Handle<Image>,
    #[asset(path = "ui/platform_arrow.png")]
    pub platform_arrow: Handle<Image>,

    #[asset(path = "ui/movement_hint_up.png")]
    pub spell_book_hint_up: Handle<Image>,
    #[asset(path = "ui/movement_hint_down.png")]
    pub spell_book_hint_down: Handle<Image>,
    #[asset(path = "ui/spell_book_view.png")]
    pub spell_book_view: Handle<Image>,
    #[asset(path = "ui/spell_book_container.png")]
    pub spell_book_container: Handle<Image>,
    #[asset(path = "ui/spell_field.png")]
    pub spell_field: Handle<Image>,
    #[asset(path = "ui/spell_field_selector.png")]
    pub spell_field_selector: Handle<Image>,

    #[asset(path = "ui/spell_console_icon.png")]
    pub spell_console_icon: Handle<Image>,
    #[asset(path = "ui/fulgur_icon.png")]
    pub fulgur_icon: Handle<Image>,
    #[asset(path = "ui/fulgur_avis_icon.png")]
    pub fulgur_avis_icon: Handle<Image>,
    #[asset(path = "ui/ignis_pila_icon.png")]
    pub ignis_pila_icon: Handle<Image>,
    #[asset(path = "ui/inferno_pila_icon.png")]
    pub inferno_pila_icon: Handle<Image>,
    #[asset(path = "ui/scutum_glaciei_icon.png")]
    pub scutum_glaciei_icon: Handle<Image>,
    #[asset(path = "ui/aer_tracto_icon.png")]
    pub aer_tracto_icon: Handle<Image>,
    #[asset(path = "ui/aer_pello_icon.png")]
    pub aer_pello_icon: Handle<Image>,
    #[asset(path = "ui/placeholder_icon.png")]
    pub placeholder_icon: Handle<Image>,

    // --- SOUND ---
    #[asset(path = "sounds/player_footstep.ogg")]
    pub player_step_sound: Handle<AudioSource>,

    #[asset(path = "sounds/slime_jump.ogg")]
    pub slime_jump_sound: Handle<AudioSource>,
    #[asset(path = "sounds/slime_land.ogg")]
    pub slime_land_sound: Handle<AudioSource>,
    #[asset(path = "sounds/slime_hit.ogg")]
    pub slime_hit_sound: Handle<AudioSource>,
    #[asset(path = "sounds/slime_death.ogg")]
    pub slime_death_sound: Handle<AudioSource>,

    #[asset(path = "sounds/demon_boss_step.ogg")]
    pub demon_boss_step_sound: Handle<AudioSource>,
    #[asset(path = "sounds/demon_boss_vocal_explosion.ogg")]
    pub demon_boss_vocal_explosion_sound: Handle<AudioSource>,
    #[asset(path = "sounds/demon_boss_vocal_earth_prison.ogg")]
    pub demon_boss_vocal_earth_prison_sound: Handle<AudioSource>,
    #[asset(path = "sounds/gitgud.ogg")]
    pub git_gud: Handle<AudioSource>,

    #[asset(path = "sounds/lightning_bird_flap.ogg")]
    pub lightning_bird_flap_sound: Handle<AudioSource>,

    #[asset(path = "sounds/item_unlock.ogg")]
    pub item_unlock_sound: Handle<AudioSource>,
    #[asset(path = "sounds/earth_wall.ogg")]
    pub earth_wall_sound: Handle<AudioSource>,

    #[asset(path = "sounds/game_won.ogg")]
    pub game_won_sound: Handle<AudioSource>,
    #[asset(path = "sounds/game_over.ogg")]
    pub game_over_sound: Handle<AudioSource>,

    // --- MUSIC ---
    #[asset(path = "music/bgm.ogg")]
    pub bgm: Handle<AudioSource>,
    #[asset(path = "music/bgm_boss.ogg")]
    pub bgm_boss: Handle<AudioSource>,

    // --- FONT ---
    #[asset(path = "fonts/PressStart2P.ttf")]
    pub font: Handle<Font>,
}
