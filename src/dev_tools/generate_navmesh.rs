use std::fs::File;

use anyhow::Context as _;
use bevy::{input::common_conditions::input_just_pressed, prelude::*};
use bevy_rerecast::prelude::*;

use crate::gameplay::npc::{NPC_HEIGHT, NPC_RADIUS};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(
        Update,
        generate_navmesh.run_if(input_just_pressed(KeyCode::F4)),
    );
    app.add_observer(save_navmesh);
}

#[derive(Resource)]
#[expect(
    dead_code,
    reason = "We simply need to keep the navmesh alive *somewhere*"
)]
struct GlobalNavmesh(Handle<Navmesh>);

fn generate_navmesh(mut generator: NavmeshGenerator, mut commands: Commands) {
    let handle = generator.generate(NavmeshSettings::from_agent_3d(NPC_RADIUS, NPC_HEIGHT));
    commands.insert_resource(GlobalNavmesh(handle));
}

fn save_navmesh(trigger: Trigger<NavmeshReady>, navmeshes: Res<Assets<Navmesh>>) -> Result {
    let handle = trigger.event().0;
    let navmesh = navmeshes.get(handle).context("Navmesh not ready")?;
    let mut file = File::create("navmesh.nav")?;
    bincode::serde::encode_into_std_write(navmesh, &mut file, bincode::config::standard())?;
    info!("Wrote navmesh.nav");
    Ok(())
}
