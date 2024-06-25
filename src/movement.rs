use bevy::prelude::*;

#[derive(Component, Default)]
pub struct Velocity {
    pub velocity: Vec2,
    pub drag: f32,
}

fn apply_velocity(time: Res<Time>, mut query: Query<(&mut Transform, &Velocity)>) {
    for (
        mut transform,
        &Velocity {
            velocity: Vec2 { x: vx, y: vy },
            ..
        },
    ) in &mut query
    {
        transform.translation += Vec3::new(vx, vy, 0.) * time.delta_seconds();
    }
}

fn reduce_velocity(time: Res<Time>, mut query: Query<&mut Velocity>) {
    for mut velocity in &mut query {
        let drag = velocity.drag;
        let v = velocity.velocity;

        velocity.velocity -= v * drag * time.delta_seconds();
    }
}

#[derive(Component)]
pub struct Movement {
    pub speed: f32,
}

fn movement(
    time: Res<Time>,
    keys: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&mut Velocity, &Movement)>,
) {
    let mut offset = Vec2::ZERO;

    if keys.pressed(KeyCode::KeyW) {
        offset.y += 1.;
    }

    if keys.pressed(KeyCode::KeyS) {
        offset.y -= 1.;
    }

    if keys.pressed(KeyCode::KeyD) {
        offset.x += 1.;
    }

    if keys.pressed(KeyCode::KeyA) {
        offset.x -= 1.;
    }

    if offset != Vec2::ZERO {
        for (mut velocity, movement) in &mut query {
            velocity.velocity += offset.normalize() * movement.speed * time.delta_seconds();
        }
    }
}

#[derive(Component)]
pub struct WrapMovement;

fn wrap_movement(
    windows: Query<&Window>,
    projections: Query<&OrthographicProjection>,
    mut transforms: Query<&mut Transform, With<WrapMovement>>,
) {
    let Ok(window) = windows.get_single() else {
        return;
    };
    let Ok(projection) = projections.get_single() else {
        return;
    };

    // TODO: Isn't using projection.scaling_mode
    let width = window.width() * projection.scale;
    let height = window.height() * projection.scale;
    let max_x = width / 2.;
    let min_x = -max_x;
    let max_y = height / 2.;
    let min_y = -max_y;

    for mut transform in &mut transforms {
        let translation = &mut transform.translation;

        if translation.x < min_x {
            translation.x = max_x;
        }

        if translation.x > max_x {
            translation.x = min_x;
        }

        if translation.y < min_y {
            translation.y = max_y;
        }

        if translation.y > max_y {
            translation.y = min_y;
        }
    }
}

pub fn plugin(app: &mut App) {
    app.add_systems(
        Update,
        (movement, apply_velocity, (reduce_velocity, wrap_movement)).chain(),
    );
}
