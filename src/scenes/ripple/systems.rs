use bevy::prelude::*;

use crate::{audio_processing::spectrum::AudioSpectrum, scenes::ripple::components::*};

const BASE_HEIGHT: f32 = -5.0;

pub fn setup_scene(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let emissive_color = materials.add(StandardMaterial {
        emissive: Color::hsl(220.0, 5.0, 0.5),
        ..default()
    });

    let mesh = meshes.add(
        shape::Icosphere {
            radius: 0.1,
            subdivisions: 5,
        }
        .try_into()
        .unwrap(),
    );

    for i in 0..128 {
        for j in 0..128 {
            let material = emissive_color.clone();

            commands.spawn((
                PbrBundle {
                    mesh: mesh.clone(),
                    material,
                    transform: Transform::from_xyz(
                        (i - 64) as f32 * 0.3,
                        BASE_HEIGHT,
                        (j - 64) as f32 * 0.3,
                    ),
                    ..default()
                },
                Bouncing {},
            ));
        }
    }
}

pub fn bounce_particles(
    time: Res<Time>,
    mut query: Query<(&mut Transform, &Bouncing)>,
    source_query: Query<&Source>,
) {
    for (mut transform, _bouncing) in query.iter_mut() {
        let mut height = 0.0;
        for source in source_query.iter() {
            let distance = transform.translation.distance(source.point);
            let theta = distance - 4.0 * (time.elapsed_seconds() - source.spawn_time);

            if 0.0 <= theta && theta < std::f32::consts::PI {
                height += source.magnitude * theta.sin();
            }
        }

        transform.translation.y = height + BASE_HEIGHT;
    }
}

pub fn spawn_ripple(mut commands: Commands, time: Res<Time>, audio_spectrum: Res<AudioSpectrum>) {
    let amp = audio_spectrum.max_amplitude;
    if amp > 0.1 {
        commands.spawn(Source {
            point: Vec3 {
                x: 0.0,
                y: BASE_HEIGHT,
                z: 0.0,
            },
            spawn_time: time.elapsed_seconds(),
            magnitude: amp,
        });
    }
}

pub fn despawn_ripples(mut commands: Commands, time: Res<Time>, query: Query<(Entity, &Source)>) {
    let now = time.elapsed_seconds();
    for (entity, source) in query.iter() {
        if now - source.spawn_time > 10.0 {
            commands.entity(entity).despawn();
        }
    }
}

pub fn switch_visibility(
    mut query: Query<&mut Visibility, With<Bouncing>>,
    keyboard_input: Res<Input<KeyCode>>,
) {
    if keyboard_input.just_pressed(KeyCode::Key2) {
        for mut visibility in query.iter_mut() {
            *visibility = match *visibility {
                Visibility::Inherited => Visibility::Hidden,
                Visibility::Hidden => Visibility::Inherited,
                _ => Visibility::Inherited,
            };
        }
    }
}
