use bevy::{prelude::*, render::view::RenderLayers};
use bevy_rerecast::{
    debug::{DetailNavmeshGizmo, NavmeshGizmoConfig},
    prelude::*,
};

use crate::{RenderLayer, gameplay::level::LevelAssets};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(
        Update,
        add_navmesh_gizmo.run_if(resource_exists_and_changed::<LevelAssets>),
    );
}
fn add_navmesh_gizmo(
    level: Res<LevelAssets>,
    mut commands: Commands,
    gizmos: Query<Entity, With<DetailNavmeshGizmo>>,
    mut gizmo_config: ResMut<NavmeshGizmoConfig>,
) {
    for entity in gizmos {
        commands.entity(entity).despawn();
    }
    commands.spawn(DetailNavmeshGizmo::new(&level.navmesh));
    gizmo_config.detail_navmesh.enabled = false;
    gizmo_config.detail_navmesh.render_layers = RenderLayers::from(RenderLayer::GIZMO3);
}
