// Disable console on Windows for non-dev builds.
#![cfg_attr(feature = "release", windows_subsystem = "windows")]

mod asset_processing;
mod asset_tracking;
mod audio;
#[cfg(feature = "dev")]
mod dev_tools;
mod fixed_update_inspection;
mod gameplay;
mod hdr;
mod menus;
mod props;
mod screens;
mod shader_compilation;
mod theme;
mod third_party;
mod ui_camera;

use asset_processing::default_image_sampler_descriptor;
use bevy::ecs::error::{GLOBAL_ERROR_HANDLER, error};
use bevy::pbr::DefaultOpaqueRendererMethod;
use bevy_landmass::LandmassSystemSet;
use bevy_mod_skinned_aabb::SkinnedAabbPlugin;
use bitflags::bitflags;

use bevy::core_pipeline::experimental::taa::TemporalAntiAliasPlugin;
use bevy::{asset::AssetMetaCheck, prelude::*, render::view::RenderLayers};

#[cfg(all(feature = "native", feature = "web"))]
compile_error!(
    "Exactly one of the `native` or the `web` feature must be active at the same time. Instead, both are currently enabled."
);
#[cfg(not(any(feature = "native", feature = "web")))]
compile_error!(
    "Exactly one of the `native` or the `web` feature must be active at the same time. Instead, both are currently disabled."
);
#[cfg(all(feature = "dev", feature = "release"))]
compile_error!(
    "Exactly one of the `dev` or the `release` feature must be active at the same time. Instead, both are currently enabled."
);
#[cfg(not(any(feature = "dev", feature = "release")))]
compile_error!(
    "Exactly one of the `dev` or the `release` feature must be active at the same time. Instead, both are currently disabled."
);

fn main() -> AppExit {
    // Don't panic on Bevy system errors, just log them.
    GLOBAL_ERROR_HANDLER
        .set(error)
        .expect("Error handler already set");

    let mut app = App::new();

    // Add Bevy plugins.
    app.insert_resource(DefaultOpaqueRendererMethod::deferred());
    app.add_plugins(
        DefaultPlugins
            .set(AssetPlugin {
                // Wasm builds will check for meta files (that don't exist) if this isn't set.
                // This causes errors and even panics on web build on itch.
                // See https://github.com/bevyengine/bevy_github_ci_template/issues/48.
                meta_check: AssetMetaCheck::Never,
                ..default()
            })
            .set(WindowPlugin {
                primary_window: Window {
                    title: "Foxtrot".to_string(),
                    fit_canvas_to_parent: true,
                    ..default()
                }
                .into(),
                ..default()
            })
            .set(ImagePlugin {
                default_sampler: default_image_sampler_descriptor(),
            }),
    );

    // Add next-gen audio backend
    #[cfg(feature = "native")]
    app.add_plugins(bevy_seedling::SeedlingPlugin::default());
    // right now, `Default` isn't implemented for any non-cpal backend
    #[cfg(feature = "web")]
    app.add_plugins(
        bevy_seedling::SeedlingPlugin::<firewheel_web_audio::WebAudioBackend> {
            config: Default::default(),
            stream_config: Default::default(),
            spawn_default_pool: true,
            pool_size: 4..=32,
        },
    );
    app.insert_resource(AmbientLight::NONE);
    app.add_plugins((TemporalAntiAliasPlugin, SkinnedAabbPlugin));

    // Order new `AppSet` variants by adding them here:
    app.configure_sets(
        Update,
        (
            PostPhysicsAppSystems::TickTimers,
            PostPhysicsAppSystems::ChangeUi,
            PostPhysicsAppSystems::PlaySounds,
            PostPhysicsAppSystems::PlayAnimations,
            PostPhysicsAppSystems::Update,
        )
            .chain(),
    );
    app.configure_sets(
        RunFixedMainLoop,
        (
            PrePhysicsAppSystems::UpdateNavmeshPositions,
            PrePhysicsAppSystems::UpdateNavmeshTargets,
            LandmassSystemSet::SyncExistence,
        )
            .chain()
            .in_set(RunFixedMainLoopSystem::BeforeFixedMainLoop),
    );
    // Set up the `Pause` state.
    app.init_state::<Pause>();
    app.configure_sets(Update, PausableSystems.run_if(in_state(Pause(false))));

    #[cfg(feature = "dev_native")]
    // Adding these here so that third party plugins can register their BRP methods.
    app.add_plugins((
        bevy::remote::RemotePlugin::default(),
        bevy::remote::http::RemoteHttpPlugin::default(),
    ));

    // Add third-party plugins.
    app.add_plugins(third_party::plugin);

    // Add other plugins.
    app.add_plugins((
        asset_processing::plugin,
        asset_tracking::plugin,
        #[cfg(feature = "dev")]
        dev_tools::plugin,
        screens::plugin,
        menus::plugin,
        props::plugin,
        theme::plugin,
        ui_camera::plugin,
        hdr::plugin,
        audio::plugin,
        fixed_update_inspection::plugin,
    ));

    // Add plugins that proload levels. These have to come later than the other plugins
    // because the objects they reference need to have been registered first.
    app.add_plugins((gameplay::plugin, shader_compilation::plugin));
    app.run()
}

