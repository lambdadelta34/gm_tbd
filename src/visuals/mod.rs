mod debug;
mod player;
mod utils;
mod world;

use bevy::prelude::*;
use bevy_rapier2d::prelude::RapierConfiguration;

pub struct BevyPlugin;

impl Plugin for BevyPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.init_resource::<utils::Settings>()
            .add_startup_system(scaling.system())
            .add_plugin(player::BevyPlugin)
            .add_plugin(world::BevyPlugin);
    }
}

fn scaling(mut rapier_conf: ResMut<RapierConfiguration>, settings: Res<utils::Settings>) {
    // rapier_conf.gravity *= settings.unit_graphics_ratio;
    rapier_conf.scale = settings.unit_graphics_ratio;
}
