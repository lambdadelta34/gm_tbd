mod systems;

use super::utils;
use crate::GameState;
use bevy::prelude::*;
use systems::setup::setup_player;

pub struct BevyPlugin;

impl Plugin for BevyPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system_set(SystemSet::on_enter(GameState::Game).with_system(setup_player.system()));
    }
}
