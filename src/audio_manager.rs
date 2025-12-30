use rodio::{Decoder, OutputStreamBuilder, Sink};
use std::fs::File;
use std::io::BufReader;
use std::sync::mpsc::{self, Sender};
use std::thread::{self, JoinHandle};
use std::time::{Duration, Instant};

use log::info;

/// The seek step in seconds for forward/backward seeking
const SEEK_STEP_SECS: u64 = 10;

/// Commands that can be sent to the audio background thread
enum AudioCommand {
    Load(String),
    Play,
    Pause,
    Stop,
    SeekTo(Duration),
    SeekForward,
    SeekBackward,
    Detach,
    Shutdown,
}

/// AudioManager runs audio playback on a background thread to avoid blocking the main thread.
/// Communication happens via channels - the main thread sends commands, the background thread executes them.
pub(crate) struct AudioManager {
    command_tx: Sender<AudioCommand>,
    _thread_handle: Option<JoinHandle<()>>,
}

impl AudioManager {
    /// Creates a new AudioManager with a background thread for audio playback.
    pub fn new() -> Self {
        let (command_tx, command_rx) = mpsc::channel::<AudioCommand>();

        let thread_handle = thread::spawn(move || {
            // Audio stream and sink live entirely on this background thread
            let stream = OutputStreamBuilder::open_default_stream().unwrap();
            let sink = Sink::connect_new(stream.mixer());

            // Position tracking state - all local to this thread
            let mut accumulated_ms: u64 = 0; // Time accumulated from previous play sessions
            let mut play_start: Option<Instant> = None; // When current play session started
            let mut current_file_path = String::new();

            // Helper closure to get current position
            let get_current_position_ms = |accumulated: u64, start: &Option<Instant>| -> u64 {
                match start {
                    Some(instant) => accumulated + instant.elapsed().as_millis() as u64,
                    None => accumulated,
                }
            };

            // Helper to reload and seek to a position (needed for backward seeking)
            let reload_and_seek = |sink: &Sink, path: &str, position: Duration| -> bool {
                sink.stop();
                sink.clear();
                if let Ok(file) = File::open(path) {
                    let reader = BufReader::new(file);
                    if let Ok(source) = Decoder::new(reader) {
                        sink.append(source);
                        if position.as_millis() > 0
                            && let Err(e) = sink.try_seek(position)
                        {
                            info!("Seek after reload failed: {:?}", e);
                            return false;
                        }

                        return true;
                    }
                }
                false
            };

            // Process commands from the main thread
            loop {
                match command_rx.recv() {
                    Ok(AudioCommand::Load(path)) => {
                        if let Ok(file) = File::open(&path) {
                            let reader = BufReader::new(file);
                            if let Ok(source) = Decoder::new(reader) {
                                sink.stop();
                                sink.clear();
                                sink.append(source);
                                current_file_path = path.clone();
                                accumulated_ms = 0;
                                play_start = None;
                                info!("Audio loaded: {}", path);
                            }
                        }
                    }
                    Ok(AudioCommand::Play) => {
                        sink.play();
                        play_start = Some(Instant::now());
                        info!("Audio playing");
                    }
                    Ok(AudioCommand::Pause) => {
                        // Accumulate elapsed time before pausing
                        if let Some(start) = play_start.take() {
                            accumulated_ms += start.elapsed().as_millis() as u64;
                        }
                        sink.pause();
                        info!("Audio paused at {}ms", accumulated_ms);
                    }
                    Ok(AudioCommand::Stop) => {
                        accumulated_ms = 0;
                        play_start = None;
                        sink.stop();
                        info!("Audio stopped");
                    }
                    Ok(AudioCommand::SeekTo(position)) => {
                        if let Err(e) = sink.try_seek(position) {
                            info!("Seek failed: {:?}", e);
                        } else {
                            accumulated_ms = position.as_millis() as u64;
                            // Reset play_start if currently playing
                            if play_start.is_some() {
                                play_start = Some(Instant::now());
                            }
                            info!("Seeked to {:?}", position);
                        }
                    }
                    Ok(AudioCommand::SeekForward) => {
                        let current_ms = get_current_position_ms(accumulated_ms, &play_start);
                        let new_pos_ms = current_ms.saturating_add(SEEK_STEP_SECS * 1000);
                        let new_pos = Duration::from_millis(new_pos_ms);
                        if let Err(e) = sink.try_seek(new_pos) {
                            info!("Seek forward failed: {:?}", e);
                        } else {
                            accumulated_ms = new_pos_ms;
                            if play_start.is_some() {
                                play_start = Some(Instant::now());
                            }
                            info!("Seeked forward to {:?}", new_pos);
                        }
                    }
                    Ok(AudioCommand::SeekBackward) => {
                        let current_ms = get_current_position_ms(accumulated_ms, &play_start);
                        let new_pos_ms = current_ms.saturating_sub(SEEK_STEP_SECS * 1000);
                        let new_pos = Duration::from_millis(new_pos_ms);

                        // Backward seeking requires reloading the file since most decoders
                        // don't support true backward seeking
                        let was_playing = play_start.is_some();
                        if reload_and_seek(&sink, &current_file_path, new_pos) {
                            accumulated_ms = new_pos_ms;
                            if was_playing {
                                sink.play();
                                play_start = Some(Instant::now());
                            } else {
                                sink.pause();
                                play_start = None;
                            }
                            info!("Seeked backward to {:?}", new_pos);
                        } else {
                            info!("Seek backward failed");
                        }
                    }
                    Ok(AudioCommand::Detach) => {
                        // Keep the thread alive to let audio play, but stop processing commands
                        sink.sleep_until_end();
                        info!("Audio detached and finished");
                        break;
                    }
                    Ok(AudioCommand::Shutdown) | Err(_) => {
                        info!("Audio thread shutting down");
                        break;
                    }
                }
            }
        });

        Self {
            command_tx,
            _thread_handle: Some(thread_handle),
        }
    }

