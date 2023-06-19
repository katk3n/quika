use rand::Rng;

use bevy::{core_pipeline::bloom::BloomSettings, prelude::*};

use crate::audio_processing::spectrum::AudioSpectrum;

use super::components::*;

pub fn setup_scene(
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
        emissive: Color::hsl(70.0, 10.0, 0.5),
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

    let mut rng = rand::thread_rng();
    for i in 0..64 {
        let material = if rng.gen::<f32>() < 0.5 {
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
                threshold: rng.gen::<f32>() * 2.0 * std::f32::consts::PI,
            },
            Bouncing {
                threshold: rng.gen::<f32>() * 2.0 * std::f32::consts::PI,
                frequency_range: (i as f32 * 20.0, (i + 3) as f32 * 20.0),
            },
        ));
    }
}

pub fn revolve_spheres(time: Res<Time>, mut query: Query<(&mut Transform, &Revolving)>) {
    for (mut transform, revolving) in query.iter_mut() {
        transform.translation.x =
            revolving.radius * (revolving.threshold + time.elapsed_seconds() * 0.5).cos();
        transform.translation.z =
            revolving.radius * (revolving.threshold + time.elapsed_seconds() * 0.5).sin();
    }
}

pub fn bounce_spheres(
    time: Res<Time>,
    mut query: Query<(&mut Transform, &Bouncing)>,
    audio_spectrum: Res<AudioSpectrum>,
) {
    let amp = audio_spectrum.max_amplitude;
    let freq = audio_spectrum.max_frequency;
    for (mut transform, bouncing) in query.iter_mut() {
        let freq_range = bouncing.frequency_range;
        let power = if amp > 0.1 && freq_range.0 <= freq && freq < freq_range.1 {
            1.0 + audio_spectrum.max_amplitude * 2.0
        } else {
            1.0
        };
        transform.translation.y = power * (bouncing.threshold + time.elapsed_seconds()).sin();
    }
}
