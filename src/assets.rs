use bevy::prelude::*;
use bevy_asset_loader::prelude::*;
use bevy_ecs_ldtk::prelude::LdtkProject;
use bevy_kira_audio::AudioSource;
use bevy_trickfilm::prelude::*;

#[derive(AssetCollection, Resource)]
pub struct GameAssets {
    #[asset(texture_atlas(tile_size_x = 32.0, tile_size_y = 32.0, columns = 6, rows = 4))]
    #[asset(path = "mage.png")]
    pub player: Handle<TextureAtlas>,

    #[asset(texture_atlas(tile_size_x = 48.0, tile_size_y = 48.0, columns = 6, rows = 4))]
    #[asset(path = "enemy/enemy.png")]
    pub enemy: Handle<TextureAtlas>,

    #[asset(texture_atlas(tile_size_x = 288.0, tile_size_y = 160.0, columns = 22, rows = 5))]
    #[asset(path = "enemy/enemy_boss.png")]
    pub enemy_boss: Handle<TextureAtlas>,
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
    pub enemy_boss_animations: Vec<Handle<AnimationClip2D>>,
    #[asset(path = "enemy/enemy_boss_shadow.png")]
    pub enemy_boss_shadow: Handle<Image>,
    #[asset(path = "enemy/enemy_boss_aura.png")]
    pub enemy_boss_aura: Handle<Image>,

    // --- SPELL ---
    #[asset(texture_atlas(tile_size_x = 70.0, tile_size_y = 11.0, columns = 10, rows = 6))]
    #[asset(path = "spell/fireball.png")]
    pub fireball: Handle<TextureAtlas>,

    #[asset(texture_atlas(tile_size_x = 32.0, tile_size_y = 112.0, columns = 12, rows = 1))]
    #[asset(path = "spell/lightning.png")]
    pub lightning: Handle<TextureAtlas>,

    #[asset(texture_atlas(tile_size_x = 66.0, tile_size_y = 66.0, columns = 13, rows = 1))]
    #[asset(path = "spell/lightning_strike.png")]
    pub lightning_strike: Handle<TextureAtlas>,
    #[asset(paths("spell/lightning_strike.trickfilm#main"), collection(typed))]
    pub lightning_strike_animations: Vec<Handle<AnimationClip2D>>,

    #[asset(texture_atlas(tile_size_x = 50.0, tile_size_y = 50.0, columns = 16, rows = 1))]
    #[asset(path = "spell/lightning_bird.png")]
    pub lightning_bird: Handle<TextureAtlas>,
    #[asset(
        paths(
            "spell/lightning_bird.trickfilm#spawn",
            "spell/lightning_bird.trickfilm#fly"
        ),
        collection(typed)
    )]
    pub lightning_bird_animations: Vec<Handle<AnimationClip2D>>,
    #[asset(texture_atlas(tile_size_x = 34.0, tile_size_y = 34.0, columns = 6, rows = 1))]
    #[asset(path = "spell/lightning_bird_death.png")]
    pub lightning_bird_death: Handle<TextureAtlas>,
    #[asset(paths("spell/lightning_bird_death.trickfilm#main"), collection(typed))]
    pub lightning_bird_death_animations: Vec<Handle<AnimationClip2D>>,
    #[asset(path = "spell/lightning_bird_shadow.png")]
    pub lightning_bird_shadow: Handle<Image>,

    #[asset(texture_atlas(tile_size_x = 48.0, tile_size_y = 48.0, columns = 7, rows = 1))]
    #[asset(path = "spell/aer_tracto.png")]
    pub aer_tracto: Handle<TextureAtlas>,

    #[asset(texture_atlas(tile_size_x = 64.0, tile_size_y = 64.0, columns = 30, rows = 1))]
    #[asset(path = "spell/icicle.png")]
    pub icicle: Handle<TextureAtlas>,
    #[asset(texture_atlas(tile_size_x = 96.0, tile_size_y = 96.0, columns = 49, rows = 1))]
    #[asset(path = "spell/icicle_shatter.png")]
    pub icicle_shatter: Handle<TextureAtlas>,

    #[asset(texture_atlas(tile_size_x = 80.0, tile_size_y = 96.0, columns = 18, rows = 1))]
    #[asset(path = "spell/death.png")]
    pub death: Handle<TextureAtlas>,

    #[asset(texture_atlas(tile_size_x = 48.0, tile_size_y = 48.0, columns = 4, rows = 4))]
    #[asset(path = "spell/earth_wall.png")]
    pub earth_wall: Handle<TextureAtlas>,
    #[asset(
        paths(
            "spell/earth_wall.trickfilm#spawn",
            "spell/earth_wall.trickfilm#despawn"
        ),
        collection(typed)
    )]
    pub earth_wall_animations: Vec<Handle<AnimationClip2D>>,

    #[asset(texture_atlas(tile_size_x = 64.0, tile_size_y = 64.0, columns = 4, rows = 4))]
    #[asset(path = "spell/explosion.png")]
    pub demon_boss_explosion: Handle<TextureAtlas>,
    #[asset(
        paths("spell/explosion.trickfilm#blink", "spell/explosion.trickfilm#explode"),
        collection(typed)
    )]
    pub demon_boss_explosion_animations: Vec<Handle<AnimationClip2D>>,
    #[asset(texture_atlas(tile_size_x = 50.0, tile_size_y = 50.0, columns = 18, rows = 1))]
    #[asset(path = "spell/explosion2.png")]
    pub demon_boss_explosion2: Handle<TextureAtlas>,
    #[asset(paths("spell/explosion2.trickfilm#main"), collection(typed))]
    pub demon_boss_explosion2_animations: Vec<Handle<AnimationClip2D>>,

    // --- MAP ---
    #[asset(path = "map/level.ldtk")]
    pub level: Handle<LdtkProject>,
    #[asset(path = "map/statue.png")]
    pub statue: Handle<Image>,
    #[asset(texture_atlas(tile_size_x = 32.0, tile_size_y = 32.0, columns = 5, rows = 1))]
    #[asset(path = "map/statue_blink.png")]
    pub statue_blink: Handle<TextureAtlas>,
    #[asset(texture_atlas(tile_size_x = 32.0, tile_size_y = 64.0, columns = 4, rows = 1))]
    #[asset(path = "map/statue_beam.png")]
    pub statue_beam: Handle<TextureAtlas>,

    // --- UI ---
    #[asset(path = "ui/white_pixel.png")]
    pub white_pixel: Handle<Image>,
    #[asset(path = "ui/heart_full.png")]
    pub heart_full: Handle<Image>,
    #[asset(path = "ui/heart_empty.png")]
    pub heart_empty: Handle<Image>,
    #[asset(path = "ui/statue_icon.png")]
    pub statue_ui_icon: Handle<Image>,
    #[asset(texture_atlas(tile_size_x = 34.0, tile_size_y = 34.0, columns = 3, rows = 5))]
    #[asset(path = "ui/keys.png")]
    pub keyboard_ui: Handle<TextureAtlas>,

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
