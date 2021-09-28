mod systems;

use crate::GameState;
use bevy::prelude::*;

pub struct BevyPlugin;

impl Plugin for BevyPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system_set(
            SystemSet::on_enter(GameState::Game).with_system(systems::setup_player.system()),
        );
    }
}
