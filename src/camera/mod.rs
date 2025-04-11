use bevy::{input::mouse::MouseMotion, prelude::*, render::view::RenderLayers};

pub const GIZMO_RENDER_LAYER: u8 = 31;

#[derive(Component)]
struct FreeCameraState {
    yaw: f32,
    pitch: f32,
}

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_camera);
        app.add_systems(Update, camera_control_system);
    }
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn((
        Camera3d::default(),
        Camera {
            order: 0,
            ..default()
        },
        Transform::from_xyz(0.0, 32.0, 32.0).looking_at(Vec3::ZERO, Vec3::Y),
        FreeCameraState {
            yaw: 0.0,
            pitch: 0.0,
        },
        RenderLayers::from_layers(&[0, GIZMO_RENDER_LAYER.into()]),
    ));
}

fn camera_control_system(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mouse_buttons: Res<ButtonInput<MouseButton>>,
    mut camera: Query<(&mut Transform, &mut FreeCameraState), With<Camera>>,
    mut mouse_motion_events: EventReader<MouseMotion>,
) {
    let (mut transform, mut camera_state) = camera.single_mut();
    let mut rotation_delta = Vec2::ZERO;
    if mouse_buttons.pressed(MouseButton::Right) {
        for event in mouse_motion_events.read() {
            rotation_delta += event.delta;
        }

        // Sensitivity scale
        let sensitivity = 0.005;
        camera_state.yaw -= rotation_delta.x * sensitivity;
        camera_state.pitch -= rotation_delta.y * sensitivity;

        // Clamp pitch to avoid flipping
        camera_state.pitch = camera_state.pitch.clamp(
            -std::f32::consts::FRAC_PI_2 + 0.01,
            std::f32::consts::FRAC_PI_2 - 0.01,
        );

        // Update rotation from yaw/pitch
        transform.rotation =
            Quat::from_euler(EulerRot::YXZ, camera_state.yaw, camera_state.pitch, 0.0);
    }

    // Movement
    let mut direction = Vec3::ZERO;

    if keyboard_input.pressed(KeyCode::KeyW) {
        direction += *transform.forward();
    }
    if keyboard_input.pressed(KeyCode::KeyS) {
        direction -= *transform.forward();
    }
    if keyboard_input.pressed(KeyCode::KeyA) {
        direction -= *transform.right();
    }
    if keyboard_input.pressed(KeyCode::KeyD) {
        direction += *transform.right();
    }

    // Optional: add up/down movement
    if keyboard_input.pressed(KeyCode::Space) {
        direction += Vec3::Y;
    }
    if keyboard_input.pressed(KeyCode::ShiftLeft) {
        direction -= Vec3::Y;
    }

    let speed = 0.5;
    if direction != Vec3::ZERO {
        direction = direction.normalize();
        transform.translation += direction * speed;
    }
}
