//! Synthesized UI sounds.
//!
//! Rather than bundling WAV/MP3 files we generate short tones in memory
//! on demand. This keeps the binary ~300 KB smaller, avoids any audio-
//! licensing ambiguity, and gives us exact control over every sound so
//! they all share the same sonic "family" (pitch, envelope, length).
//!
//! Design rules for the sound palette:
//!
//! * Click feedback is very short (30 ms) and quiet, so it disappears
//!   behind the user's own perception rather than distracting them.
//! * Connect / Disconnect are louder, ~200 ms, with a major-third
//!   interval going up (for connect) or down (for disconnect) so the
//!   direction of the state change is audible even without a screen.
//! * Error is a lower frequency with a slight warble, ~350 ms, so it
//!   reads as "something went wrong" without being alarming.
//!
//! Every tone is shaped by an attack/release envelope to avoid the
//! "pop" you get from cutting a sine wave abruptly at zero.

use rodio::buffer::SamplesBuffer;
use rodio::{OutputStream, OutputStreamHandle, Sink};
use std::sync::{Arc, Mutex};

/// Discrete UI sound events. Adding a new one means adding both a
/// variant and a case in `synthesize()`.
#[derive(Debug, Clone, Copy)]
pub enum Sfx {
    /// Soft click — buttons, nav items.
    Click,
    /// Rising two-note chirp — fired when a Tor connection completes.
    Connect,
    /// Descending two-note chirp — fired when disconnecting.
    Disconnect,
    /// Low warbling tone — failures.
    Error,
    /// Single short beep — minor notifications (copy IP, toggle flip).
    Tick,
}

/// Lazily-constructed audio output. We wrap in Mutex<Option<...>> so the
/// first use triggers device initialisation; subsequent uses reuse the
/// same stream. If audio initialisation fails (no output device, driver
/// absent) we cache the failure so we don't retry on every click.
pub struct AudioEngine {
    inner: Arc<Mutex<AudioInner>>,
    enabled: Arc<std::sync::atomic::AtomicBool>,
}

enum AudioInner {
    Uninit,
    Ready {
        // We have to keep `_stream` alive even though we never read it —
        // when it drops, the audio output closes.
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

    /// Toggle sounds on/off at runtime. Cheap — a single atomic store.
    pub fn set_enabled(&self, enabled: bool) {
        self.enabled
            .store(enabled, std::sync::atomic::Ordering::Relaxed);
    }

    pub fn is_enabled(&self) -> bool {
        self.enabled.load(std::sync::atomic::Ordering::Relaxed)
    }

    /// Play a sound. Non-blocking — synthesis runs on the caller thread
    /// (< 1 ms for our tone lengths) and playback is queued on the sink
    /// which plays back on its own thread inside rodio.
    pub fn play(&self, sfx: Sfx) {
        if !self.is_enabled() {
            return;
        }
        // We do the lazy-init work behind the Mutex, but release the
        // mutex before calling `play_raw` so concurrent sounds don't
        // serialise on the lock.
        let handle = {
            let mut guard = match self.inner.lock() {
                Ok(g) => g,
                Err(_) => return, // poisoned lock — silently no-op
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
        // rodio's Sink plays queued sources on a background thread. For
        // one-shot UI sounds we don't even need to keep the sink around
        // after firing — detach it and let it drop when playback ends.
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

// ---- Synthesis ---------------------------------------------------------

const SAMPLE_RATE: u32 = 44_100;

/// Render the samples for a given sound event. Returns an f32 PCM buffer
/// at `SAMPLE_RATE` Hz, mono. Values in [-1.0, 1.0].
fn synthesize(sfx: Sfx) -> Vec<f32> {
    match sfx {
        Sfx::Click => tone(1800.0, 0.030, 0.18),
        Sfx::Tick => tone(2400.0, 0.050, 0.22),
        Sfx::Connect => {
            // Two ascending tones, A5 then C#6 — a pleasant major third.
            let mut buf = tone(880.0, 0.12, 0.28);
            buf.extend(tone(1108.73, 0.16, 0.28));
            buf
        }
        Sfx::Disconnect => {
            // Same two tones in reverse — descending feels like shutting down.
            let mut buf = tone(1108.73, 0.12, 0.26);
            buf.extend(tone(880.0, 0.16, 0.26));
            buf
        }
        Sfx::Error => {
            // Slightly dissonant low tone with vibrato for "something's off".
            warbled_tone(220.0, 0.35, 0.3, 6.0, 12.0)
        }
    }
}

/// A single sine tone with an attack/release envelope to avoid clicks.
/// `freq_hz` is the pitch, `duration_s` the length in seconds, `gain`
/// the peak amplitude (0..1).
fn tone(freq_hz: f32, duration_s: f32, gain: f32) -> Vec<f32> {
    let total = (SAMPLE_RATE as f32 * duration_s) as usize;
    // 5 ms attack, 20 ms release. Everything in between is at full gain.
    let attack = ((SAMPLE_RATE as f32) * 0.005) as usize;
    let release = ((SAMPLE_RATE as f32) * 0.020) as usize;
    let mut buf = Vec::with_capacity(total);
    for i in 0..total {
        let t = i as f32 / SAMPLE_RATE as f32;
        let envelope = if i < attack {
            // Linear fade-in.
            i as f32 / attack as f32
        } else if i > total.saturating_sub(release) {
            // Linear fade-out.
            (total - i) as f32 / release as f32
        } else {
            1.0
        };
        let sample = (2.0 * std::f32::consts::PI * freq_hz * t).sin() * envelope * gain;
        buf.push(sample);
    }
    buf
}

/// Sine tone with frequency-modulated vibrato. Used for the Error sound.
/// `vibrato_hz` is how many wobbles per second, `vibrato_depth` the pitch
/// swing in Hz.
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
    // We integrate the instantaneous frequency to get phase, rather than
    // feeding a shifting freq into sin() directly — that would produce
    // audible FM discontinuities. Phase-accumulator synthesis is the
    // correct way to do vibrato.
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
