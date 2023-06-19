use bevy::prelude::*;

use self::{spectrum::*, stream::setup_audio_stream};

pub mod spectrum;
mod stream;

pub const NUM_SAMPLES: usize = 2048;

pub struct AudioProcessingPlugin;

impl Plugin for AudioProcessingPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<AudioSpectrum>()
            .add_startup_system(setup_audio_stream)
            .add_system(update_audio_spectrum);
    }
}
