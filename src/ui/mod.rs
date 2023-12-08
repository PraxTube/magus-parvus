pub mod health;
mod pop_up;
pub mod text_field;
pub mod world_text;

use bevy::prelude::*;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            health::HealthPlugin,
            world_text::WorldTextPlugin,
            text_field::TextFieldPlugin,
            pop_up::PopUpPlugin,
        ));
    }
}
