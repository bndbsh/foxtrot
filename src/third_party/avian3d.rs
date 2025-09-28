//! [Avian](https://github.com/Jondolf/avian) is our physics engine.

use avian3d::prelude::*;
use bevy::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins(PhysicsPlugins::default());
    app.add_observer(enable_interpolation);
}

#[derive(Debug, PhysicsLayer, Default)]
pub(crate) enum CollisionLayer {
    #[default]
    Default,
    Prop,
    Character,
}

fn enable_interpolation(
    add: On<Add, RigidBody>,
    rigid_body: Query<&RigidBody>,
    mut commands: Commands,
) {
    let Ok(rigid_body) = rigid_body.get(add.entity) else {
        return;
    };
    if rigid_body.is_dynamic() {
        commands.entity(add.entity).insert((
            TransformInterpolation,
            SleepThreshold {
                // need to make the sleep threshold a bit more aggressive than the default,
                // otherwise some objects jitter around
                linear: 0.45,
                angular: 0.45,
            },
        ));
    }
}
