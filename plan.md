# Better Control RS - Development Plan

A modern system control panel built with Rust, GTK4, and libadwaita.

## Project Overview

**Technology Stack:**
- **Language:** Rust
- **GUI Framework:** GTK4
- **UI Library:** libadwaita
- **Async Runtime:** tokio
- **D-Bus Communication:** zbus
- **Build System:** Meson + Cargo
- **Testing:** cargo test + GTK test framework

**Target Platforms:**
- Primary: Linux
- Secondary: Maybe freebsd

## Project File Structure

```
better-control-rs/
├── Cargo.toml                          # Main project configuration
├── build.rs                            # Build script for resource compilation
├── plan.md                             # Development plan (this file)
├── README.md                           # Project documentation
├── LICENSE                             # Project license (GPL v3)
├── .gitignore                          # Git ignore rules
├── src/
│   ├── main.rs                         # Application entry point
│   ├── app.rs                          # AdwApplication implementation
│   ├── window.rs                       # AdwApplicationWindow implementation
│   ├── config.rs                       # Configuration management
│   ├── actions.rs                      # GSimpleAction implementations
│   ├── shortcuts.rs                    # Keyboard shortcuts setup
│   ├── 
│   ├── pages/
│   │   ├── mod.rs                      # Pages module
│   │   ├── system_info.rs              # System information page
│   │   ├── power.rs                    # Power management page
│   │   ├── wifi.rs                     # WiFi management page
│   │   ├── bluetooth.rs                # Bluetooth management page
│   │   ├── displays.rs                 # Display management page
│   │   ├── audio.rs                    # Audio management page
│   │   └── page_trait.rs               # Common page interface
│   ├── 
│   ├── widgets/
│   │   ├── mod.rs                      # Widgets module
│   │   ├── preference_row.rs           # Custom preference rows
│   │   ├── network_row.rs              # WiFi network display row
│   │   ├── device_row.rs               # Bluetooth/Audio device row
│   │   ├── battery_widget.rs           # Battery status widget
│   │   ├── signal_strength.rs          # WiFi signal strength indicator
│   │   └── progress_circle.rs          # Circular progress indicator
│   ├── 
│   ├── dialogs/
│   │   ├── mod.rs                      # Dialogs module
│   │   ├── about.rs                    # AdwAboutDialog implementation
│   │   ├── preferences.rs              # AdwPreferencesDialog implementation
│   │   ├── wifi_connect.rs             # WiFi connection dialog
│   │   ├── bluetooth_pair.rs           # Bluetooth pairing dialog
│   │   └── confirmation.rs             # Generic confirmation dialog
│   ├── 
│   ├── services/
│   │   ├── mod.rs                      # Services module
│   │   ├── system_monitor.rs           # System information collection
│   │   ├── network_manager.rs          # NetworkManager D-Bus client
│   │   ├── bluetooth_manager.rs        # BlueZ D-Bus client
│   │   ├── power_manager.rs            # UPower D-Bus client
│   │   ├── audio_manager.rs            # PulseAudio/PipeWire client
│   │   ├── display_manager.rs          # Display configuration service
│   │   └── dbus_client.rs              # Common D-Bus utilities
│   ├── 
│   ├── utils/
│   │   ├── mod.rs                      # Utilities module
│   │   ├── formatters.rs               # Data formatting helpers
│   │   ├── validators.rs               # Input validation
│   │   ├── file_utils.rs               # File system operations
│   │   ├── async_utils.rs              # Async operation helpers
│   │   └── logging.rs                  # Logging configuration
│   └── 
│   └── constants.rs                    # Application constants
│
├── resources/
│   ├── ui/
│   │   ├── window.ui                   # Main window UI definition
│   │   ├── pages/
│   │   │   ├── system_info.ui          # System info page UI
│   │   │   ├── power.ui                # Power management UI
│   │   │   ├── wifi.ui                 # WiFi management UI
│   │   │   ├── bluetooth.ui            # Bluetooth management UI
│   │   │   ├── displays.ui             # Display management UI
│   │   │   └── audio.ui                # Audio management UI
│   │   ├── dialogs/
│   │   │   ├── preferences.ui          # Preferences dialog UI
│   │   │   ├── wifi_connect.ui         # WiFi connection dialog
│   │   │   └── bluetooth_pair.ui       # Bluetooth pairing dialog
│   │   └── widgets/
│   │       ├── network_row.ui          # WiFi network row template
│   │       ├── device_row.ui           # Device row template
│   │       └── battery_widget.ui       # Battery widget template
│   │
│   ├── icons/
│   │   ├── app-icon.svg                # Application icon
│   │   ├── symbolic/
│   │   │   ├── wifi-symbolic.svg       # WiFi icons
│   │   │   ├── bluetooth-symbolic.svg  # Bluetooth icons
│   │   │   ├── battery-symbolic.svg    # Battery icons
│   │   │   ├── display-symbolic.svg    # Display icons
│   │   │   └── audio-symbolic.svg      # Audio icons
│   │   └── scalable/
│   │       └── better-control.svg      # Main application icon
│   ├── 
│   ├── schemas/
│   │   └── rs.better.control.gschema.xml # GSettings schema
│   ├── 
│   └── desktop/
│       ├── better-control.desktop      # Desktop entry
│       └── better-control.metainfo.xml # AppStream metadata
│
├── po/                                 # Internationalization
│   ├── POTFILES.in                     # Translatable files list
│   ├── LINGUAS                         # Supported languages
│   ├── better-control.pot              # Translation template
│   ├── pt_BR.po                        # Portuguese (Brazil) translations
│   └── en.po                           # English translations
│
├── tests/
│   ├── integration/
│   │   ├── mod.rs                      # Integration tests module
│   │   ├── dbus_tests.rs               # D-Bus integration tests
│   │   ├── ui_tests.rs                 # UI interaction tests
│   │   └── system_tests.rs             # System integration tests
│   ├── unit/
│   │   ├── mod.rs                      # Unit tests module
│   │   ├── services_tests.rs           # Services unit tests
│   │   ├── utils_tests.rs              # Utilities unit tests
│   │   └── widgets_tests.rs            # Widget unit tests
│   └── fixtures/
│       ├── mock_dbus_responses.json    # Mock D-Bus responses
│       └── test_configs.toml           # Test configurations
│
├── docs/
│   ├── README.md                       # Detailed documentation
│   ├── CONTRIBUTING.md                 # Contribution guidelines
│   ├── ARCHITECTURE.md                 # Architecture documentation
│   ├── API.md                          # API documentation
│   ├── BUILDING.md                     # Build instructions
│   ├── TROUBLESHOOTING.md              # Common issues and solutions
│   └── screenshots/
│       ├── main-window.png             # Application screenshots
│       ├── wifi-page.png
│       ├── bluetooth-page.png
│       └── about-dialog.png
│
├── packaging/
│   ├── flatpak/
│   │   ├── rs.better.control.json      # Flatpak manifest
│   │   └── rs.better.control.appdata.xml
│   ├── rpm/
│   │   └── better-control.spec         # RPM spec file
│   ├── deb/
│   │   ├── control                     # Debian control file
│   │   ├── changelog                   # Debian changelog
│   │   └── rules                       # Debian rules
│   └── arch/
│       └── PKGBUILD                    # Arch Linux package
│
└── .github/
    ├── workflows/
    │   ├── ci.yml                      # Continuous integration
    │   ├── release.yml                 # Release automation
    │   └── docs.yml                    # Documentation generation
    ├── ISSUE_TEMPLATE/
    │   ├── bug_report.md               # Bug report template
    │   └── feature_request.md          # Feature request template
    └── PULL_REQUEST_TEMPLATE.md        # PR template
```

