use bevy::prelude::*;
use bevy_kira_audio::prelude::*;

const MAX_DISTANCE: f64 = 1000.0;

fn update(
    receiver_transform: &GlobalTransform,
    emitters: &Query<(&GlobalTransform, &AudioEmitter)>,
    audio_instances: &mut Assets<AudioInstance>,
) {
    for (emitter_transform, emitter) in emitters {
        let sound_path = emitter_transform.translation().truncate()
            - receiver_transform.translation().truncate();
        let volume: f64 = (1.0 - sound_path.length_squared() as f64 / MAX_DISTANCE.powi(2))
            .clamp(0.0, 1.0)
            .powi(2);

        for instance in emitter.instances.iter() {
            if let Some(instance) = audio_instances.get_mut(instance) {
                instance.set_volume(volume, AudioTween::default());
            }
        }
    }
}

fn update_volumes(
    receiver: Query<&GlobalTransform, With<AudioReceiver>>,
    emitters: Query<(&GlobalTransform, &AudioEmitter)>,
    mut audio_instances: ResMut<Assets<AudioInstance>>,
) {
    if let Ok(receiver_transform) = receiver.get_single() {
        update(receiver_transform, &emitters, &mut audio_instances);
    }
}

fn cleanup_stopped_spacial_instances(
    mut emitters: Query<&mut AudioEmitter>,
    instances: ResMut<Assets<AudioInstance>>,
) {
    for mut emitter in emitters.iter_mut() {
        let handles = &mut emitter.instances;

        handles.retain(|handle| {
            if let Some(instance) = instances.get(handle) {
                instance.state() != PlaybackState::Stopped
            } else {
                true
            }
        });
    }
}

pub struct SpacialAudioPlugin;

impl Plugin for SpacialAudioPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (update_volumes, cleanup_stopped_spacial_instances));
    }
}
