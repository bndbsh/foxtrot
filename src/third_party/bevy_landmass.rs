//! [Landmass](https://github.com/andriyDev/landmass) powers out agent navigation.
//! The underlying navmesh is generated using [Oxidized Navigation](https://github.com/TheGrimsey/oxidized_navigation).

use crate::{
    gameplay::{level::LevelAssets, npc::NPC_RADIUS},
    screens::{Screen, loading::LoadingScreen},
};
use bevy::prelude::*;
use bevy_landmass::{PointSampleDistance3d, prelude::*};
#[cfg(feature = "hot_patch")]
use bevy_simple_subsecond_system::hot;
use landmass_rerecast::{Island3dBundle, LandmassRerecastPlugin, NavMeshHandle3d};

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((
        Landmass3dPlugin::default(),
        LandmassRerecastPlugin::default(),
    ));
    app.add_systems(OnEnter(LoadingScreen::Level), setup_archipelago);
}

#[cfg_attr(feature = "hot_patch", hot)]
fn setup_archipelago(mut commands: Commands, level_assets: Res<LevelAssets>) {
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
