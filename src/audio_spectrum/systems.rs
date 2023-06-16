use bevy::prelude::*;
use ringbuf::Consumer;
use spectrum_analyzer::scaling::*;
use spectrum_analyzer::windows::hann_window;
use spectrum_analyzer::{samples_fft_to_spectrum, FrequencyLimit};

use super::resources::AudioSpectrum;

const NUM_SAMPLES: usize = 2048;
const MIN_FREQUENCY: f32 = 27.0;
const MAX_FREQUENCY: f32 = 2000.0;
const SAMPLING_RATE: u32 = 44100;

pub fn update_audio_spectrum(
    mut audio_spectrum: ResMut<AudioSpectrum>,
    mut cons: NonSendMut<Consumer<f32>>,
) {
    if cons.len() < NUM_SAMPLES {
        return;
    }

    let mut samples: [f32; NUM_SAMPLES] = [0.0; NUM_SAMPLES];
    for i in 0..NUM_SAMPLES {
        let sample = cons.pop().unwrap();
        samples[i] = sample;
    }

    let hann_window = hann_window(&samples);
    let spectrum = samples_fft_to_spectrum(
        &hann_window,
        SAMPLING_RATE,
        FrequencyLimit::Range(MIN_FREQUENCY, MAX_FREQUENCY),
        Some(&divide_by_N_sqrt),
    )
    .unwrap();

    let frequencies = spectrum
        .data()
        .iter()
        .map(|freq| (freq.0.val(), freq.1.val()))
        .collect();

    let (max_fr, max_amp) = spectrum.max();

    audio_spectrum.frequencies = frequencies;
    audio_spectrum.max_frequency = max_fr.val();
    audio_spectrum.max_amplitude = max_amp.val();

    //println!(
    //    "freq: {}, amp: {}",
    //    audio_spectrum.max_frequency, audio_spectrum.max_amplitude
    //);
}
