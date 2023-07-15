use bevy::{core_pipeline::bloom::BloomSettings, prelude::*, window::PrimaryWindow};

const CAMERA_VEROCITY: f32 = 50.0;

#[derive(Component)]
pub struct OrbitCamera {
    pub focus: Vec3,
    pub radius: f32,
}

impl Default for OrbitCamera {
    fn default() -> Self {
        OrbitCamera {
            focus: Vec3::ZERO,
            radius: 5.0,
        }
    }
}

pub fn setup(mut commands: Commands) {
    let translation = Vec3::new(0.0, 0.0, 40.0);
    let radius = translation.length();

    commands.spawn((
        Camera3dBundle {
            camera: Camera {
                hdr: true,
                ..default()
            },
            transform: Transform::from_translation(translation).looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        },
        BloomSettings::default(),
        OrbitCamera {
            radius,
            ..Default::default()
        },
    ));
}

pub fn rotate(
    query_window: Query<&Window, With<PrimaryWindow>>,
    keyboard_input: Res<Input<KeyCode>>,
    time: Res<Time>,
    mut query: Query<(&mut OrbitCamera, &mut Transform)>,
) {
    let mut rotation_move = Vec2::ZERO;
    let mut key_pressed = false;

    if keyboard_input.pressed(KeyCode::Left) {
        rotation_move += Vec2::new(-1.0, 0.0);
        key_pressed = true;
    }
    if keyboard_input.pressed(KeyCode::Right) {
        rotation_move += Vec2::new(1.0, 0.0);
        key_pressed = true;
    }
    if keyboard_input.pressed(KeyCode::Up) {
        rotation_move += Vec2::new(0.0, 1.0);
        key_pressed = true;
    }
    if keyboard_input.pressed(KeyCode::Down) {
        rotation_move += Vec2::new(0.0, -1.0);
        key_pressed = true;
    }

    if !key_pressed {
        return;
    }

    rotation_move *= time.delta_seconds() * CAMERA_VEROCITY;

    let window = query_window.get_single().unwrap();

    for (orbit, mut transform) in query.iter_mut() {
        let delta_x = rotation_move.x / window.width() * std::f32::consts::PI * 2.0;
        let delta_y = rotation_move.y / window.height() * std::f32::consts::PI;
        let yaw = Quat::from_rotation_y(-delta_x);
        let pitch = Quat::from_rotation_x(-delta_y);
        transform.rotation = yaw * transform.rotation; // rotate around global y axis
        transform.rotation = transform.rotation * pitch; // rotate around local x axis

        let rot_matrix = Mat3::from_quat(transform.rotation);
        transform.translation =
            orbit.focus + rot_matrix.mul_vec3(Vec3::new(0.0, 0.0, orbit.radius));
    }
}