/// High-level groupings of systems for the app in the [`RunFixedMainLoop`] schedule
/// and the [`RunFixedMainLoopSystem::BeforeFixedMainLoop`] system set.
/// When adding a new variant, make sure to order it in the `configure_sets`
/// call above.
#[derive(SystemSet, Debug, Clone, Copy, Eq, PartialEq, Hash, PartialOrd, Ord)]
enum PrePhysicsAppSystems {
    /// Update last valid positions on the navmesh
    UpdateNavmeshPositions,
    /// Update agent targets to the last valid navmesh position
    UpdateNavmeshTargets,
}

/// High-level groupings of systems for the app in the [`Update`] schedule.
/// When adding a new variant, make sure to order it in the `configure_sets`
/// call above.
#[derive(SystemSet, Debug, Clone, Copy, Eq, PartialEq, Hash, PartialOrd, Ord)]
enum PostPhysicsAppSystems {
    /// Tick timers.
    TickTimers,
    /// Change UI.
    ChangeUi,
    /// Play sounds.
    PlaySounds,
    /// Play animations.
    PlayAnimations,
    /// Do everything else (consider splitting this into further variants).
    Update,
}

/// This enum is converted to an `isize` to be used as a camera's order.
/// Since we have three camera, we use three enum variants.
/// This ordering here mean UI > ViewModel > World.
enum CameraOrder {
    World,
    ViewModel,
    Ui,
}

impl From<CameraOrder> for isize {
    fn from(order: CameraOrder) -> Self {
        order as isize
    }
}

bitflags! {
    struct RenderLayer: u32 {
        /// Used implicitly by all entities without a `RenderLayers` component.
        /// Our world model camera and all objects other than the player are on this layer.
        /// The light source belongs to both layers.
        const DEFAULT = 0b00000001;
        /// Used by the view model camera and the player's arm.
        /// The light source belongs to both layers.
        const VIEW_MODEL = 0b00000010;
        /// Since we use multiple cameras, we need to be explicit about
        /// which one is allowed to render particles.
        const PARTICLES = 0b00000100;
        /// 3D gizmos. These need to be rendered only by a 3D camera, otherwise the UI camera will render them in a buggy way.
        /// Specifically, the UI camera is a 2D camera, which by default is placed at a far away Z position,
        /// so it will effectively render a very zoomed out view of the scene in the center of the screen.
        const GIZMO3 = 0b0001000;
    }
}

impl From<RenderLayer> for RenderLayers {
    fn from(layer: RenderLayer) -> Self {
        // Render layers are just vectors of ints, so we convert each active bit to an int.
        RenderLayers::from_iter(layer.iter().map(|l| (l.bits() >> 1) as usize))
    }
}

/// Whether or not the game is paused.
#[derive(States, Copy, Clone, Eq, PartialEq, Hash, Debug, Default)]
#[states(scoped_entities)]
struct Pause(pub(crate) bool);

/// A system set for systems that shouldn't run while the game is paused.
#[derive(SystemSet, Copy, Clone, Eq, PartialEq, Hash, Debug)]
struct PausableSystems;