### File Structure Explanation

**Core Application (`src/`):**
- **`main.rs`** - Application entry point, sets up logging and launches app
- **`app.rs`** - `AdwApplication` implementation with action registration
- **`window.rs`** - Main window with header bar, view switcher, and toast overlay
- **`config.rs`** - Configuration management using GSettings
- **`actions.rs`** - All application actions (menu, shortcuts, etc.)

**UI Components:**
- **`pages/`** - Each system settings page as separate modules
- **`widgets/`** - Custom widgets for specialized functionality
- **`dialogs/`** - Modal dialogs (about, preferences, connection dialogs)

**System Integration (`services/`):**
- **D-Bus clients** for each system service (NetworkManager, BlueZ, UPower)
- **Async service managers** for real-time system monitoring
- **Common utilities** for D-Bus communication

**Resources (`resources/`):**
- **GTK Builder UI files** for complex layouts
- **Application icons** in multiple formats and sizes
- **GSettings schema** for preferences persistence
- **Desktop integration files** for system integration

**Development Support:**
- **Comprehensive test suite** (unit + integration tests)
- **Documentation** with architecture and API reference
- **Build and packaging scripts** for multiple distributions
- **CI/CD configuration** for automated testing and releases

## Development Timeline

### Week 1: Project Foundation & Basic Structure

