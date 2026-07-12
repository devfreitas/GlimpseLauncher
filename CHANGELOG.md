# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.8.0] - 2026-07-11

### Added
- Complete redesign of the Settings Panel with a categorized sidebar navigation (Features, Appearance, Position, Shortcuts).
- Interactive 3x3 visual grid for intuitive window positioning mapping to exact anchors.
- UI support for custom Global Hotkey configuration.
- Palette of customizable accent colors (Pastel Purple, Light Blue, Green, Pink, Orange) for both Dark and Light modes.

### Changed
- Improved launcher background with a frosted matte-black aesthetic (95% opacity).
- Refined the selected item highlight to the classic translucid style.
- Increased number of visible search results in the launcher from 3 to 4.
- Increased default fallback icon sizes for improved visibility.

## [0.7.0] - 2026-07-10

### Added
- App and UWP icon extraction with disk caching via bincode
- External shadow and simulated acrylic effect on main window
- Clearer typography hierarchy and pill-shaped badges

### Changed
- Refined launcher UI paddings, removing empty space at the bottom of the window
- Updated search session colors

### Fixed
- Inner shadow ghosting on bottom edges
- Fallback icons for EXE and DIR badges
- CI / Check & Lint configurations

## [0.6.0] - 2026-06-22

### Added
- Ultra-fast fuzzy search powered by nucleo-matcher
- Inline calculator for math expressions
- Web search with `g ` prefix (Google)
- Terminal command execution with `> ` prefix
- Light/Dark theme toggle
- System tray integration with quick settings
- Auto-start with Windows option
- Draggable window positioning
- UWP and Win32 app indexing
- File system watcher for real-time index updates
- Singleton instance via IPC
- Persistent index cache with bincode
- mimalloc custom allocator for minimal memory footprint

### Fixed
- Launcher positioning on multi-monitor setups
- Window size calculation for search results

## [0.1.0] - 2026-01-01

### Added
- Initial release with basic search functionality
