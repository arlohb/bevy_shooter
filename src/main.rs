mod movement;
mod player;
pub use player::*;
mod time_to_live;
pub use time_to_live::*;
mod shooting;

use bevy::prelude::*;
use movement::{Movement, Velocity, WrapMovement};

fn add_player(mut commands: Commands, asset_server: Res<AssetServer>) {
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

fn add_camera(mut commands: Commands) {
    commands.spawn((Camera2dBundle::default(),));
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugins(movement::plugin)
        .add_plugins(time_to_live::plugin)
        .add_plugins(shooting::plugin)
        .add_systems(Startup, add_player)
        .add_systems(Startup, add_camera)
        .run();
}
