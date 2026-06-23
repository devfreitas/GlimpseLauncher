# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

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
