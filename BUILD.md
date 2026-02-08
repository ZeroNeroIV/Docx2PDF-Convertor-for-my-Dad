# Docx2PDF Converter - Build Instructions

## Quick Start

```bash
# 1. Setup (install dependencies)
./setup.sh

# 2. Run in development mode
npm run tauri:dev

# 3. Build for production
npm run tauri:build
```

## Prerequisites

### All Platforms
- Node.js 18+
- Rust (latest stable)

### Linux
```bash
sudo apt update
sudo apt install libwebkit2gtk-4.0-dev libssl-dev libgtk-3-dev
```

### Windows
- Visual Studio Build Tools with C++ workload

## Development

### Start Development Server
```bash
npm run tauri:dev
```

This will:
1. Start the Vite dev server
2. Launch the Tauri window
3. Enable hot reload

### File Structure
```
src/              # React frontend
src-tauri/src/    # Rust backend
libreoffice/      # LibreOffice bundles (add here)
```

## Building

### Current Platform
```bash
npm run tauri:build
```

Output locations:
- **Windows**: `src-tauri/target/release/bundle/msi/*.msi`
- **Linux**: `src-tauri/target/release/bundle/appimage/*.AppImage`
- **macOS**: `src-tauri/target/release/bundle/dmg/*.dmg`

### Cross-Compilation

See Tauri documentation for cross-compilation setup.

## Bundling LibreOffice

The app needs LibreOffice to convert documents. You have two options:

### Option 1: System LibreOffice (Linux only)
If LibreOffice is installed on the system, the app will use it automatically:
```bash
sudo apt install libreoffice-writer  # Debian/Ubuntu
sudo dnf install libreoffice-writer   # Fedora
```

### Option 2: Bundled LibreOffice (Recommended)
Bundle LibreOffice with your app:

**Windows:**
1. Download LibreOffice Portable
2. Extract and zip as `libreoffice/libreoffice-win.zip`

**Linux:**
1. Download LibreOffice portable or AppImage
2. Extract and zip as `libreoffice/libreoffice-linux.zip`

Run `./download-libreoffice.sh` for detailed instructions.

## Troubleshooting

### "LibreOffice not found"
- Install system LibreOffice, OR
- Add LibreOffice bundles to `libreoffice/` directory

### Build fails on Linux
```bash
# Install missing dependencies
sudo apt install libwebkit2gtk-4.0-dev libssl-dev
```

### Frontend build errors
```bash
# Rebuild frontend
npm run build
```

### Rust compilation errors
```bash
cd src-tauri
cargo clean
cargo build
```

## Distribution

After building:

1. **Windows**: Distribute the `.msi` installer
2. **Linux**: Distribute the `.AppImage` (no installation needed)
3. **macOS**: Distribute the `.dmg` file

The app is completely portable and works offline!

## Customization

### Change App Name
Edit these files:
- `package.json` - "name" field
- `src-tauri/tauri.conf.json` - "identifier" and window title
- `src-tauri/Cargo.toml` - package name

### Change Icons
Replace files in `src-tauri/icons/`:
- `32x32.png`
- `128x128.png`
- `128x128@2x.png`
- `icon.icns` (macOS)
- `icon.ico` (Windows)

### Enable Auto-Updates
1. Set up a GitHub repository
2. Edit `src-tauri/src/main.rs`:
   - Replace GitHub API URL in `check_for_updates()`
3. Enable updater in `tauri.conf.json`

## License

MIT - Modify and distribute freely!
