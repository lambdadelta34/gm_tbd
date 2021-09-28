use bevy::{
    diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin},
    prelude::*,
};
use bevy_rapier3d::prelude::*;

pub struct BevyPlugin;

impl Plugin for BevyPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_plugin(LogDiagnosticsPlugin::default())
            .add_plugin(FrameTimeDiagnosticsPlugin)
            .add_system(display_events.system());
    }
}

fn display_events(
    mut intersection_events: EventReader<bevy_rapier3d::prelude::IntersectionEvent>,
    mut contact_events: EventReader<bevy_rapier3d::prelude::ContactEvent>,
) {
    for intersection_event in intersection_events.iter() {
        println!("Received intersection event: {:?}", intersection_event);
    }

    for contact_event in contact_events.iter() {
        println!("Received contact event: {:?}", contact_event);
    }
}
