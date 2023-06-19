pub mod audio_processing;
pub mod camera;
pub mod scenes;

use audio_processing::AudioProcessingPlugin;
use bevy::prelude::*;
use camera::CameraPlugin;
use scenes::orbital::OrbitalPlugin;
use scenes::ripple::RipplePlugin;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::BLACK))
        .add_plugins(DefaultPlugins)
        .add_plugin(CameraPlugin)
        .add_plugin(AudioProcessingPlugin)
        .add_plugin(OrbitalPlugin)
        .add_plugin(RipplePlugin)
        .run();
}
