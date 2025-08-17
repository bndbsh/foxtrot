//! [Landmass](https://github.com/andriyDev/landmass) powers out agent navigation.
//! The underlying navmesh is generated using [Oxidized Navigation](https://github.com/TheGrimsey/oxidized_navigation).

use std::sync::Arc;

use crate::{
    gameplay::{level::LevelAssets, npc::NPC_RADIUS},
    screens::Screen,
};
use bevy::prelude::*;
use bevy_landmass::{ArchipelagoRef, HeightPolygon, PointSampleDistance3d, prelude::*};
use bevy_rerecast::rerecast::PolygonNavmesh;
#[cfg(feature = "hot_patch")]
use bevy_simple_subsecond_system::hot;
use landmass_rerecast::{Island3dBundle, LandmassRerecastPlugin, NavMeshHandle3d};

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((
        Landmass3dPlugin::default(),
        LandmassRerecastPlugin::default(),
    ));
    app.add_systems(Startup, setup_archipelago);
    app.add_systems(OnEnter(Screen::Gameplay), setup_island);
}

#[cfg_attr(feature = "hot_patch", hot)]
fn setup_archipelago(mut commands: Commands) {
    commands.spawn((
        Name::new("Main Level Archipelago"),
        Archipelago3d::new(AgentOptions {
            point_sample_distance: PointSampleDistance3d {
                horizontal_distance: 0.6,
                distance_above: 1.0,
                distance_below: 1.0,
                vertical_preference_ratio: 2.0,
            },
            ..AgentOptions::from_agent_radius(NPC_RADIUS)
        }),
    ));
}

#[cfg_attr(feature = "hot_patch", hot)]
fn setup_island(
    mut commands: Commands,
    archipelago: Single<Entity, With<Archipelago3d>>,
    level_assets: Res<LevelAssets>,
) {
    commands.spawn((
        StateScoped(Screen::Gameplay),
        Name::new("Main Level Island"),
        Island3dBundle {
            island: Island,
            archipelago_ref: ArchipelagoRef3d::new(archipelago.into_inner()),
            nav_mesh: NavMeshHandle3d(level_assets.navmesh.clone()),
        },
    ));
}
