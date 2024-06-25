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
