//! Little synthesized chimes, so there are no audio files to ship.
//! If there's no audio device the game just stays quiet.

use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use std::sync::{Arc, Mutex};

#[derive(Clone, Copy)]
pub enum Cue {
    Right,
    Almost,
    Wrong,
    Tap,
    Hint,
    LevelDone,
}

struct Voice {
    freq: f32,
    /// Samples to wait before starting, so a cue can be a little tune.
    delay: usize,
    len: usize,
    t: usize,
    gain: f32,
}

pub struct Sound {
    voices: Arc<Mutex<Vec<Voice>>>,
    rate: f32,
    _stream: Option<cpal::Stream>,
}

fn open(voices: Arc<Mutex<Vec<Voice>>>) -> Option<(cpal::Stream, f32)> {
    let device = cpal::default_host().default_output_device()?;
    let config = device.default_output_config().ok()?;
    let rate = config.sample_rate() as f32;
    let channels = config.channels() as usize;
    let stream = device
        .build_output_stream(
            config.into(),
            move |out: &mut [f32], _| {
                out.fill(0.0);
                let Ok(mut vs) = voices.try_lock() else { return };
                for frame in out.chunks_mut(channels) {
                    let mut v = 0.0;
                    for voice in vs.iter_mut() {
                        if voice.delay > 0 {
                            voice.delay -= 1;
                            continue;
                        }
                        if voice.t < voice.len {
                            let x = voice.t as f32 / rate;
                            let p = voice.t as f32 / voice.len as f32;
                            let env = (p * 40.0).min(1.0) * (1.0 - p).powi(2);
                            let w = (x * voice.freq * std::f32::consts::TAU).sin() + 0.3 * (x * voice.freq * 2.0 * std::f32::consts::TAU).sin();
                            v += w * env * voice.gain;
                            voice.t += 1;
                        }
                    }
                    frame.fill(v.clamp(-0.9, 0.9));
                }
                vs.retain(|v| v.t < v.len);
            },
            |e| eprintln!("bilang audio: {e}"),
            None,
        )
        .ok()?;
    stream.play().ok()?;
    Some((stream, rate))
}

impl Sound {
    pub fn new() -> Self {
        let voices = Arc::new(Mutex::new(Vec::new()));
        let (stream, rate) = match open(voices.clone()) {
            Some((s, r)) => (Some(s), r),
            None => (None, 48_000.0),
        };
        Sound { voices, rate, _stream: stream }
    }

    pub fn play(&self, cue: Cue) {
        // (semitones from C5, start in seconds, length in seconds)
        let notes: &[(f32, f32, f32)] = match cue {
            Cue::Right => &[(4.0, 0.0, 0.25), (7.0, 0.08, 0.25), (12.0, 0.16, 0.45)],
            Cue::Almost => &[(7.0, 0.0, 0.2), (9.0, 0.1, 0.3)],
            Cue::Wrong => &[(-5.0, 0.0, 0.25), (-8.0, 0.14, 0.35)],
            Cue::Tap => &[(12.0, 0.0, 0.06)],
            Cue::Hint => &[(9.0, 0.0, 0.15), (14.0, 0.07, 0.3)],
            Cue::LevelDone => &[(0.0, 0.0, 0.3), (4.0, 0.12, 0.3), (7.0, 0.24, 0.3), (12.0, 0.36, 0.7), (16.0, 0.36, 0.7)],
        };
        let gain = if matches!(cue, Cue::Tap) { 0.08 } else { 0.18 };
        let Ok(mut vs) = self.voices.lock() else { return };
        for &(semi, start, len) in notes {
            vs.push(Voice {
                freq: 523.25 * 2f32.powf(semi / 12.0),
                delay: (start * self.rate) as usize,
                len: (len * self.rate) as usize,
                t: 0,
                gain,
            });
        }
    }
}
