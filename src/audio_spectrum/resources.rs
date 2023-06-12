use bevy::prelude::*;

#[derive(Resource)]
pub struct AudioSpectrum {
    pub frequencies: Vec<(f32, f32)>,
    pub max_frequency: f32,
    pub max_amplitude: f32,
}

impl Default for AudioSpectrum {
    fn default() -> Self {
        Self {
            frequencies: vec![],
            max_frequency: 0.0,
            max_amplitude: 0.0,
        }
    }
}
