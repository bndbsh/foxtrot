use avian_pickup::prop::PreferredPickupRotation;
use avian3d::prelude::*;
use bevy::prelude::*;
use bevy_landmass::{Character, prelude::*};

use bevy_trenchbroom::prelude::*;

use crate::{
    props::setup::setup_static_prop_with_convex_hull,
    third_party::{avian3d::CollisionLayer, bevy_trenchbroom::LoadTrenchbroomModel as _},
};

pub(super) fn plugin(app: &mut App) {
    app.add_observer(setup_crate_small);
    app.add_observer(setup_static_prop_with_convex_hull::<CrateBig>);
}

#[point_class(
    base(Transform, Visibility),
    model("models/darkmod/containers/crate01_big.gltf"),
    hooks(SpawnHooks::new().preload_model::<Self>())
)]
pub(crate) struct CrateBig;

#[point_class(
    base(Transform, Visibility),
    model("models/darkmod/containers/crate01_small.gltf"),
    hooks(SpawnHooks::new().preload_model::<Self>())
)]
pub(crate) struct CrateSmall;

fn setup_crate_small(
    add: On<Add, CrateSmall>,
    asset_server: Res<AssetServer>,
    mut commands: Commands,
    archipelago: Single<Entity, With<Archipelago3d>>,
) {
    let model = asset_server.load_trenchbroom_model::<CrateSmall>();
    commands.entity(add.entity).insert(Character3dBundle {
        character: Character::default(),
        settings: CharacterSettings { radius: 0.5 },
        archipelago_ref: ArchipelagoRef3d::new(*archipelago),
    });
    commands.entity(add.entity).insert((
        ColliderConstructorHierarchy::new(ColliderConstructor::ConvexHullFromMesh)
            .with_default_layers(CollisionLayers::new(CollisionLayer::Prop, LayerMask::ALL))
            .with_default_density(1_000.0),
        // Not inserting `TnuaNotPlatform`, otherwise the player will not be able to jump on it.
        SceneRoot(model),
        // The prop should be held upright.
        PreferredPickupRotation(Quat::IDENTITY),
        // Holding a big crate right in your face looks bad, so
        // let's move the pickup distance a bit further away.
        RigidBody::Dynamic,
    ));
}
