use super::input::{Action, InputSystem};
use bevy::prelude::{info, EventReader, Query, Res, Transform, Vec2};
use bevy_rapier2d::prelude::{
    RigidBodyForces, RigidBodyMassProps, RigidBodyPosition, RigidBodyVelocity,
};

#[derive(Debug, Clone, Copy)]
pub struct Player {
    pub width: f32,
    pub height: f32,
    pub position: Vec2,
}

impl Default for Player {
    fn default() -> Self {
        Self {
            width: 0.75,
            height: 1.85,
            position: Vec2::new(2.0, 5.0),
        }
    }
}

impl Player {
    pub fn move_left(&mut self) {}
}

pub fn player_movement(
    input: Res<InputSystem>,
    mut player: Query<(
        &mut Player,
        &mut RigidBodyPosition,
        &mut RigidBodyVelocity,
        &RigidBodyMassProps,
    )>,
) {
    player.iter_mut().into_iter().for_each(
        |(mut player, mut position, mut velocity, rigid_props)| {
            info!("props {:?}", rigid_props);
            if input.is_action_in_progress(Action::MoveLeft) {
                velocity.apply_impulse(rigid_props, Vec2::new(-1.5, 0.0).into());
                info!("Move left");
            }

            if input.is_action_in_progress(Action::MoveRight) {
                velocity.apply_impulse(rigid_props, Vec2::new(1.5, 0.0).into());
                info!("Move right");
            }

            // TODO: make realistic jumps
            // if input.is_action_started(Action::Jump) {
            //     velocity.apply_impulse(rigid_props, Vec2::new(0.0, 10.0).into());
            //     info!("Jump");
            // }
        },
    );
}
