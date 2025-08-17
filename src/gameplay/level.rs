//! Spawn the main level.

use crate::{
    asset_tracking::LoadResource, audio::MusicPool, gameplay::npc::NPC_RADIUS, screens::Screen,
};
use bevy::prelude::*;
use bevy_landmass::{PointSampleDistance3d, prelude::*};
use bevy_rerecast::prelude::*;
use bevy_seedling::prelude::*;
use bevy_seedling::sample::Sample;
#[cfg(feature = "hot_patch")]
use bevy_simple_subsecond_system::hot;
use landmass_rerecast::{Island3dBundle, NavMeshHandle3d};

pub(super) fn plugin(app: &mut App) {
    app.load_resource::<LevelAssets>();
    app.register_type::<Level>();
}

/// A system that spawns the main level.
#[cfg_attr(feature = "hot_patch", hot)]
pub(crate) fn spawn_level(mut commands: Commands, level_assets: Res<LevelAssets>) {
    commands.spawn((
        Name::new("Level"),
        SceneRoot(level_assets.level.clone()),
        StateScoped(Screen::Gameplay),
        Level,
        children![(
            Name::new("Level Music"),
            SamplePlayer::new(level_assets.music.clone()).looping(),
            MusicPool
        )],
    ));
    commands.insert_resource(AmbientLight::NONE);

    let archipelago = commands
        .spawn((
            Name::new("Main Level Archipelago"),
            StateScoped(Screen::Gameplay),
            Archipelago3d::new(AgentOptions {
                point_sample_distance: PointSampleDistance3d {
                    horizontal_distance: 0.6,
                    distance_above: 1.0,
                    distance_below: 1.0,
                    vertical_preference_ratio: 2.0,
                },
                ..AgentOptions::from_agent_radius(NPC_RADIUS)
            }),
        ))
        .id();

    commands.spawn((
        Name::new("Main Level Island"),
        StateScoped(Screen::Gameplay),
        Island3dBundle {
            island: Island,
            archipelago_ref: ArchipelagoRef3d::new(archipelago),
            nav_mesh: NavMeshHandle3d(level_assets.navmesh.clone()),
        },
    ));
}

#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub(crate) struct Level;

/// A [`Resource`] that contains all the assets needed to spawn the level.
/// We use this to preload assets before the level is spawned.
#[derive(Resource, Asset, Clone, TypePath)]
pub(crate) struct LevelAssets {
    #[dependency]
    pub(crate) level: Handle<Scene>,
    #[dependency]
    pub(crate) navmesh: Handle<Navmesh>,
    #[dependency]
    pub(crate) music: Handle<Sample>,
    #[dependency]
    pub(crate) env_map_specular: Handle<Image>,
    #[dependency]
    pub(crate) env_map_diffuse: Handle<Image>,
}

impl FromWorld for LevelAssets {
    fn from_world(world: &mut World) -> Self {
        let assets = world.resource::<AssetServer>();

        Self {
            // Our main level is inspired by the TheDarkMod fan mission [Volta I: The Stone](https://www.thedarkmod.com/missiondetails/?internalName=volta1_3)
            level: assets.load("maps/volta_i/volta_i.map#Scene"),
            // You can regenerate the navmesh by using `bevy_rerecast_editor`
            navmesh: assets.load("maps/volta_i/volta_i.nav"),
            music: assets.load("audio/music/Ambiance_Rain_Calm_Loop_Stereo.ogg"),
            env_map_specular: assets.load("cubemaps/NightSkyHDRI001_4K-HDR_specular.ktx2"),
            env_map_diffuse: assets.load("cubemaps/NightSkyHDRI001_4K-HDR_diffuse.ktx2"),
        }
    }
}
