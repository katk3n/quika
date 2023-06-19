pub mod audio_processing;
pub mod scenes;

use audio_processing::AudioProcessingPlugin;
use bevy::prelude::*;
use scenes::orbital::OrbitalPlugin;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::BLACK))
        .add_plugins(DefaultPlugins)
        .add_plugin(AudioProcessingPlugin)
        .add_plugin(OrbitalPlugin)
        .run();
}
