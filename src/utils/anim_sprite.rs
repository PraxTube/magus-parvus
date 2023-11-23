use bevy::prelude::*;

use crate::GameState;

#[derive(Component, Default)]
pub struct AnimSprite {
    sprites: usize,
    repeating: bool,
    disabled: bool,
}

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

#[derive(Component)]
pub struct AnimSpriteTimer {
    timer: Timer,
}

impl Default for AnimSpriteTimer {
    fn default() -> Self {
        Self {
            timer: Timer::from_seconds(0.075, TimerMode::Repeating),
        }
    }
}

#[allow(dead_code)]
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
            (animate_sprites, despawn_anim_sprites).run_if(in_state(GameState::Gaming)),
        );
    }
}
