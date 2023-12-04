mod assets;
mod enemy;
mod player;
mod spell;
mod ui;
mod utils;
mod world;

pub use assets::GameAssets;

use bevy::prelude::*;
use bevy::window::{PresentMode, Window, WindowMode};

use bevy_asset_loader::prelude::*;
use bevy_rapier2d::prelude::*;
use bevy_screen_diagnostics::{
    ScreenDiagnosticsPlugin, ScreenEntityDiagnosticsPlugin, ScreenFrameDiagnosticsPlugin,
};

#[derive(States, Clone, Eq, PartialEq, Debug, Hash, Default)]
pub enum GameState {
    #[default]
    AssetLoading,
    Gaming,
}

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        present_mode: PresentMode::Fifo,
                        mode: WindowMode::Windowed,
                        ..default()
                    }),
                    ..default()
                })
                .set(ImagePlugin::default_nearest())
                .build(),
            ScreenDiagnosticsPlugin {
                timestep: 1.0,
                style: Style {
                    position_type: PositionType::Absolute,
                    top: Val::Px(5.0),
                    left: Val::Px(15.0),
                    ..default()
                },
                ..default()
            },
            ScreenFrameDiagnosticsPlugin,
            ScreenEntityDiagnosticsPlugin,
            RapierPhysicsPlugin::<NoUserData>::default(),
            RapierDebugRenderPlugin::default(),
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
            enemy::EnemyPlugin,
            player::PlayerPlugin,
        ))
        .run();
}
