pub mod anim_sprite;

use bevy::prelude::*;

pub struct UtilsPlugin;

impl Plugin for UtilsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(anim_sprite::AnimSpritePlugin);
    }
}

#[allow(dead_code)]
pub fn quat_from_vec2(direction: Vec2) -> Quat {
    if direction == Vec2::default() {
        return Quat::IDENTITY;
    }
    Quat::from_euler(EulerRot::XYZ, 0.0, 0.0, Vec2::X.angle_between(direction))
}

#[allow(dead_code)]
pub fn quat_from_vec3(direction: Vec3) -> Quat {
    quat_from_vec2(direction.truncate())
}
