use crate::movement::{Movement, Velocity, WrapMovement};
use bevy::prelude::*;

#[derive(Component)]
pub struct Player;

pub fn add_player(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        SpriteBundle {
            transform: Transform::from_scale(Vec3::new(3., 3., 3.)),
            // Not yet decided on outline or not
            texture: asset_server.load("ship_outline.png"),
            ..Default::default()
        },
        Velocity {
            velocity: Vec3::ZERO,
            drag: 1.5,
        },
        Movement { speed: 3200. },
        WrapMovement,
        Player,
    ));
}

pub fn rotate_to_mouse(
    windows: Query<&Window>,
    cameras: Query<(&Camera, &GlobalTransform)>,
    mut players: Query<&mut Transform, With<Player>>,
) {
    let (camera, camera_transform) = cameras.single();
    let Some(target) = windows
        .single()
        .cursor_position()
        .and_then(|cursor_pos| camera.viewport_to_world_2d(camera_transform, cursor_pos))
    else {
        return;
    };

    let mut transform = players.single_mut();
    let pos = transform.translation;
    let dir = target - Vec2::new(pos.x, pos.y);

    transform.rotation = Quat::from_rotation_z(-dir.x.atan2(dir.y));
}
