# Music Player GPUI

A simple music player application built with [GPUI](https://crates.io/crates/gpui), Zed's GPU-accelerated UI framework for Rust.

![Rust](https://img.shields.io/badge/Rust-2024_Edition-orange)
![GPUI](https://img.shields.io/badge/GPUI-0.2.2-blue)
![rodio](https://img.shields.io/badge/rodio-0.21.1-green)

## Features

- 🎵 **MP3 Playback** - Play MP3 audio files using the rodio audio library
- ▶️ **Play/Pause Control** - Toggle playback with a single click
- ⏩ **Seek Forward** - Skip forward 10 seconds
- ⏪ **Seek Backward** - Skip backward 10 seconds
- 📋 **Song List** - Automatically scans and displays MP3 files in the current directory
- 🎨 **Modern UI** - Clean, GPU-accelerated interface built with GPUI
- 🔄 **Click to Play** - Click any song in the list to immediately start playing it
- 🧵 **Non-blocking Audio** - Audio runs on a dedicated background thread for smooth UI

## Screenshots

<img width="805" height="832" alt="Screenshot 2025-12-31 at 10 28 43 AM" src="https://github.com/user-attachments/assets/bc20e052-e423-4226-b635-e010e6a3c91a" />


The application features a centred layout with:
- Application title at the top
- Song list in the middle (with header showing Song name, Song writer, Singer)
- Control panel at the bottom with Seek Backwards, Play/Pause, and Seek Forward buttons

## Prerequisites

- Rust (2024 Edition)
- macOS (primary development platform)
- MP3 files for playback

## Dependencies

| Crate | Version | Purpose |
|-------|---------|---------|
| [gpui](https://crates.io/crates/gpui) | 0.2.2 | GPU-accelerated UI framework |
| [rodio](https://crates.io/crates/rodio) | 0.21.1 | Audio playback |
| [log](https://crates.io/crates/log) | 0.4.29 | Logging facade |
| [env_logger](https://crates.io/crates/env_logger) | 0.11.8 | Logger implementation |

## Installation

1. Clone the repository:
   ```bash
   git clone https://github.com/yourusername/music-player-gpui.git
   cd music-player-gpui
   ```

2. Build the project:
   ```bash
   cargo build --release
   ```

3. Run the application:
   ```bash
   cargo run
   ```

## Usage

1. Place MP3 files in the same directory as the executable (or the directory where you run the app)
2. Launch the application
3. The song list will automatically populate with detected MP3 files
4. Click on any song in the list to start playback
5. Use the play/pause button to toggle playback
6. Use the left/right arrow buttons to seek backward/forward by 10 seconds

### Keyboard Shortcuts

| Shortcut | Action |
|----------|--------|
| `Cmd+Q` | Quit the application |
| `Ctrl+C` | Quit the application |

## Project Structure

```
music-player-gpui/
├── Cargo.toml              # Project dependencies and metadata
├── README.md               # This file
├── assets/                 # UI assets
│   ├── left-button.png     # Seek backward button icon
│   ├── pause-button.png    # Pause button icon
│   ├── play-button.png     # Play button icon
│   └── right-button.png    # Seek forward button icon
└── src/
    ├── main.rs             # Application entry point and main UI
    ├── audio_manager.rs    # Background audio thread management
    ├── music_list_view.rs  # Song list view component
    └── play_element.rs     # Play/pause button component
```

## Architecture

### Main Application (`main.rs`)
The main module sets up the GPUI application window (800x800 pixels) and composes the UI with:
- A title header
- A `ListView` component for displaying songs (using `uniform_list` for efficient rendering)
- A control panel with seek backward, play/pause, and seek forward buttons
- Global communication channel for syncing player state between components

### Audio Manager (`audio_manager.rs`)
Handles audio playback on a dedicated background thread to prevent UI blocking. Features:
- **Command-based Architecture**: Uses channels (`mpsc`) to send commands (Load, Play, Pause, Stop, SeekTo, SeekForward, SeekBackward, Detach, Shutdown) to the audio thread
- **Position Tracking**: Maintains accurate playback position for seeking
- **Backward Seek Support**: Reloads and seeks when seeking backward (required by most audio decoders)
- **Non-blocking Operations**: All public methods return immediately

Supported commands:
- `Load` - Load a new audio file
- `Play` / `Pause` / `Stop` - Playback control
- `SeekTo` - Seek to specific position
- `SeekForward` / `SeekBackward` - Relative seeking (10 second steps)
- `Detach` - Keep playing until end, then stop
- `Shutdown` - Clean thread shutdown

### Play Element (`play_element.rs`)
A GPUI component that renders the play/pause button and manages the audio state. Features:
- Toggles between play and pause icons based on the current playback state
- Receives player state updates via channel communication
- Provides seek forward/backward functionality (10 second steps)

### Music List View (`music_list_view.rs`)
Scans the current directory for MP3 files and displays them in a virtualized list using GPUI's `uniform_list` for efficient rendering of large song collections. Features:
- Click any song to load and play it immediately
- Syncs playback state with the play button via channel communication
- Shows a "No songs found" message when the directory is empty

## Development

### Enable Logging
```bash
RUST_LOG=info cargo run
```

### Build Documentation
```bash
cargo doc --open
```

## Known Limitations

- Currently only supports MP3 format
- Song metadata (artist, album) is not yet parsed from files
- Scans only the current working directory for MP3 files

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

This project is open source. Please add an appropriate license file.

## Acknowledgments

- [GPUI](https://github.com/zed-industries/zed) - The GPU-accelerated UI framework from Zed
- [Rodio](https://github.com/RustAudio/rodio) - Rust audio playback library
