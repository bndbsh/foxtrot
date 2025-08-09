//! Development tools for the game. This plugin is only enabled in dev builds.

use bevy::{dev_tools::states::log_transitions, prelude::*};
use bevy_rerecast::{debug::DetailNavmeshGizmo, prelude::*};
#[cfg(feature = "hot_patch")]
use bevy_simple_subsecond_system::prelude::*;

mod debug_ui;
#[cfg(feature = "native")]
mod generate_navmesh;
mod input;
mod navmesh_gizmo;
mod validate_preloading;

use crate::{menus::Menu, screens::loading::LoadingScreen};

pub(super) fn plugin(app: &mut App) {
    // Log `Screen` state transitions.
    app.add_systems(
        Update,
        (log_transitions::<Menu>, log_transitions::<LoadingScreen>).chain(),
    );

    app.add_plugins((
        #[cfg(feature = "hot_patch")]
        SimpleSubsecondPlugin::default(),
        // There are nice editor inspectors for VS Code that can connect to the remote server.
        #[cfg(feature = "bevy_remote")]
        (
            bevy::remote::RemotePlugin::default(),
            bevy::remote::http::RemoteHttpPlugin::default(),
        ),
        debug_ui::plugin,
        input::plugin,
        validate_preloading::plugin,
        navmesh_gizmo::plugin,
        #[cfg(feature = "native")]
        generate_navmesh::plugin,
    ));
}
