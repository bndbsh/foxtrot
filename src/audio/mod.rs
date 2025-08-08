use bevy::prelude::*;
use bevy_seedling::{prelude::*, sample::Sample};

pub(crate) mod perceptual;

pub(super) fn plugin(app: &mut App) {
    app.register_type::<Music>();
    app.register_type::<Sfx>();

    app.add_systems(Startup, initialize_audio);
}

#[derive(PoolLabel, Reflect, PartialEq, Eq, Debug, Hash, Clone)]
#[reflect(Component)]
pub(crate) struct Sfx;

#[derive(PoolLabel, Reflect, PartialEq, Eq, Debug, Hash, Clone)]
#[reflect(Component)]
pub(crate) struct Music;

/// A music audio instance.
pub(crate) fn music(handle: Handle<Sample>) -> impl Bundle {
    (SamplePlayer::new(handle).looping(), Music)
}

/// A sound effect audio instance.
pub(crate) fn sound_effect(handle: Handle<Sample>) -> impl Bundle {
    (SamplePlayer::new(handle), Sfx)
}

pub(crate) const DEFAULT_VOLUME: Volume = Volume::Linear(0.3);

fn initialize_audio(mut master: Single<&mut VolumeNode, With<MainBus>>, mut commands: Commands) {
    // Since the main bus already exists, we can just set the desired volume.
    master.volume = Volume::UNITY_GAIN;

    // For each new pool, we can provide non-default initial values for the volume.
    commands.spawn((
        Name::new("Music audio sampler pool"),
        SamplerPool(Music),
        VolumeNode {
            volume: DEFAULT_VOLUME,
        },
    ));
    commands.spawn((
        Name::new("SFX audio sampler pool"),
        SamplerPool(Sfx),
        VolumeNode {
            volume: DEFAULT_VOLUME,
        },
    ));
}
