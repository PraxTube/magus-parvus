pub mod anim_sprite;

mod diagnostics;

use bevy::prelude::*;
use bevy_rapier2d::{plugin::RapierTransformPropagateSet, prelude::*};

pub const COLLISION_GROUPS_NONE: CollisionGroups = CollisionGroups::new(Group::NONE, Group::NONE);

pub struct UtilsPlugin;

impl Plugin for UtilsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            anim_sprite::AnimSpritePlugin,
            diagnostics::DiagnosticsPlugin,
        ))
        .add_systems(
            PostUpdate,
            reset_rotations.before(RapierTransformPropagateSet),
        );
    }
}

#[derive(Component)]
pub struct NoRotation {
    pub offset: Vec3,
}

#[allow(dead_code)]
pub fn quat_from_vec2(direction: Vec2) -> Quat {
    if direction == Vec2::ZERO {
        return Quat::IDENTITY;
    }
    Quat::from_euler(EulerRot::XYZ, 0.0, 0.0, Vec2::X.angle_between(direction))
}

#[allow(dead_code)]
pub fn quat_from_vec3(direction: Vec3) -> Quat {
    quat_from_vec2(direction.truncate())
}

/// there is no way to inherit position but not rotation from the parent entity transform yet
/// see: https://github.com/bevyengine/bevy/issues/1780
fn reset_rotations(
    mut q_transforms: Query<(&Parent, &mut Transform, &NoRotation)>,
    q_parents: Query<&Transform, Without<NoRotation>>,
) {
    for (parent, mut transform, no_rotation) in q_transforms.iter_mut() {
        if let Ok(parent_transform) = q_parents.get(parent.get()) {
            let rot_inv = parent_transform.rotation.inverse();
            transform.rotation = rot_inv;
            transform.translation = rot_inv.mul_vec3(no_rotation.offset);
        }
    }
}
