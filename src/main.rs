#![allow(clippy::too_many_arguments, clippy::type_complexity)]

mod assets;
mod audio;
mod enemy;
mod item;
mod player;
mod spell;
mod ui;
mod utils;
mod world;

pub use assets::GameAssets;

use bevy::asset::AssetMetaCheck;
use bevy::prelude::*;
use bevy::window::{PresentMode, Window, WindowMode};

use bevy_asset_loader::prelude::*;
use bevy_rapier2d::prelude::*;
use bevy_trickfilm::Animation2DPlugin;

const BACKGROUND_COLOR: Color = Color::rgb(0.95, 0.90, 0.75);

#[derive(States, Clone, Eq, PartialEq, Debug, Hash, Default)]
pub enum GameState {
    #[default]
    AssetLoading,
    Gaming,
    GameOver,
    Win,
}

fn main() {
    App::new()
        .insert_resource(AssetMetaCheck::Never)
        .add_plugins((
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        present_mode: PresentMode::Fifo,
                        mode: WindowMode::Windowed,
                        fit_canvas_to_parent: true,
                        ..default()
                    }),
                    ..default()
                })
                .set(ImagePlugin::default_nearest())
                .build(),
            RapierPhysicsPlugin::<NoUserData>::default(),
            RapierDebugRenderPlugin::default(),
            Animation2DPlugin,
        ))
        .insert_resource(Msaa::Off)
        .add_state::<GameState>()
        .add_loading_state(
            LoadingState::new(GameState::AssetLoading).continue_to_state(GameState::Gaming),
        )
        .add_collection_to_loading_state::<_, GameAssets>(GameState::AssetLoading)
        .add_plugins((
            ui::UiPlugin,
            world::WorldPlugin,
            spell::SpellPlugin,
            utils::UtilsPlugin,
            item::ItemPlugin,
            enemy::EnemyPlugin,
            player::PlayerPlugin,
            audio::GameAudioPlugin,
        ))
        .insert_resource(ClearColor(BACKGROUND_COLOR))
        .run();
}
