use std::f32::consts::TAU;

use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use bevy_trickfilm::prelude::*;

use crate::{
    audio::PlaySound, item::platform::TriggerFinalAct, player::PLAYER_SPAWN_POS,
    world::camera::YSort, GameAssets, GameState,
};

const WALL_PADDING: f32 = 10.0;
const COUNT: usize = 100;
const RADIUS: f32 = 300.0;

fn spawn_wall(commands: &mut Commands, assets: &Res<GameAssets>, pos: Vec3, flip_x: bool) {
    let mut animator = AnimationPlayer2D::default();
    animator.play(assets.earth_wall_animations[0].clone());

    commands.spawn((
        animator,
        YSort(0.0),
        Sprite {
            image: assets.earth_wall_texture.clone(),
            flip_x,
            texture_atlas: Some(TextureAtlas {
                layout: assets.earth_wall_layout.clone(),
                ..default()
            }),
            ..default()
        },
        Transform::from_translation(pos),
    ));
}

fn spawn_collider(commands: &mut Commands, pos: Vec3, offset: f32) {
    let mut vertices = Vec::new();
    for i in 0..COUNT {
        let rot = Quat::from_rotation_z(TAU * i as f32 / COUNT as f32);
        let pos = rot.mul_vec3(Vec3::X * offset - WALL_PADDING);
        vertices.push(Vect::new(pos.x, pos.y));
    }
    vertices.push(Vect::new(offset - WALL_PADDING, 0.0));

    commands.spawn((
        Collider::polyline(vertices, None),
        CollisionGroups::default(),
        Transform::from_translation(pos),
    ));
}

fn spawn_walls(
    mut commands: Commands,
    assets: Res<GameAssets>,
    mut ev_trigger_final_act: EventReader<TriggerFinalAct>,
    mut ev_play_sound: EventWriter<PlaySound>,
) {
    if ev_trigger_final_act.is_empty() {
        return;
    }
    ev_trigger_final_act.clear();

    ev_play_sound.write(PlaySound {
        clip: assets.earth_wall_sound.clone(),
        ..default()
    });

    spawn_collider(&mut commands, PLAYER_SPAWN_POS, RADIUS);
    for i in 0..COUNT {
        let rot = Quat::from_rotation_z(TAU * i as f32 / COUNT as f32);
        let pos = PLAYER_SPAWN_POS + rot.mul_vec3(Vec3::X * RADIUS - WALL_PADDING);
        let flip_x = rot.to_euler(EulerRot::ZYX).0.abs() < TAU / 4.0;
        spawn_wall(&mut commands, &assets, pos, flip_x);
    }
}

pub struct DemonBossWallPlugin;

impl Plugin for DemonBossWallPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (spawn_walls,).run_if(in_state(GameState::Gaming)));
    }
}
