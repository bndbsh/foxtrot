use super::setup::*;
use bevy::prelude::*;
use bevy_trenchbroom::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_observer(setup_static_prop_with_convex_hull::<Grate>)
        .add_observer(setup_static_prop_with_convex_decomposition::<Table>)
        .add_observer(setup_static_prop_with_convex_hull::<Bookshelf>)
        .add_observer(setup_static_prop_with_convex_hull::<Generator2>)
        .add_observer(setup_static_prop_with_convex_hull::<BarrelLargeClosed>)
        .add_observer(setup_static_prop_with_convex_hull::<Barrel01>)
        .add_observer(setup_static_prop_with_convex_hull::<CrateSquare>)
        .add_observer(setup_static_prop_with_convex_hull::<FenceBarsDecorativeSingle>)
        .add_observer(setup_static_prop_with_convex_hull::<DoorStainedGlass>);

    app.add_observer(setup_dynamic_prop_with_convex_hull::<PackageMedium>)
        .add_observer(setup_dynamic_prop_with_convex_hull::<PackageSmall>);

    app.add_observer(setup_nonphysical_prop::<IvyPart8>)
        .add_observer(setup_nonphysical_prop::<SmallDoorSign1>);
}

// generic dynamic props

#[point_class(
    base(Transform, Visibility),
    model("models/darkmod/containers/package_medium.gltf"),
    hooks(SpawnHooks::new().preload_model::<Self>())
)]
pub(crate) struct PackageMedium;

#[point_class(
    base(Transform, Visibility),
    model("models/darkmod/containers/package_small.gltf"),
    hooks(SpawnHooks::new().preload_model::<Self>())
)]
pub(crate) struct PackageSmall;

// generic static props
#[point_class(
    base(Transform, Visibility),
    model("models/darkmod/fireplace/grate.gltf"),
    hooks(SpawnHooks::new().preload_model::<Self>())
)]
pub(crate) struct Grate;

#[point_class(
    base(Transform, Visibility),
    model("models/darkmod/furniture/tables/rtable1.gltf"),
    hooks(SpawnHooks::new().preload_model::<Self>())
)]
pub(crate) struct Table;

#[point_class(
    base(Transform, Visibility),
    model("models/darkmod/furniture/shelves/bookshelf02.gltf"),
    hooks(SpawnHooks::new().preload_model::<Self>())
)]
pub(crate) struct Bookshelf;

#[point_class(
    base(Transform, Visibility),
    model("models/darkmod/mechanical/generator2/generator2.gltf"),
    hooks(SpawnHooks::new().preload_model::<Self>())
)]
pub(crate) struct Generator2;

#[point_class(
    base(Transform, Visibility),
    model("models/darkmod/containers/barrel_large_closed.gltf"),
    hooks(SpawnHooks::new().preload_model::<Self>())
)]
pub(crate) struct BarrelLargeClosed;

#[point_class(
    base(Transform, Visibility),
    model("models/darkmod/containers/barrel01.gltf"),
    hooks(SpawnHooks::new().preload_model::<Self>())
)]
pub(crate) struct Barrel01;

#[point_class(
    base(Transform, Visibility),
    model("models/darkmod/containers/crate_square.gltf"),
    hooks(SpawnHooks::new().preload_model::<Self>())
)]
pub(crate) struct CrateSquare;

#[point_class(
    base(Transform, Visibility),
    model("models/darkmod/architecture/fencing/fence_bars_decorative01_single.gltf"),
    hooks(SpawnHooks::new().preload_model::<Self>())
)]
pub(crate) struct FenceBarsDecorativeSingle;

#[point_class(
    base(Transform, Visibility),
    model("models/darkmod/architecture/doors/door_stained_glass_118x52.gltf"),
    hooks(SpawnHooks::new().preload_model::<Self>())
)]
pub(crate) struct DoorStainedGlass;

// Generic non-physical props

#[point_class(
    base(Transform, Visibility),
    model("models/darkmod/nature/ivy_part08.gltf"),
    hooks(SpawnHooks::new().preload_model::<Self>())
)]
pub(crate) struct IvyPart8;

#[point_class(
    base(Transform, Visibility),
    model("models/darkmod/decorative/signs/small_door_sign1.gltf"),
    hooks(SpawnHooks::new().preload_model::<Self>())
)]
pub(crate) struct SmallDoorSign1;
