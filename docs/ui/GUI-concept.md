# Fluxara AVC – GUI Concept

## Overview

This document outlines the future graphical user interface for Fluxara AVC, designed with Linux-first principles using GTK4 and Libadwaita.

## Design Philosophy

### Linux Native

- **GTK4/Libadwaita**: Modern GNOME Human Interface Guidelines
- **Native Look**: Adapts to system theme (light/dark mode)
- **Responsive**: Works on various screen sizes
- **Accessible**: Full keyboard navigation, screen reader support

### Workflow-Oriented

The GUI follows a natural analog restoration workflow:

1. **Capture** → Digitize from physical media
2. **Enhance** → Apply restoration filters
3. **Convert** → Export to desired format
4. **Queue** → Batch processing

## Main Window Layout

```
┌─────────────────────────────────────────────────┐
│ ☰  Fluxara AVC                          ⚙ ☰     │ Header Bar
├─────────────────────────────────────────────────┤
│                                                 │
│  ┌─────────────────────────────────────────┐   │
│  │  🎥  Capture      🎬  VHS Rescue       │   │ Main Actions
│  │  🎵  Enhance Audio  📹  Enhance Video   │   │
│  │  🔄  Convert      📋  Queue             │   │
│  └─────────────────────────────────────────┘   │
│                                                 │
│  ┌─────────────────────────────────────────┐   │
│  │ Recent Files                            │   │
│  │ • tape01.mkv  (2.1 GB)  15 min ago     │   │
│  │ • audio-cassette.wav  (890 MB)  1h ago │   │ File List
│  │ • vhs-family.avi  (4.5 GB)  2h ago     │   │
│  └─────────────────────────────────────────┘   │
│                                                 │
│  [ No active jobs ]                             │ Status Bar
└─────────────────────────────────────────────────┘
```

## Key Views

### 1. Capture View

For live capture from V4L2/ALSA devices:

```
┌─────────────────────────────────────────────────┐
│ ← Back                  Capture           Start  │
├─────────────────────────────────────────────────┤
│                                                 │
│  Video Device: /dev/video0                     │
│  ┌───────────────────┐                         │
│  │                   │  [Live Preview]         │
│  │   📹             │                         │
│  │                   │  Resolution: 720×480    │
│  └───────────────────┘  Format: NTSC          │
│                                                 │
│  Audio Device: hw:1,0                          │
│  ▬▬▬▬▬▬▬▬▬▬▬▬▬  [Level meters]                │
│                                                 │
│  Output:                                        │
│  📁 ~/Videos/capture-2025-10-10.mkv           │
│  Format: FFV1 (lossless)                       │
│                                                 │
└─────────────────────────────────────────────────┘
```

### 2. VHS Rescue View

Specialized preset for VHS tapes:

```
┌─────────────────────────────────────────────────┐
│ ← Back              VHS Rescue            Apply  │
├─────────────────────────────────────────────────┤
│                                                 │
│  Source: tape01.mkv                            │
│                                                 │
│  Restoration Preset: ▼ VHS Standard Quality    │
│                                                 │
│  ☑ Deinterlace (yadif)                         │
│  ☑ Remove VHS noise (hqdn3d)                   │
│  ☑ Stabilize frame (deshake)                   │
│  ☑ Audio noise reduction                       │
│  ☐ Advanced color correction                   │
│                                                 │
│  Output Format: ▼ MP4 (H.264)                  │
│  Quality: ▰▰▰▰▰▱▱▱▱▱ High                      │
│                                                 │
│  Preview:                                       │
│  ┌───────────────────────────────────────┐     │
│  │ Before          │  After              │     │
│  │                 │                      │     │
│  └───────────────────────────────────────┘     │
│                                                 │
└─────────────────────────────────────────────────┘
```

### 3. Queue View

Batch processing management:

```
┌─────────────────────────────────────────────────┐
│ ☰  Queue                         ⏸ Clear All   │
├─────────────────────────────────────────────────┤
│                                                 │
│  Active (2 of 5 complete)                      │
│  ┌─────────────────────────────────────────┐   │
│  │ ✓ tape01.mkv → tape01.mp4              │   │
│  │ ⚙ tape02.mkv → tape02.mp4  [▰▰▰▱▱] 67% │   │
│  │ ⏸ tape03.mkv → tape03.mp4              │   │
│  │   audio01.wav → audio01.mp3            │   │
│  │   audio02.wav → audio02.mp3            │   │
│  └─────────────────────────────────────────┘   │
│                                                 │
│  Overall: 2 complete, 1 processing, 2 queued   │
│  Estimated time: 15 minutes remaining          │
│                                                 │
└─────────────────────────────────────────────────┘
```

### 4. Settings View

Configuration and preferences:

