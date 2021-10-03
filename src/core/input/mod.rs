use bevy::prelude::{info, EventReader, EventWriter, Input, KeyCode, Res, ResMut};
use std::collections::HashMap;

#[derive(Hash, Eq, PartialEq, Clone, Copy, Debug)]
pub enum Action {
    MoveLeft,
    MoveRight,
    Jump,
}

#[derive(Eq, PartialEq, Debug)]
pub enum ActionState {
    Started,
    InProgress,
    Ended,
}

#[derive(Default, Debug)]
pub struct InputSystem {
    bindings: HashMap<Action, KeyCode>,
    state: HashMap<Action, ActionState>,
}

impl InputSystem {
    pub fn is_action_started(&self, action: Action) -> bool {
        self.is_action_triggered(&action) && self.state[&action] == ActionState::Started
    }

    pub fn is_action_in_progress(&self, action: Action) -> bool {
        self.is_action_triggered(&action) && self.state[&action] == ActionState::InProgress
    }

    pub fn is_action_ended(&self, action: Action) -> bool {
        self.is_action_triggered(&action) && self.state[&action] == ActionState::Ended
    }

    fn is_action_triggered(&self, action: &Action) -> bool {
        self.state.contains_key(&action)
    }

    pub fn register_binding(&mut self, action: Action, key: KeyCode) {
        self.bindings.insert(action, key);
    }

    fn set_action_state(&mut self, action: Action, state: ActionState) {
        self.state.insert(action, state);
    }

    fn reset_state(&mut self, action: Action) {
        self.state.remove(&action);
    }
}

pub fn track_input(mut input: ResMut<InputSystem>, key_input: Res<Input<KeyCode>>) {
    let bindings = input.bindings.clone();

    bindings.into_iter().for_each(|(action, key_code)| {
        if key_input.pressed(key_code) {
            input.set_action_state(action, ActionState::InProgress);
        }

        if key_input.just_pressed(key_code) {
            input.set_action_state(action, ActionState::Started);
        }

        if key_input.just_released(key_code) {
            input.set_action_state(action, ActionState::Ended);
        }

        if input.is_action_ended(action) && !key_input.just_released(key_code) {
            input.reset_state(action);
        }
    });
    info!("INPUTS {:?}", *input);
}
