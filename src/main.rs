pub mod audio_spectrum;
pub mod audio_stream;
pub mod scenes;

use audio_spectrum::{resources::*, systems::*};
use audio_stream::systems::*;
use bevy::prelude::*;
use scenes::orbital::systems::*;

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
