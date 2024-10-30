use bevy::{input::ButtonInput, math::Vec3, prelude::*, render::camera::Camera};

/// A system that enables camera movement based on keyboard input.
///
/// The system listens for keyboard inputs (W, A, S, D) to move the camera along the X and Y axes.
/// The movement is scaled by the time delta to ensure frame-rate independent movement.
///
/// ## Parameters
///
/// - `time`
///    A Bevy resource that provides the elapsed time between frames. Used to scale the movement.
///
/// - `keyboard_input`
///    Captures and processes input from the keyboard.
///
/// - `query`:
///    A Bevy query that retrieves the `Transform` and `OrthographicProjection` components of the
///    camera entity.
///
/// ## Behavior
///
/// - `W`: Moves the camera upward.
/// - `S`: Moves the camera downward.
/// - `A`: Moves the camera left.
/// - `D`: Moves the camera right.
///
/// The movement speed is scaled by `500`` units per second.
///
#[allow(dead_code)]
pub fn movement(
    time: Res<Time>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&mut Transform, &mut OrthographicProjection), With<Camera>>,
) {
    for (mut transform, mut _ortho) in query.iter_mut() {
        let mut direction = Vec3::ZERO;

        if keyboard_input.pressed(KeyCode::KeyA) {
            direction -= Vec3::new(1.0, 0.0, 0.0);
        }

        if keyboard_input.pressed(KeyCode::KeyD) {
            direction += Vec3::new(1.0, 0.0, 0.0);
        }

        if keyboard_input.pressed(KeyCode::KeyW) {
            direction += Vec3::new(0.0, 1.0, 0.0);
        }

        if keyboard_input.pressed(KeyCode::KeyS) {
            direction -= Vec3::new(0.0, 1.0, 0.0);
        }

        transform.translation += time.delta_seconds() * direction * 500.;
    }
}
