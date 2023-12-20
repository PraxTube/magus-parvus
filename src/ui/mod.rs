pub mod health;
pub mod text_field;
pub mod world_text;

mod game_over_ui;
mod keyboard_ui;
mod pop_up;
mod spell_book;
mod statue_counter;
mod vignette;

use bevy::prelude::*;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            health::HealthPlugin,
            world_text::WorldTextPlugin,
            text_field::TextFieldPlugin,
            pop_up::PopUpPlugin,
            keyboard_ui::KeyboardUiPlugin,
            game_over_ui::GameOverUiPlugin,
            spell_book::SpellBookPlugin,
            statue_counter::StatueCounterUiPlugin,
            vignette::VignettePlugin,
        ));
    }
}