**Goals:**
- Set up project structure and build system
- Create basic application framework
- Implement main window with AdwHeaderBar # Plans changed, we won't have AdwHeaderBar
- Set up AdwViewSwitcher navigation

**Technical Tasks:**
- [x] Initialize Cargo project with proper dependencies
- [x] Create `Cargo.toml` with GTK4, libadwaita, glib, gio dependencies
- [x] Set up `build.rs` for resource compilation
- [x] Create main application structure (`AdwApplication`)
- [x] Implement `AdwApplicationWindow` with basic layout
- [x] Add `AdwHeaderBar` with integrated `AdwViewSwitcher` # Plans changed, we won't have AdwHeaderBar because it looks bad on tilling window managers
- [x] Create `AdwViewStack` for page management
- [x] Implement basic page structure with placeholder content
- [x] Set up proper error handling and logging

**Deliverables:**
- Working application skeleton
- Navigation between empty pages
- Proper window management (minimize, maximize, close)

### Week 2: Menu System & Dialogs

**Goals:**
- Implement hamburger menu system    # changed idea, we don't need it.
- Create About dialog                # changed idea, we don't need it.
- Build preferences dialog           # changed idea, we don't need it.
- Add keyboard shortcuts

**Technical Tasks:**
- [ ] Add `GtkMenuButton` to header bar # changed idea, we don't need it.
- [ ] Create `GMenuModel` for menu structure # changed idea, we don't need it.
- [ ] Implement `GSimpleAction` system for menu items # changed idea, we don't need it.
- [ ] Build `AdwAboutDialog` with application metadata # changed idea, we don't need it.
- [ ] Create `AdwPreferencesDialog` with basic settings # changed idea, we don't need it.
- [ ] Add `AdwPreferencesPage` and `AdwPreferencesGroup` structure # changed idea, we don't need it.
- [x] Implement `GtkShortcutController` for keyboard shortcuts
- [ ] Add `AdwToastOverlay` for user feedback # changed idea, we don't need it.
- [ ] Create settings persistence with `GSettings` # changed idea, we don't need it.

**Deliverables:**
- Functional hamburger menu # changed idea, we don't need it.
- Complete About dialog # changed idea, we don't need it.
- Basic preferences dialog # changed idea, we don't need it.
- Working keyboard shortcuts

### Week 3: System Information Page

**Goals:**
- Create comprehensive system information display
- Implement real-time data updates
- Add system monitoring capabilities

**Technical Tasks:**
- [ ] Build `AdwPreferencesPage` for system info
- [ ] Create `AdwPreferencesGroup` for different info categories
- [ ] Implement system info collection via `/proc` and `/sys`
- [ ] Add CPU information display (`AdwActionRow` with details)
- [ ] Implement memory usage monitoring
- [ ] Add disk usage information with progress bars
- [ ] Create network interface status display
- [ ] Implement real-time updates using `glib::timeout_add_seconds`
- [ ] Add system uptime and load average
- [ ] Create kernel and distribution information display

**Deliverables:**
- Complete system information page
- Real-time system monitoring
- Responsive UI updates

### Week 4: Power Management Page

**Goals:**
- Implement battery status monitoring
- Add power profile management
- Create thermal monitoring

