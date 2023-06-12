pub mod spectrum;

use std::{
    collections::hash_map::DefaultHasher,
    hash::{Hash, Hasher},
};

use bevy::{audio, core_pipeline::bloom::BloomSettings, prelude::*};
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use ringbuf::{Consumer, Producer, RingBuffer};
use spectrum::{update_spectrum, AudioSpectrum};

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::BLACK))
        .add_plugins(DefaultPlugins)
        .init_resource::<AudioSpectrum>()
        .add_startup_system(setup_audio_stream)
        .add_startup_system(setup_scene)
        .add_system(revolve_spheres)
        .add_system(update_spectrum)
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

const NUM_SAMPLES: usize = 2048;

fn setup_audio_stream(world: &mut World) {
    let host = cpal::default_host();
    let input_device = host
        .default_input_device()
        .expect("failed to find input device");
    let mut supported_config_range = input_device
        .supported_input_configs()
        .expect("error while querying configs");
    let supported_config = supported_config_range
        .next()
        .expect("no supported config")
        .with_max_sample_rate();
    let config = supported_config.into();

    let ring_buffer = RingBuffer::<f32>::new(NUM_SAMPLES * 2);
    let (mut prod, mut cons) = ring_buffer.split();
    for _ in 0..NUM_SAMPLES {
        prod.push(0.0).unwrap();
    }

    let stream = input_device
        .build_input_stream(
            &config,
            move |data: &[f32], _: &cpal::InputCallbackInfo| {
                for d in data {
                    match prod.push(*d) {
                        Ok(()) => {}
                        Err(_) => {}
                    }
                }
            },
            move |_err| {},
            None,
        )
        .unwrap();

    stream.play().unwrap();
    world.insert_non_send_resource(stream);
    world.insert_non_send_resource(cons);
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
