# Docx2PDF Converter

A simple, offline DOCX to PDF converter built with Tauri (Rust + React + TypeScript). Perfect for your dad!

## Features

- 🚀 **Drag & drop** DOCX files
- 📦 **Batch conversion** of multiple files
- 🔒 **Completely offline** - no internet required
- 🖥️ **Cross-platform** - works on Windows and Linux
- 🎯 **Simple UI** - easy for non-technical users
- 📊 **Progress tracking** with visual feedback
- 🔄 **Auto-update checker** (works offline, checks when online)

## Prerequisites

Before building, you need:

1. **Rust** (latest stable)
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```

2. **Node.js** (v18 or later)
   ```bash
   # Using nvm (recommended)
   nvm install 18
   nvm use 18
   ```

3. **System dependencies**
   - Linux: `sudo apt install libwebkit2gtk-4.0-dev libssl-dev`
   - Windows: Visual Studio Build Tools

## Setup

1. **Clone/download this project**
   ```bash
   cd docx2pdf-converter
   ```

2. **Install Node dependencies**
   ```bash
   npm install
   ```

3. **Download LibreOffice Portable**
   
   You need to bundle LibreOffice with the app. Download the portable version:
   
   - **Windows**: Download [LibreOffice Portable](https://portableapps.com/apps/office/libreoffice_portable)
     - Extract and zip the contents as `libreoffice/libreoffice-win.zip`
   
   - **Linux**: Download [LibreOffice AppImage](https://www.libreoffice.org/download/appimage/) or use your package manager
     - Extract and zip as `libreoffice/libreoffice-linux.zip`

   The app will extract these on first run.

## Development

Run the app in development mode:

```bash
npm run tauri:dev
```

## Building

### Build for current platform:

```bash
npm run tauri:build
```

This creates:
- **Windows**: `src-tauri/target/release/bundle/msi/*.msi`
- **Linux**: `src-tauri/target/release/bundle/appimage/*.AppImage`

### Build for specific platform:

```bash
# Windows (from Linux with cross-compilation setup)
npm run tauri:build -- --target x86_64-pc-windows-msvc

# Linux
npm run tauri:build -- --target x86_64-unknown-linux-gnu
```

## Project Structure

```
docx2pdf-converter/
├── src/                    # React frontend
│   ├── components/         # UI components
│   ├── App.tsx            # Main app component
│   └── main.tsx           # Entry point
├── src-tauri/             # Rust backend
│   ├── src/
│   │   ├── main.rs        # Main entry & commands
│   │   └── libreoffice.rs # LibreOffice management
│   └── tauri.conf.json    # Tauri configuration
├── libreoffice/           # Bundled LibreOffice (add zips here)
│   ├── libreoffice-win.zip
│   └── libreoffice-linux.zip
└── package.json
```

## How It Works

1. **Frontend (React)**: Provides drag-and-drop interface, file selection, and progress display
2. **Backend (Rust)**: 
   - Manages file dialogs using Tauri APIs
   - Handles LibreOffice extraction and execution
   - Performs DOCX→PDF conversion via LibreOffice headless mode
3. **LibreOffice**: The actual conversion engine, bundled with the app

## Customization

### Change app name:
Edit `src-tauri/tauri.conf.json`:
- `tauri.bundle.identifier`
- `tauri.windows[0].title`

### Update check:
Edit `src-tauri/src/main.rs`:
- Replace GitHub API URL in `check_for_updates()` with your repository

## Troubleshooting

**"LibreOffice bundle not found"**
- Ensure you've downloaded and zipped LibreOffice to the `libreoffice/` directory

**Build fails on Linux**
- Install dependencies: `sudo apt install libwebkit2gtk-4.0-dev libssl-dev`

**App is too large**
- LibreOffice is ~150-200MB, this is expected for standalone offline conversion

## License

MIT - Feel free to modify and distribute!

## For Your Dad

Once built:
1. Install the `.msi` (Windows) or run the `.AppImage` (Linux)
2. Open the app
3. Drag DOCX files into the window
4. Click "Convert"
5. Find PDFs in the same folder as your DOCX files

That's it! No internet needed, no technical knowledge required.