**Technical Tasks:**
- [ ] Integrate with UPower D-Bus service (`org.freedesktop.UPower`)
- [ ] Create battery status display with `AdwActionRow`
- [ ] Implement power profile switching (performance/balanced/power-saver)
- [ ] Add thermal zone monitoring via `/sys/class/thermal`
- [ ] Create power consumption graphs using `GtkDrawingArea`
- [ ] Implement AC adapter status detection
- [ ] Add suspend/hibernate controls
- [ ] Create power history tracking
- [ ] Implement low battery notifications using `AdwToast`

**Deliverables:**
- Functional power management interface
- Battery monitoring with visual feedback
- Power profile controls

### Week 5: WiFi Management Page

**Goals:**
- Implement comprehensive WiFi management
- Add network scanning and connection
- Create connection history

**Technical Tasks:**
- [ ] Integrate with NetworkManager D-Bus (`org.freedesktop.NetworkManager`)
- [ ] Create network scanning functionality
- [ ] Implement `AdwActionRow` for each WiFi network
- [ ] Add signal strength indicators using custom widgets
- [ ] Create connection dialog with password entry (`AdwPasswordEntryRow`)
- [ ] Implement connection management (connect/disconnect)
- [ ] Add network profile management
- [ ] Create connection history tracking
- [ ] Implement automatic connection preferences
- [ ] Add network troubleshooting tools

**Deliverables:**
- Complete WiFi management interface
- Network scanning and connection
- Connection history and profiles

### Week 6: Bluetooth Management Page

**Goals:**
- Implement Bluetooth device management
- Add pairing and connection controls
- Create device-specific settings

**Technical Tasks:**
- [ ] Integrate with BlueZ D-Bus service (`org.bluez`)
- [ ] Create Bluetooth adapter status display
- [ ] Implement device discovery and scanning
- [ ] Add device pairing workflow with `AdwMessageDialog`
- [ ] Create paired devices list with `AdwActionRow`
- [ ] Implement device connection/disconnection controls
- [ ] Add device-specific settings (audio quality, input methods)
- [ ] Create device removal and management
- [ ] Implement Bluetooth file transfer integration
- [ ] Add troubleshooting and diagnostics

**Deliverables:**
- Full Bluetooth management interface
- Device pairing and connection
- Device-specific controls

### Week 7: Display Management Page

**Goals:**
- Implement multi-monitor support
- Add display configuration
- Create display profiles

**Technical Tasks:**
- [ ] Integrate with display protocols (X11/Wayland)
- [ ] Create display detection and enumeration
- [ ] Implement resolution and refresh rate controls
- [ ] Add display arrangement interface with drag-and-drop
- [ ] Create display mirroring and extending options
- [ ] Implement display profiles (home, work, presentation)
- [ ] Add color temperature and brightness controls
- [ ] Create display calibration tools integration
- [ ] Implement display rotation controls
- [ ] Add night light/blue light filter controls

**Deliverables:**
- Complete display management interface
- Multi-monitor configuration
- Display profiles and presets

### Week 8: Audio Management Page

**Goals:**
- Implement comprehensive audio control
- Add device management
- Create audio profiles

**Technical Tasks:**
- [ ] Integrate with PulseAudio/PipeWire via D-Bus
- [ ] Create audio device enumeration (input/output)
- [ ] Implement volume controls with `AdwScaleRow`
- [ ] Add audio device switching and management
- [ ] Create audio profile management (stereo, surround, etc.)
- [ ] Implement audio effects and equalizer integration
- [ ] Add microphone settings and noise cancellation
- [ ] Create audio troubleshooting tools
- [ ] Implement audio format and quality settings
- [ ] Add system sounds configuration

**Deliverables:**
- Full audio management interface
- Device switching and controls
- Audio profiles and effects

### Week 9: Settings Persistence & State Management

**Goals:**
- Implement comprehensive settings storage
- Add application state persistence
- Create backup and restore functionality

**Technical Tasks:**
- [ ] Implement `GSettings` schema for all preferences
- [ ] Create settings migration system for updates
- [ ] Add application state persistence (window size, position)
- [ ] Implement user preferences backup/restore
- [ ] Create settings import/export functionality
- [ ] Add settings validation and error handling
- [ ] Implement settings synchronization across sessions
- [ ] Create settings reset functionality
- [ ] Add settings search and filtering
- [ ] Implement settings categories and organization

