mod movement;
mod player;
pub use player::*;
mod time_to_live;
pub use time_to_live::*;
mod shooting;

use bevy::prelude::*;

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
        .add_systems(Update, rotate_to_mouse)
        .run();
}
