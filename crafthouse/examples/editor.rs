//! Shows how to create a 3D orthographic view (for isometric-look games or CAD applications).

use std::f32::consts::PI;
use bevy::prelude::*;
use bevy::time::{FixedTimestep, FixedTimesteps};
use bevy::window::CursorGrabMode;
use crafthouse::cameras::blender_camera::{spawn_camera, pan_orbit_camera};

const TIME_STEP: f32 = 1.0 / 60.0;

#[derive(Clone, Eq, PartialEq, Debug, Hash)]
enum GameState {
    Playing,
    // GameOver,
}


fn main() {
    App::new()
        .init_resource::<Game>()
        .add_plugins(DefaultPlugins)
        .add_state(GameState::Playing)
        .add_startup_system(spawn_camera)
        .add_system_set(SystemSet::on_enter(GameState::Playing).with_system(setup))
        .add_system_set(
            SystemSet::on_update(GameState::Playing)
                .with_run_criteria(FixedTimestep::step(TIME_STEP as f64))
                // .with_system(move_player)
                .with_system(pan_orbit_camera.after(move_player))
        )
        // .add_system(cursor_grab_system)
        // .add_system_set(
        //     SystemSet::new()
        //         .with_run_criteria(FixedTimestep::step(TIME_STEP as f64))
        //         .with_system(move_view_angle)
        // )
        .run();
}


fn cursor_grab_system(
    mut windows: ResMut<Windows>,
    btn: Res<Input<MouseButton>>,
    key: Res<Input<KeyCode>>,
) {
    let window = windows.get_primary_mut().unwrap();

    if btn.just_pressed(MouseButton::Left) {
        // if you want to use the cursor, but not let it leave the window,
        // use `Confined` mode:
        // window.set_cursor_grab_mode(CursorGrabMode::Confined);

        // for a game that doesn't use the cursor (like a shooter):
        // use `Locked` mode to keep the cursor in one place
        window.set_cursor_grab_mode(CursorGrabMode::Locked);
        // also hide the cursor
        window.set_cursor_visibility(false);
    }

    if key.just_pressed(KeyCode::Escape) {
        window.set_cursor_grab_mode(CursorGrabMode::None);
        window.set_cursor_visibility(true);
    }
}


#[derive(Resource, Default)]
struct Game {
    player: Player,
    // camera_should_focus: Vec2,
    camera_is_focus: Vec2,
}

#[derive(Default)]
struct Player {
    entity: Option<Entity>,
    x: f32,
    y: f32,
    z: f32,
}

// const RESET_FOCUS: [f32; 2] = [
//     PI,
//     0.
// ];

/// set up a simple 3D scene
fn setup(
    mut commands: Commands,
    mut game: ResMut<Game>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {

    game.player.entity = Some(
        commands
            .spawn(SceneBundle {
                transform: Transform {
                    translation: Vec3::new(
                        game.player.x,
                        game.player.y,
                        game.player.z,
                    ),
                    rotation: Quat::from_rotation_y(-PI/2.0),
                    ..default()
                },
                ..default()
            }
            ).id()
    );

    // plane
    commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Plane { size: 5.0 })),
        material: materials.add(Color::rgba(0.3, 0.5, 0.3, 0.5).into()),
        ..default()
    });

    commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Plane { size: 5.0 })),
        material: materials.add(Color::rgb(0.8, 0.5, 0.8).into()),
        transform: Transform::from_xyz(0., 3., 0.).with_rotation(Quat::from_rotation_x(PI)),
        ..default()
    });

    commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Plane { size: 5.0 })),
        material: materials.add(Color::rgb(0.8, 0.9, 0.8).into()),
        transform: Transform::from_xyz(-2., 0.5, 0.).with_rotation(Quat::from_rotation_z(-PI/2.)),
        ..default()
    });

    commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Plane { size: 5.0 })),
        material: materials.add(Color::rgb(0.8, 0.9, 0.8).into()),
        transform: Transform::from_xyz(2., 0.5, 0.).with_rotation(Quat::from_rotation_z(PI/2.)),
        ..default()
    });

    commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Plane { size: 5.0 })),
        material: materials.add(Color::rgb(0.8, 0.9, 0.8).into()),
        transform: Transform::from_xyz(0., 0.5, -2.).with_rotation(Quat::from_rotation_x(PI/2.)),
        ..default()
    });

    commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Plane { size: 5.0 })),
        material: materials.add(Color::rgb(0.8, 0.9, 0.8).into()),
        transform: Transform::from_xyz(0., 0.5, 2.).with_rotation(Quat::from_rotation_x(-PI/2.)),
        ..default()
    });


    // cubes
    commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
        material: materials.add(Color::rgba(0.8, 0.7, 0.6, 0.1).into()),
        transform: Transform::from_xyz(1.5, 0.5, 1.5),
        ..default()
    });
    commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
        material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
        transform: Transform::from_xyz(1.5, 0.5, -1.5),
        ..default()
    });
    commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
        material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
        transform: Transform::from_xyz(-1.5, 0.5, 1.5),
        ..default()
    });
    commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
        material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
        transform: Transform::from_xyz(-1.5, 0.5, -1.5),
        ..default()
    });
    // light
    commands.spawn(PointLightBundle {
        transform: Transform::from_xyz(3.0, 8.0, 5.0),
        ..default()
    });
}


fn move_player(
    // mut commands: Commands,
    windows: ResMut<Windows>,
    keyboard_input: Res<Input<KeyCode>>,
    mut game: ResMut<Game>,
    // mut transforms: Query<&mut Transform>,
    mut view_transforms: ParamSet<(Query<&mut Transform, With<Camera3d>>, Query<&Transform>)>,
) {
    const SPEED: f32 = 0.1;
    let normalized = |x: f32, y: f32| {let sum = (1. + y.tan() * y.tan()).sqrt(); (x.cos()/sum as f32, y.tan()/sum as f32, x.sin()/sum as f32)};

    let window = windows.get_primary().unwrap();
    let mut delta = (0., 0., 0.);
    if window.cursor_grab_mode() != CursorGrabMode::Locked {
        return
    }

    if keyboard_input.pressed(KeyCode::W) {
        delta = normalized(game.camera_is_focus.x, game.camera_is_focus.y);
    } else if keyboard_input.pressed(KeyCode::S) {
        delta = normalized(game.camera_is_focus.x, game.camera_is_focus.y);
        (delta.0, delta.1, delta.2) = (-delta.0, -delta.1, -delta.2)

    } else if keyboard_input.pressed(KeyCode::D) {
        delta = (-game.camera_is_focus.x.sin(), 0., game.camera_is_focus.x.cos())
    } else if keyboard_input.pressed(KeyCode::A) {
        delta = (game.camera_is_focus.x.sin(), 0., -game.camera_is_focus.x.cos())
    } else if keyboard_input.pressed(KeyCode::Space) {
        delta = (0., 1., 0.)

    } else if keyboard_input.pressed(KeyCode::LShift) {
        delta = (0., -1., 0.)
    }

    let (x, y, z) = (game.player.x, game.player.y, game.player.z) ;
    let (dx, dy, dz) = delta;

    game.player.x += dx * SPEED;
    game.player.y += dy * SPEED;
    game.player.z += dz * SPEED;

    for mut transform in view_transforms.p0().iter_mut() {
        *transform = transform.with_translation(Vec3::from_array([game.player.x,game.player.y,game.player.z]));
    }
}
