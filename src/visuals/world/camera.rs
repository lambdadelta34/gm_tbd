use super::super::utils::Settings;
use crate::Player;
use bevy::{prelude::*, render::camera::OrthographicProjection};

pub fn setup_camera(mut commands: Commands, settings: Res<Settings>) {
    let mut camera = OrthographicCameraBundle::new_2d();
    camera.orthographic_projection.scale = settings.camera_scale;
    commands.spawn_bundle(camera);
}

pub fn camera_movement(
    mut query: QuerySet<(
        Query<&Transform, With<Player>>,
        Query<&mut Transform, With<OrthographicProjection>>,
    )>,
) {
    let player_transform = query.q0().single().expect("Player is missing");
    let player_pos = *player_transform.translation;
    let mut camera_transform = query.q1_mut().single_mut().expect("Camera is missing");
    camera_transform.translation = Vec3::new(player_pos.x, player_pos.y, 1000.0);
}
