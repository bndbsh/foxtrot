use bevy::prelude::*;

use bevy_trenchbroom::prelude::*;

use crate::props::effects::disable_shadow_casting;

pub(super) fn plugin(app: &mut App) {
    app.add_observer(setup_light_window_brush_entity);
}

#[solid_class(base(Transform, Visibility))]
pub(crate) struct LightWindow;

fn setup_light_window_brush_entity(add: On<Add, LightWindow>, mut commands: Commands) {
    let entity = add.entity;
    commands
        .entity(entity)
        // Using `children!` here would run into https://github.com/Noxmore/bevy_trenchbroom/issues/95
        .with_child(SpotLight {
            color: Color::srgb_u8(239, 173, 144),
            intensity: 200_000.0,
            radius: 0.1,
            shadows_enabled: true,
            ..default()
        })
        .queue(disable_shadow_casting);
}
