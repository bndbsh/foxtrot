use bevy::prelude::*;
use bevy_seedling::prelude::*;

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
pub(crate) struct UiSfx;

#[derive(PoolLabel, Reflect, PartialEq, Eq, Debug, Hash, Clone)]
#[reflect(Component)]
pub(crate) struct Music;

/// Set somewhere below 0 dB so that the user can turn the volume up if they want to.
pub(crate) const DEFAULT_MAIN_VOLUME: Volume = Volume::Linear(0.5);

fn initialize_audio(mut master: Single<&mut VolumeNode, With<MainBus>>, mut commands: Commands) {
    master.volume = DEFAULT_MAIN_VOLUME;
    // Tuned by ear
    const DEFAULT_POOL_VOLUME: Volume = Volume::Linear(1.2);

    // For each new pool, we can provide non-default initial values for the volume.
    commands.spawn((
        Name::new("Music audio sampler pool"),
        SamplerPool(Music),
        VolumeNode {
            volume: DEFAULT_POOL_VOLUME,
        },
    ));
    commands.spawn((
        Name::new("SFX audio sampler pool"),
        SamplerPool(Sfx),
        sample_effects![(
            SpatialBasicNode {
                panning_threshold: 1.0,
                ..default()
            },
            SpatialScale(Vec3::splat(2.0))
        )],
        VolumeNode {
            volume: DEFAULT_POOL_VOLUME,
        },
    ));
    commands.spawn((
        Name::new("UI SFX audio sampler pool"),
        SamplerPool(UiSfx),
        VolumeNode {
            volume: DEFAULT_POOL_VOLUME,
        },
    ));
}
