pub mod input;
mod player;

use bevy::prelude::*;
use input::{track_input, Action, InputSystem};
use player::{player_movement, Player};
use prelude::GameState;

pub mod prelude {
    pub use super::input;
    pub use super::player::Player;

    pub type Unit = f32;

    #[derive(Debug, Clone, Copy)]
    pub struct Wall {
        pub length: f32,
        pub thickness: f32,
    }

    impl Default for Wall {
        fn default() -> Self {
            Self {
                length: 300.,
                thickness: 0.5,
            }
        }
    }

    #[derive(Debug, Clone, Eq, PartialEq, Hash)]
    pub enum GameState {
        Game,
        MainMenu,
        OptionsMenu,
    }
}

pub struct BevyPlugin;

impl Plugin for BevyPlugin {
    fn build(&self, app: &mut AppBuilder) {
        use input;
        app.insert_resource(InputSystem::default())
            .add_startup_system(bind_input.system())
            .add_system_set(
                SystemSet::on_update(GameState::Game)
                    .label("input")
                    .with_system(track_input.system()),
            )
            .add_system_set(
                SystemSet::on_update(GameState::Game)
                    .after("input")
                    .with_system(player_movement.system()),
            );
    }
}

fn bind_input(mut input: ResMut<InputSystem>) {
    input.register_binding(Action::MoveLeft, KeyCode::A);
    input.register_binding(Action::MoveRight, KeyCode::D);
    input.register_binding(Action::Jump, KeyCode::Space);
}