**Deliverables:**
- Complete settings persistence system
- Backup and restore functionality
- Settings management tools

### Week 10: Advanced Features & Polish

**Goals:**
- Add advanced system features
- Implement search functionality
- Create system integration

**Technical Tasks:**
- [ ] Implement global search across all pages
- [ ] Add quick actions and shortcuts
- [ ] Create system service integration (systemd)
- [ ] Implement log viewing and system diagnostics
- [ ] Add system maintenance tools
- [ ] Create performance monitoring and optimization
- [ ] Implement system update notifications
- [ ] Add accessibility features and high contrast support
- [ ] Create user account management integration
- [ ] Implement system backup and restore tools

**Deliverables:**
- Advanced system features
- Search and quick actions
- System integration tools

### Week 11: Testing & Quality Assurance

**Goals:**
- Comprehensive testing implementation
- Performance optimization
- Security auditing

**Technical Tasks:**
- [ ] Create unit tests for all core functionality
- [ ] Implement integration tests for D-Bus interactions
- [ ] Add UI tests using GTK test framework
- [ ] Create performance benchmarks and optimization
- [ ] Implement memory leak detection and fixes
- [ ] Add security audit for D-Bus permissions
- [ ] Create error handling and recovery tests
- [ ] Implement stress testing for system monitoring
- [ ] Add accessibility testing and compliance
- [ ] Create automated testing pipeline

**Deliverables:**
- Comprehensive test suite
- Performance optimizations
- Security compliance

### Week 12: Documentation & Deployment

**Goals:**
- Complete documentation
- Package preparation
- Release preparation

**Technical Tasks:**
- [ ] Create comprehensive user documentation
- [ ] Write developer documentation and API reference
- [ ] Implement help system integration
- [ ] Create installation and setup guides
- [ ] Prepare distribution packages (RPM, DEB)
- [ ] Create Flatpak package and manifest
- [ ] Implement automatic update system
- [ ] Create release notes and changelog
- [ ] Add troubleshooting guides
- [ ] Prepare project website and documentation site

**Deliverables:**
- Complete documentation
- Distribution packages
- Release-ready application

## Technical Architecture

### Core Components

**Application Layer:**
- `AdwApplication` - Main application class
- `AdwApplicationWindow` - Primary window
- Action system for menu and shortcuts

**UI Layer:**
- `AdwHeaderBar` with `AdwViewSwitcher` # Plans changed, we won't have AdwHeaderBar
- `AdwViewStack` for page management
- `AdwPreferencesPage` for each settings category
- Custom widgets for specialized controls

**System Integration Layer:**
- D-Bus clients for system services
- Async/await pattern for non-blocking operations
- Signal handling for real-time updates

**Data Layer:**
- `GSettings` for preferences persistence
- SQLite for complex data storage
- Configuration file management

## Cargo.toml Versions

The following dependencies will be used with their latest stable versions.

```toml
[dependencies]
gtk4 = "0.9.7"
libadwaita = "0.7.2"
glib = "0.20.12"
gio = "0.20.12"
zbus = "5.7.1"
tokio = "1.45.1"
serde = { version = "1.0.219", features = ["derive"] }
serde_json = "1.0.140"
tracing = "0.1.41"
tracing-subscriber = "0.3.19"
anyhow = "1.0.98"
thiserror = "2.0.12"
```

## Success Metrics

- **Performance:** < 100MB RAM usage, < 5% CPU when idle, < 1s Startup time
- **Responsiveness:** All UI operations < 100ms response time
- **Reliability:** Zero crashes during normal operation
- **Compatibility:** Works on major Linux distributions
- **Usability:** Intuitive interface following GNOME HIG
- **Accessibility:** Full keyboard navigation and screen reader support

## Risk Mitigation

**Technical Risks:**
- D-Bus service availability - implement graceful fallbacks
- Permission issues - proper error handling and user guidance
- Performance with real-time updates - efficient data polling
- Cross-distribution compatibility - comprehensive testing

**Timeline Risks:**
- Complex system integration - allocate buffer time
- UI/UX refinement - iterative design process
- Testing and debugging - dedicated QA week

This plan provides a structured approach to building a comprehensive system control panel while maintaining focus on code quality, user experience, and system integration.
