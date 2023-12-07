pub mod damage_number;
pub mod health;
mod pop_up;
pub mod text_field;

use bevy::prelude::*;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            health::HealthPlugin,
            damage_number::DamageNumberPlugin,
            text_field::TextFieldPlugin,
            pop_up::PopUpPlugin,
        ));
    }
}
