use rodio::buffer::SamplesBuffer;
use rodio::{OutputStream, OutputStreamHandle, Sink};
use std::sync::{Arc, Mutex};

#[derive(Debug, Clone, Copy)]
pub enum Sfx {
    Click,
    Connect,
    Disconnect,
    Error,
    Tick,
}

pub struct AudioEngine {
    inner: Arc<Mutex<AudioInner>>,
    enabled: Arc<std::sync::atomic::AtomicBool>,
}

enum AudioInner {
    Uninit,
    Ready {
        _stream: OutputStream,
        handle: OutputStreamHandle,
    },
    Failed,
}

impl AudioEngine {
    pub fn new() -> Self {
        Self {
            inner: Arc::new(Mutex::new(AudioInner::Uninit)),
            enabled: Arc::new(std::sync::atomic::AtomicBool::new(true)),
        }
    }

    pub fn set_enabled(&self, enabled: bool) {
        self.enabled
            .store(enabled, std::sync::atomic::Ordering::Relaxed);
    }

    pub fn is_enabled(&self) -> bool {
        self.enabled.load(std::sync::atomic::Ordering::Relaxed)
    }

    pub fn play(&self, sfx: Sfx) {
        if !self.is_enabled() {
            return;
        }
        let handle = {
            let mut guard = match self.inner.lock() {
                Ok(g) => g,
                Err(_) => return, 
            };
            if matches!(*guard, AudioInner::Uninit) {
                *guard = match OutputStream::try_default() {
                    Ok((stream, handle)) => AudioInner::Ready {
                        _stream: stream,
                        handle,
                    },
                    Err(e) => {
                        log::warn!("Audio output unavailable: {e}");
                        AudioInner::Failed
                    }
                };
            }
            match &*guard {
                AudioInner::Ready { handle, .. } => handle.clone(),
                _ => return,
            }
        };

        let samples = synthesize(sfx);
        if let Ok(sink) = Sink::try_new(&handle) {
            sink.append(SamplesBuffer::new(1, SAMPLE_RATE, samples));
            sink.detach();
        }
    }
}

impl Default for AudioEngine {
    fn default() -> Self {
        Self::new()
    }
}


const SAMPLE_RATE: u32 = 44_100;

fn synthesize(sfx: Sfx) -> Vec<f32> {
    match sfx {
        Sfx::Click => tone(1800.0, 0.030, 0.18),
        Sfx::Tick => tone(2400.0, 0.050, 0.22),
        Sfx::Connect => {
            let mut buf = tone(880.0, 0.12, 0.28);
            buf.extend(tone(1108.73, 0.16, 0.28));
            buf
        }
        Sfx::Disconnect => {
            let mut buf = tone(1108.73, 0.12, 0.26);
            buf.extend(tone(880.0, 0.16, 0.26));
            buf
        }
        Sfx::Error => {
            warbled_tone(220.0, 0.35, 0.3, 6.0, 12.0)
        }
    }
}

fn tone(freq_hz: f32, duration_s: f32, gain: f32) -> Vec<f32> {
    let total = (SAMPLE_RATE as f32 * duration_s) as usize;
    let attack = ((SAMPLE_RATE as f32) * 0.005) as usize;
    let release = ((SAMPLE_RATE as f32) * 0.020) as usize;
    let mut buf = Vec::with_capacity(total);
    for i in 0..total {
        let t = i as f32 / SAMPLE_RATE as f32;
        let envelope = if i < attack {
            i as f32 / attack as f32
        } else if i > total.saturating_sub(release) {
            (total - i) as f32 / release as f32
        } else {
            1.0
        };
        let sample = (2.0 * std::f32::consts::PI * freq_hz * t).sin() * envelope * gain;
        buf.push(sample);
    }
    buf
}

fn warbled_tone(
    base_freq: f32,
    duration_s: f32,
    gain: f32,
    vibrato_hz: f32,
    vibrato_depth: f32,
) -> Vec<f32> {
    let total = (SAMPLE_RATE as f32 * duration_s) as usize;
    let attack = ((SAMPLE_RATE as f32) * 0.010) as usize;
    let release = ((SAMPLE_RATE as f32) * 0.060) as usize;
    let mut buf = Vec::with_capacity(total);
    let mut phase = 0.0f32;
    let dt = 1.0 / SAMPLE_RATE as f32;
    for i in 0..total {
        let t = i as f32 / SAMPLE_RATE as f32;
        let envelope = if i < attack {
            i as f32 / attack as f32
        } else if i > total.saturating_sub(release) {
            (total - i) as f32 / release as f32
        } else {
            1.0
        };
        let inst_freq = base_freq + vibrato_depth * (2.0 * std::f32::consts::PI * vibrato_hz * t).sin();
        phase += 2.0 * std::f32::consts::PI * inst_freq * dt;
        buf.push(phase.sin() * envelope * gain);
    }
    buf
}
