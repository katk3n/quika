pub mod audio_spectrum;
pub mod audio_stream;

use std::{
    collections::hash_map::DefaultHasher,
    hash::{Hash, Hasher},
};

use audio_spectrum::{resources::*, systems::*};
use audio_stream::systems::*;
use bevy::{core_pipeline::bloom::BloomSettings, prelude::*};

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::BLACK))
        .add_plugins(DefaultPlugins)
        .init_resource::<AudioSpectrum>()
        .add_startup_system(setup_audio_stream)
        .add_startup_system(setup_scene)
        .add_system(revolve_spheres)
        .add_system(update_audio_spectrum)
        .run();
}

fn setup_scene(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn((
        Camera3dBundle {
            camera: Camera {
                hdr: true,
                ..default()
            },
            transform: Transform::from_xyz(-2.0, 10.0, 40.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        },
        BloomSettings::default(),
    ));

    let emissive_color1 = materials.add(StandardMaterial {
        emissive: Color::rgb_linear(10.0, 20.0, 10.0),
        ..default()
    });
    let emissive_color2 = materials.add(StandardMaterial {
        emissive: Color::hsl(180.0, 10.0, 0.5),
        ..default()
    });

    let mesh = meshes.add(
        shape::Icosphere {
            radius: 0.5,
            subdivisions: 5,
        }
        .try_into()
        .unwrap(),
    );

    for i in 0..64 {
        let mut hasher = DefaultHasher::new();
        i.hash(&mut hasher);
        let rand = hasher.finish();

        let material = if rand % 2 == 0 {
            emissive_color1.clone()
        } else {
            emissive_color2.clone()
        };

        commands.spawn((
            PbrBundle {
                mesh: mesh.clone(),
                material,
                transform: Transform::from_xyz(0.0, 0.0, 0.0),
                ..default()
            },
            Revolving {
                radius: (i + 10) as f32 * 0.3,
                threshold: (rand % 1000) as f32 / 100.0,
            },
        ));
    }
}

#[derive(Component)]
struct Revolving {
    radius: f32,
    threshold: f32,
}

fn revolve_spheres(
    time: Res<Time>,
    mut query: Query<(&mut Transform, &Revolving)>,
    audio_spectrum: Res<AudioSpectrum>,
) {
    let amp = if audio_spectrum.max_amplitude > 0.5 {
        audio_spectrum.max_amplitude * 100.0
    } else {
        0.0
    };
    for (mut transform, revolving) in query.iter_mut() {
        transform.translation.x =
            revolving.radius * (revolving.threshold + time.elapsed_seconds() * 0.5).cos();
        transform.translation.z =
            revolving.radius * (revolving.threshold + time.elapsed_seconds() * 0.5).sin();
        transform.translation.y =
            (amp + revolving.radius + revolving.threshold + time.elapsed_seconds()).sin();
    }
}
