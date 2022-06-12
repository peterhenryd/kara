use bevy::prelude::*;
use bevy::render::camera::CameraPlugin as BevyCameraPlugin;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(BevyCameraPlugin)
            .add_startup_system(setup_camera)
            .add_system(camera_movement);
    }
}

fn setup_camera(mut commands: Commands) {
    commands.spawn_bundle(PerspectiveCameraBundle {
        transform: Transform::from_xyz(0.0, 0.0, 0.0).looking_at(Vec3::new(0.0, 1.0, 1.0), Vec3::Y),
        ..default()
    });
    commands.spawn_bundle(UiCameraBundle::default());
}

/*
fn camera_pan(
    windows: Res<Windows>,
    mut mouse_input: EventReader<MouseMotion>,
    mut previous: Query<&mut PreviousMouse>,
    mut camera: Query<&mut Transform, With<Camera>>
) {
    let window = windows.get_primary().unwrap();

    if !window.cursor_locked() {
        return;
    }

    if let Some(MouseMotion { delta }) = mouse_input.iter().next() {
        let mut cameras = camera.iter_mut();

        let mut previous_mouse = previous.get_single_mut().unwrap();

        let quat = Quat::from_xyzw(
            (delta.x - previous_mouse.0.x) / 32.0,
            (delta.x - previous_mouse.0.x) / 32.0,
            (delta.y - previous_mouse.0.y) / 32.0,
            (delta.y - previous_mouse.0.y) / 32.0,
        );

        previous_mouse.0.x = delta.x;
        previous_mouse.0.y = delta.y;

        if let Some(mut transform) = cameras.next() {
            transform.rotation = transform.rotation + quat;

            if let Some(mut transform) = cameras.next() {
                transform.rotation = transform.rotation + quat;
            }
        }
    }
}
 */

fn camera_movement(
    keyboard_input: Res<Input<KeyCode>>,
    mut camera: Query<&mut Transform, With<Camera>>
) {
    let mut coord_shift = Vec3::new(0.0, 0.0, 0.0);

    if keyboard_input.pressed(KeyCode::W) {
        coord_shift.y += 0.1;
    }

    if keyboard_input.pressed(KeyCode::A) {
        coord_shift.x += 0.1;
    }

    if keyboard_input.pressed(KeyCode::S) {
        coord_shift.y -= 0.1;
    }

    if keyboard_input.pressed(KeyCode::D) {
        coord_shift.x -= 0.1;
    }

    if keyboard_input.pressed(KeyCode::Space) {
        coord_shift.z -= 0.1;
    }

    if keyboard_input.pressed(KeyCode::LShift) {
        coord_shift.z += 0.1;
    }

    let mut cameras = camera.iter_mut();

    if let Some(mut transform) = cameras.next() {
        transform.translation += coord_shift;

        if let Some(mut transform) = cameras.next() {
            transform.translation += coord_shift;
        }
    }
}