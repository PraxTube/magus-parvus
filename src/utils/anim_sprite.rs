use bevy::prelude::*;

#[derive(Component, Default)]
pub struct AnimSprite {
    sprites: usize,
    repeating: bool,
    disabled: bool,
}

#[derive(Component)]
pub struct AnimSpriteTimer {
    timer: Timer,
}

#[derive(Component)]
pub struct AnimationIndices {
    pub first: usize,
    pub last: usize,
}

#[derive(Component, Deref, DerefMut)]
pub struct FrameTimer(pub Timer);

impl AnimSprite {
    /// The number of sprites starts at 1, so if there are 4 sprites in the sheet
    /// then this function expects the number 4.
    pub fn new(sprites: usize, repeating: bool) -> Self {
        Self {
            sprites: sprites - 1,
            repeating,
            disabled: false,
        }
    }
}

impl Default for AnimSpriteTimer {
    fn default() -> Self {
        Self {
            timer: Timer::from_seconds(0.075, TimerMode::Repeating),
        }
    }
}

impl AnimSpriteTimer {
    pub fn new(seconds: f32) -> Self {
        Self {
            timer: Timer::from_seconds(seconds, TimerMode::Repeating),
        }
    }
}

fn animate_sprites(
    time: Res<Time>,
    mut query: Query<(
        &mut AnimSprite,
        &mut AnimSpriteTimer,
        &mut TextureAtlasSprite,
    )>,
) {
    for (mut asprite, mut timer, mut sprite) in &mut query {
        if sprite.index > asprite.sprites {
            asprite.disabled = true;
            continue;
        }

        timer.timer.tick(time.delta());
        if timer.timer.just_finished() {
            if sprite.index == asprite.sprites {
                if asprite.repeating {
                    sprite.index = 0;
                } else {
                    asprite.disabled = true;
                }
            } else {
                sprite.index += 1;
            }
        }
    }
}

fn animate_complex_sprites(
    time: Res<Time>,
    mut q_sprites: Query<(&AnimationIndices, &mut FrameTimer, &mut TextureAtlasSprite)>,
) {
    for (indices, mut timer, mut sprite) in &mut q_sprites {
        timer.tick(time.delta());
        if timer.just_finished() {
            sprite.index = if sprite.index == indices.last {
                indices.first
            } else {
                sprite.index + 1
            };
        }
    }
}

fn despawn_anim_sprites(mut commands: Commands, q_anim_sprites: Query<(Entity, &AnimSprite)>) {
    for (entity, asprite) in &q_anim_sprites {
        if asprite.disabled {
            commands.entity(entity).despawn_recursive();
        }
    }
}

pub struct AnimSpritePlugin;

impl Plugin for AnimSpritePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                animate_sprites,
                animate_complex_sprites,
                despawn_anim_sprites,
            ),
        );
    }
}
