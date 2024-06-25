use bevy::prelude::*;

#[derive(Component, Debug)]
struct Pos {
    x: f32,
    y: f32,
}

impl Pos {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }
}

#[derive(Resource)]
struct PrintPositionTimer(Timer);

fn print_position(time: Res<Time>, mut timer: ResMut<PrintPositionTimer>, query: Query<&Pos>) {
    if timer.0.tick(time.delta()).just_finished() {
        for pos in &query {
            println!("{pos:?}");
        }
    }
}

fn move_diagonal(mut query: Query<&mut Pos>) {
    for mut pos in &mut query {
        pos.x += 0.01;
        pos.y += 0.02;
    }
}

fn add_player(mut commands: Commands) {
    commands.spawn((Pos::new(0., 0.),));
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, add_player)
        .insert_resource(PrintPositionTimer(Timer::from_seconds(
            1.,
            TimerMode::Repeating,
        )))
        .add_systems(Update, (move_diagonal, print_position).chain())
        .run();
}