```
┌─────────────────────────────────────────────────┐
│ ← Back                 Settings                 │
├─────────────────────────────────────────────────┤
│                                                 │
│  General                                        │
│  Default output folder: ~/Videos               │
│  Parallel jobs: [4] threads                    │
│                                                 │
│  Capture                                        │
│  Default video device: /dev/video0             │
│  Default audio device: hw:1,0                  │
│  Capture format: ▼ FFV1 (lossless)             │
│                                                 │
│  Appearance                                     │
│  Theme: ◉ Follow system  ○ Light  ○ Dark       │
│                                                 │
│  Advanced                                       │
│  ☑ Show FFmpeg command line                    │
│  ☑ Keep source metadata                        │
│  ☐ Auto-start queue processing                 │
│                                                 │
└─────────────────────────────────────────────────┘
```

## Component Specifications

### Technology Stack

- **Framework**: GTK4
- **UI Library**: Libadwaita
- **Language**: Rust
- **Video Preview**: GStreamer
- **Backend**: FFmpeg (via CLI or direct API)

### Key Components

1. **MainWindow** (AdwApplicationWindow)
2. **CaptureView** (AdwPreferencesPage with camera controls)
3. **VHSRescueView** (AdwPreferencesPage with filter options)
4. **QueueView** (ListBox with progress indicators)
5. **SettingsView** (AdwPreferencesWindow)

### Widgets

- **AdwHeaderBar**: Application header with title and actions
- **AdwBanner**: For notifications and warnings
- **AdwToastOverlay**: For transient messages
- **AdwPreferencesGroup**: Grouped settings
- **AdwActionRow**: List items with actions
- **GtkListBox**: File and queue lists
- **GtkProgressBar**: Operation progress
- **GtkDropTarget**: Drag-and-drop support

## User Interactions

### Drag and Drop

- Drop video/audio files anywhere to add to queue
- Drop files on action buttons (Convert, Enhance, etc.) for direct processing

### Keyboard Shortcuts

- `Ctrl+O`: Open file
- `Ctrl+Q`: Quit
- `Ctrl+,`: Settings
- `Space`: Play/pause preview
- `Ctrl+Shift+C`: Open capture view
- `F1`: Help

### Right-Click Menus

Files in queue:
- Move up/down
- Remove from queue
- Show in file manager
- Copy FFmpeg command

## Progress Feedback

### Visual Indicators

- Spinning progress wheel for active operations
- Progress bars with percentage
- Color-coded status: Blue (processing), Green (complete), Red (error)
- Toast notifications for completion

### Detailed Information

- Click on queue item for detailed progress
- Shows current processing stage
- Estimated time remaining
- FFmpeg output (optional, in expander)

## Error Handling

### User-Friendly Messages

- "FFmpeg not found. Install with: sudo apt install ffmpeg"
- "Cannot read file. Check file permissions."
- "Unsupported format. See formats list."

### Recovery Actions

- Retry button for failed operations
- Skip and continue for batch operations
- Cancel all for critical errors

## Future Enhancements

- **Timeline Editor**: Visual trim and cut interface
- **Filters Gallery**: Preview all available filters
- **Batch Presets**: Save and load workflow presets
- **Cloud Integration**: Optional sync with cloud storage
- **Plugin System**: Extend with custom filters

## Implementation Roadmap

### Phase 1: Core GUI
- Main window with basic navigation
- File selection and format conversion
- Simple progress indicators

### Phase 2: VHS Rescue
- VHS preset with common filters
- Before/after preview
- Deinterlacing and noise reduction

### Phase 3: Live Capture
- V4L2/ALSA device enumeration
- Live preview
- Recording controls

### Phase 4: Advanced Features
- Timeline editor
- Custom filter chains
- Advanced presets

## Development Guidelines

### Code Organization

```
src-gui/
├── main.rs           # Application entry
├── window.rs         # Main window
├── views/
│   ├── capture.rs    # Capture view
│   ├── rescue.rs     # VHS Rescue view
│   ├── queue.rs      # Queue management
│   └── settings.rs   # Settings view
├── widgets/
│   ├── preview.rs    # Video preview widget
│   └── meter.rs      # Audio level meter
└── backend/
    ├── ffmpeg.rs     # FFmpeg integration
    └── devices.rs    # V4L2/ALSA detection
```

### Testing

- Unit tests for backend logic
- Integration tests for FFmpeg calls
- Manual UI testing with various file types
- Accessibility testing with Orca screen reader

### Documentation

- User guide with screenshots
- Developer guide for contributors
- API documentation for backend modules

## Design Mockups

> **Note**: Full mockups and design files will be added in future updates. Current wireframes show the general layout and interaction patterns.

## References

- [GNOME Human Interface Guidelines](https://developer.gnome.org/hig/)
- [Libadwaita Documentation](https://gnome.pages.gitlab.gnome.org/libadwaita/)
- [GTK4 Documentation](https://docs.gtk.org/gtk4/)

---

**Status**: Concept / Planning  
**Implementation**: Future milestone  
**Last Updated**: 2025-10  
**Maintainer**: Fluxara Team
