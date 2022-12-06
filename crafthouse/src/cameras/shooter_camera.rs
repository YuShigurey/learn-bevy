use bevy::prelude::*;
use bevy::input::mouse::MouseMotion;
use bevy::window::CursorGrabMode;


#[derive(Component)]
pub struct ShooterCamera {
    pub focus: Vec3,
    pub radius: f32,
    pub upside_down: bool,
}


impl Default for ShooterCamera {
    fn default() -> Self {
        ShooterCamera {
            focus: Vec3::ZERO,
            radius: 5.0,
            upside_down: false,
        }
    }
}

/// Spawn a camera like this
pub fn spawn_camera(mut commands: Commands) {
    let translation = Vec3::new(-2.0, 2.5, 5.0);
    let radius = translation.length();

    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_translation(translation)
                .looking_at(Vec3::ZERO, Vec3::Y),
            ..Default::default()
        },
        ShooterCamera {
            radius,
            ..Default::default()
        },
    ));
}

fn get_primary_window_size(windows: &Res<Windows>) -> Vec2 {
    let window = windows.get_primary().unwrap();
    let window = Vec2::new(window.width() as f32, window.height() as f32);
    window
}

/// Pan the camera with middle mouse click, zoom with scroll wheel, orbit with right mouse click.
pub fn shooter_camera(
    windows: Res<Windows>,
    mut ev_motion: EventReader<MouseMotion>,
    // input_mouse: Res<Input<MouseButton>>,
    mut query: Query<(&mut ShooterCamera, &mut Transform, &Projection)>,
) {

    let window = windows.get_primary().unwrap();
    if window.cursor_grab_mode() != CursorGrabMode::Locked {
        return
    }

    let mut rotation_move = Vec2::ZERO;

    for ev in ev_motion.iter() {
        rotation_move += ev.delta;
    }


    for (mut pan_orbit, mut transform, projection) in query.iter_mut() {
        // only check for upside down when orbiting started or ended this frame
        // if the camera is "upside" down, panning horizontally would be inverted, so invert the input to make it correct
        let up = transform.rotation * Vec3::Y;
        pan_orbit.upside_down = up.y <= 0.0;

        let mut any = false;
        if rotation_move.length_squared() > 0.0 {
            any = true;
            let window = get_primary_window_size(&windows);
            let delta_x = {
                let delta = rotation_move.x / window.x * std::f32::consts::PI * 2.0;
                if pan_orbit.upside_down { -delta } else { delta }
            };
            let delta_y = rotation_move.y / window.y * std::f32::consts::PI;
            let yaw = Quat::from_rotation_y(-delta_x);
            let pitch = Quat::from_rotation_x(-delta_y);
            transform.rotation = yaw * transform.rotation; // rotate around global y axis
            transform.rotation = transform.rotation * pitch; // rotate around local x axis
        }

        if any {
            // emulating parent/child to make the yaw/y-axis rotation behave like a turntable
            // parent = x and y rotation
            // child = z-offset
            let rot_matrix = Mat3::from_quat(transform.rotation);
            transform.translation = pan_orbit.focus + rot_matrix.mul_vec3(Vec3::new(0.0, 0.0, pan_orbit.radius));
        }
    }
}
