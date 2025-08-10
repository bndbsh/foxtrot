//! [Landmass](https://github.com/andriyDev/landmass) powers out agent navigation.
//! The underlying navmesh is generated using [Oxidized Navigation](https://github.com/TheGrimsey/oxidized_navigation).

use std::sync::Arc;

use crate::gameplay::npc::NPC_RADIUS;
use bevy::prelude::*;
use bevy_landmass::{HeightPolygon, PointSampleDistance3d, prelude::*};
use bevy_rerecast::rerecast::PolygonNavmesh;
#[cfg(feature = "hot_patch")]
use bevy_simple_subsecond_system::hot;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins(Landmass3dPlugin::default());
    app.add_systems(Startup, setup_archipelago);
    app.add_systems(Update, update_landmass_navmesh);
}

#[cfg_attr(feature = "hot_patch", hot)]
fn setup_archipelago(mut commands: Commands) {
    // This *should* be scoped to the `Screen::Gameplay` state, but doing so
    // seems to never regenerate the nav mesh when the level is loaded the second
    // time.
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

fn update_landmass_navmesh(
    mut events: EventReader<AssetEvent<bevy_rerecast::Navmesh>>,
    rerecast_navmeshes: Res<Assets<bevy_rerecast::Navmesh>>,
    mut landmass_navmeshes: ResMut<Assets<bevy_landmass::NavMesh3d>>,
    archipelago: Single<Entity, With<Archipelago3d>>,
    mut commands: Commands,
) -> Result {
    let archipelago = archipelago.into_inner();
    for event in events.read() {
        let AssetEvent::LoadedWithDependencies { id } = event else {
            continue;
        };
        let Some(rerecast_navmesh) = rerecast_navmeshes.get(*id) else {
            error!("Failed to get navmesh from ID");
            continue;
        };

        let landmass_navmesh = rerecast_to_landsmass(rerecast_navmesh);
        let landmass_navmesh = match landmass_navmesh.validate() {
            Ok(landmass_navmesh) => landmass_navmesh,
            Err(e) => {
                error!("Landmass navmesh failed validation: {e}");
                continue;
            }
        };
        let landmass_navmesh = bevy_landmass::NavMesh {
            nav_mesh: Arc::new(landmass_navmesh),
            type_index_to_node_type: default(),
        };
        let landmass_navmesh_handle = landmass_navmeshes.add(landmass_navmesh);
        commands
            .entity(archipelago)
            .insert(bevy_landmass::NavMeshHandle::<ThreeD>(
                landmass_navmesh_handle,
            ));
    }
    Ok(())
}

fn rerecast_to_landsmass(
    rerecast_navmesh: &bevy_rerecast::Navmesh,
) -> bevy_landmass::NavigationMesh3d {
    let orig = rerecast_navmesh.polygon.aabb.min;
    let cs = rerecast_navmesh.polygon.cell_size;
    let ch = rerecast_navmesh.polygon.cell_height;
    let to_local = Vec3::new(cs, ch, cs);
    let nvp = rerecast_navmesh.polygon.max_vertices_per_polygon as usize;

    NavigationMesh3d {
        vertices: rerecast_navmesh
            .polygon
            .vertices
            .iter()
            .map(|v| orig + v.as_vec3() * to_local)
            .collect(),
        polygons: (0..rerecast_navmesh.polygon.polygon_count()).fold(Vec::new(), |mut acc, i| {
            let poly = &rerecast_navmesh.polygon.polygons[i * nvp..];
            let verts = poly[..nvp]
                .iter()
                .filter(|i| **i != PolygonNavmesh::NO_INDEX)
                .map(|i| *i as usize)
                .collect::<Vec<_>>();
            acc.push(verts);
            acc
        }),
        polygon_type_indices: rerecast_navmesh
            .polygon
            .areas
            .iter()
            .map(|a| a.0 as usize)
            .collect(),
        height_mesh: HeightNavigationMesh3d {
            polygons: rerecast_navmesh
                .detail
                .meshes
                .iter()
                .map(|submesh| HeightPolygon {
                    base_vertex_index: submesh.base_vertex_index,
                    vertex_count: submesh.vertex_count,
                    base_triangle_index: submesh.base_triangle_index,
                    triangle_count: submesh.triangle_count,
                })
                .collect(),
            triangles: rerecast_navmesh.detail.triangles.clone(),
            vertices: rerecast_navmesh.detail.vertices.clone(),
        }
        .into(),
    }
}