    /// load a new song
    /// this method accepts String class file path
    pub(crate) fn load(&self, path: String) {
        let _ = self.command_tx.send(AudioCommand::Load(path));
    }

    /// Start or resume playback (non-blocking)
    pub(crate) fn play(&self) {
        let _ = self.command_tx.send(AudioCommand::Play);
    }

    /// Pause playback (non-blocking)
    pub(crate) fn pause(&self) {
        let _ = self.command_tx.send(AudioCommand::Pause);
    }

    /// Stop playback (non-blocking)
    pub(crate) fn stop(&self) {
        let _ = self.command_tx.send(AudioCommand::Stop);
    }

    /// Seek to a specific position (non-blocking)
    pub(crate) fn seek_to(&self, position: Duration) {
        let _ = self.command_tx.send(AudioCommand::SeekTo(position));
    }

    /// Seek forward by SEEK_STEP_SECS seconds (non-blocking)
    pub(crate) fn seek_forward(&self) {
        let _ = self.command_tx.send(AudioCommand::SeekForward);
    }

    /// Seek backward by SEEK_STEP_SECS seconds (non-blocking)
    pub(crate) fn seek_backward(&self) {
        let _ = self.command_tx.send(AudioCommand::SeekBackward);
    }

    /// Detach the audio - it will continue playing until finished.
    /// The background thread will keep running until the audio completes.
    pub(crate) fn detach(self) {
        let _ = self.command_tx.send(AudioCommand::Detach);
        // Don't join the thread - let it run independently
        // The thread handle is dropped, but the thread continues
    }
}

#[derive(Debug, Clone)]
pub enum AudioPlayerError {
    OutputStreamError(String),
    FileError(String),
    DecodeError(String),
    SinkError(String),
}

impl std::fmt::Display for AudioPlayerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AudioPlayerError::OutputStreamError(e) => write!(f, "Output stream error: {}", e),
            AudioPlayerError::FileError(e) => write!(f, "File error: {}", e),
            AudioPlayerError::DecodeError(e) => write!(f, "Decode error: {}", e),
            AudioPlayerError::SinkError(e) => write!(f, "Sink error: {}", e),
        }
    }
}

impl std::error::Error for AudioPlayerError {}
