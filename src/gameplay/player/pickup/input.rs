//! Forward the player's input to the pickup plugin.

use bevy::prelude::*;

use avian_pickup::prelude::*;
use bevy_enhanced_input::prelude::*;

use crate::gameplay::player::default_input::{DropProp, PickupProp};

pub(super) fn plugin(app: &mut App) {
    app.add_observer(pull_prop);
    app.add_observer(throw_prop);
    app.add_observer(drop_prop);
}

fn pull_prop(
    _on: On<Fire<PickupProp>>,
    actor: Single<Entity, With<AvianPickupActor>>,
    mut avian_pickup_input_writer: MessageWriter<AvianPickupInput>,
) {
    avian_pickup_input_writer.write(AvianPickupInput {
        action: AvianPickupAction::Pull,
        actor: *actor,
    });
}

fn throw_prop(
    _on: On<Start<PickupProp>>,
    actor: Single<Entity, With<AvianPickupActor>>,
    mut avian_pickup_input_writer: MessageWriter<AvianPickupInput>,
) {
    avian_pickup_input_writer.write(AvianPickupInput {
        action: AvianPickupAction::Throw,
        actor: *actor,
    });
}

fn drop_prop(
    _on: On<Start<DropProp>>,
    actor: Single<Entity, With<AvianPickupActor>>,
    mut avian_pickup_input_writer: MessageWriter<AvianPickupInput>,
) {
    avian_pickup_input_writer.write(AvianPickupInput {
        action: AvianPickupAction::Drop,
        actor: *actor,
    });
}
