use anyhow::{Context, Result};
use std::path::{Path, PathBuf};
use std::process::Command;
use std::fs;
use dirs;

pub struct LibreOfficeManager {
    lo_path: Option<PathBuf>,
}

impl LibreOfficeManager {
    pub fn new() -> Self {
        Self { lo_path: None }
    }

    fn get_app_data_dir() -> PathBuf {
        dirs::data_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join("docx2pdf-converter")
    }

    fn get_libreoffice_dir() -> PathBuf {
        Self::get_app_data_dir().join("libreoffice")
    }

    fn get_so_binary_name() -> &'static str {
        if cfg!(target_os = "windows") {
            "soffice.exe"
        } else {
            "soffice"
        }
    }

    /// Find soffice binary by checking multiple sources in priority order:
    /// 1. System installation (registry on Windows, common paths on Linux)
    /// 2. PATH environment variable
    /// 3. Bundled version (Windows only)
    fn find_libreoffice() -> Option<PathBuf> {
        // Priority 1: Check system-wide installation
        #[cfg(target_os = "windows")]
        if let Some(path) = Self::find_windows_registry_libreoffice() {
            println!("Found LibreOffice in Windows registry: {:?}", path);
            return Some(path);
        }

        // Priority 2: Check common installation paths
        if let Some(path) = Self::find_common_path_libreoffice() {
            println!("Found LibreOffice in common path: {:?}", path);
            return Some(path);
        }

        // Priority 3: Check PATH environment variable
        if let Some(path) = Self::find_path_libreoffice() {
            println!("Found LibreOffice in PATH: {:?}", path);
            return Some(path);
        }

        // Priority 4: Check bundled version (Windows only)
        #[cfg(target_os = "windows")]
        if let Some(path) = Self::find_bundled_libreoffice() {
            println!("Found bundled LibreOffice: {:?}", path);
            return Some(path);
        }

        None
    }

    /// Check Windows registry for LibreOffice installation
    #[cfg(target_os = "windows")]
    fn find_windows_registry_libreoffice() -> Option<PathBuf> {
        use winreg::RegKey;
        use winreg::enums::HKEY_LOCAL_MACHINE;
        use winreg::enums::HKEY_CURRENT_USER;

        let binary_name = Self::get_so_binary_name();

        // Try HKEY_LOCAL_MACHINE (system-wide installation)
        let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
        if let Ok(libreoffice_key) = hklm.open_subkey("SOFTWARE\\LibreOffice") {
            if let Ok(path) = libreoffice_key.get_value::<String, _>("Path") {
                let soffice_path = PathBuf::from(&path).join("program").join(binary_name);
                if soffice_path.exists() {
                    return Some(soffice_path);
                }
            }
        }

        // Try HKEY_CURRENT_USER (user installation)
        let hkcu = RegKey::predef(HKEY_CURRENT_USER);
        if let Ok(libreoffice_key) = hkcu.open_subkey("SOFTWARE\\LibreOffice") {
            if let Ok(path) = libreoffice_key.get_value::<String, _>("Path") {
                let soffice_path = PathBuf::from(&path).join("program").join(binary_name);
                if soffice_path.exists() {
                    return Some(soffice_path);
                }
            }
        }

        None
    }

    /// Check common installation paths
    fn find_common_path_libreoffice() -> Option<PathBuf> {
        let binary_name = Self::get_so_binary_name();

        #[cfg(target_os = "windows")]
        let common_paths = [
            format!("C:\\Program Files\\LibreOffice\\program\\{}", binary_name),
            format!("C:\\Program Files (x86)\\LibreOffice\\program\\{}", binary_name),
        ];

        #[cfg(target_os = "linux")]
        let common_paths = [
            format!("/usr/bin/{}", binary_name),
            format!("/usr/local/bin/{}", binary_name),
            format!("/opt/libreoffice/program/{}", binary_name),
            format!("/usr/lib64/libreoffice/program/{}", binary_name),
            format!("/usr/lib/libreoffice/program/{}", binary_name),
            format!("/snap/bin/{}", binary_name),
        ];

        #[cfg(target_os = "macos")]
        let common_paths = [
            format!("/Applications/LibreOffice.app/Contents/MacOS/{}", binary_name),
        ];

        for path_str in &common_paths {
            let path = PathBuf::from(path_str);
            if path.exists() {
                return Some(path);
            }
        }

        None
    }

    /// Check PATH environment variable
    fn find_path_libreoffice() -> Option<PathBuf> {
        let binary_name = Self::get_so_binary_name();
        
        #[cfg(target_os = "windows")]
        {
            // On Windows, use 'where' command
            if let Ok(output) = Command::new("cmd").args(["/C", "where", binary_name]).output() {
                if output.status.success() {
                    let path_str = String::from_utf8_lossy(&output.stdout);
                    let first_path = path_str.lines().next()?.trim();
                    if !first_path.is_empty() {
                        let path = PathBuf::from(first_path);
                        if path.exists() {
                            return Some(path);
                        }
                    }
                }
            }
        }

        #[cfg(not(target_os = "windows"))]
        {
            // On Linux/Mac, use 'which' command
            if let Ok(output) = Command::new("which").arg(binary_name).output() {
                if output.status.success() {
                    let path_str = String::from_utf8_lossy(&output.stdout).trim().to_string();
                    if !path_str.is_empty() {
                        let path = PathBuf::from(&path_str);
                        if path.exists() {
                            return Some(path);
                        }
                    }
                }
            }
        }

        None
    }

    /// Check for bundled LibreOffice (Windows only)
    /// Searches in extracted location and can extract from zip if needed
    #[cfg(target_os = "windows")]
    fn find_bundled_libreoffice() -> Option<PathBuf> {
        let lo_dir = Self::get_libreoffice_dir();
        let binary_name = Self::get_so_binary_name();

        // Check if already extracted
        if let Some(path) = Self::search_for_binary_recursive(&lo_dir, binary_name) {
            return Some(path);
        }

        // Try to extract from bundled zip
        if let Ok(()) = Self::extract_bundled_libreoffice() {
            // Search again after extraction
            if let Some(path) = Self::search_for_binary_recursive(&lo_dir, binary_name) {
                return Some(path);
            }
        }

        None
    }

    /// Search recursively for a binary in a directory
    fn search_for_binary_recursive(dir: &Path, binary_name: &str) -> Option<PathBuf> {
        if let Ok(entries) = fs::read_dir(dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.is_dir() {
                    if let Some(found) = Self::search_for_binary_recursive(&path, binary_name) {
                        return Some(found);
                    }
                } else if path.file_name()
                    .and_then(|n| n.to_str())
                    .map(|n| n.eq_ignore_ascii_case(binary_name))
                    .unwrap_or(false)
                {
                    return Some(path);
                }
            }
        }
        None
    }

    /// Extract bundled LibreOffice from zip file (Windows only)
    #[cfg(target_os = "windows")]
    fn extract_bundled_libreoffice() -> Result<()> {
        let lo_dir = Self::get_libreoffice_dir();
        fs::create_dir_all(&lo_dir)?;

        let archive_name = "libreoffice-win.zip";

        // Get executable directory
        let exe_dir = std::env::current_exe()
            .context("Failed to get current executable path")?
            .parent()
            .context("Failed to get executable directory")?
            .to_path_buf();

        // Possible locations for the zip file
        let possible_paths = [
            exe_dir.join("libreoffice").join(archive_name),
            exe_dir.join("../libreoffice").join(archive_name),
            exe_dir.join("../../libreoffice").join(archive_name),
            PathBuf::from("./libreoffice").join(archive_name),
            // For development builds
            PathBuf::from("libreoffice").join(archive_name),
        ];

        for archive_path in &possible_paths {
            if archive_path.exists() {
                println!("Extracting LibreOffice from {:?}...", archive_path);
                return Self::extract_zip_sync(archive_path, &lo_dir);
            }
        }

        anyhow::bail!("Bundled LibreOffice zip not found in any of the expected locations")
    }

    /// Extract a zip file synchronously
    fn extract_zip_sync(zip_path: &Path, output_dir: &Path) -> Result<()> {
        let file = fs::File::open(zip_path)
            .with_context(|| format!("Failed to open archive: {:?}", zip_path))?;

        let mut archive = zip::ZipArchive::new(file)
            .context("Failed to read zip archive")?;

        for i in 0..archive.len() {
            let mut file = archive.by_index(i)?;
            let outpath = output_dir.join(file.name());

            if file.name().ends_with('/') {
                fs::create_dir_all(&outpath)?;
            } else {
                if let Some(p) = outpath.parent() {
                    if !p.exists() {
                        fs::create_dir_all(p)?;
                    }
                }
                let mut outfile = fs::File::create(&outpath)?;
                std::io::copy(&mut file, &mut outfile)?;
            }
        }

        println!("LibreOffice extracted successfully to {:?}", output_dir);
        Ok(())
    }

    pub async fn ensure_libreoffice(&self) -> Result<PathBuf> {
        // First, check if we already have a cached path
        if let Some(ref path) = self.lo_path {
            if path.exists() {
                return Ok(path.clone());
            }
        }

        // Search for LibreOffice using priority order
        if let Some(path) = Self::find_libreoffice() {
            return Ok(path);
        }

        // Nothing found
        #[cfg(target_os = "windows")]
        anyhow::bail!(
            "LibreOffice not found. Please either:\n\
            1. Install LibreOffice on your system, or\n\
            2. Ensure libreoffice-win.zip is bundled in the application"
        );

        #[cfg(not(target_os = "windows"))]
        anyhow::bail!(
            "LibreOffice not found. Please install LibreOffice on your system:\n\
            - Ubuntu/Debian: sudo apt install libreoffice-writer\n\
            - Fedora: sudo dnf install libreoffice-writer\n\
            - Arch: sudo pacman -S libreoffice-still"
        );
    }

    pub async fn convert_file(&self, input_path: &str, output_path: &str) -> Result<PathBuf> {
        let soffice_path = self.ensure_libreoffice().await?;
        
        let input_path = PathBuf::from(input_path);
        let output_dir = PathBuf::from(output_path)
            .parent()
            .map(|p| p.to_path_buf())
            .unwrap_or_else(|| PathBuf::from("."));

        // Ensure output directory exists
        fs::create_dir_all(&output_dir)?;

        println!("Converting {:?} to PDF using {:?}...", input_path, soffice_path);

        let mut cmd = Command::new(&soffice_path);
        cmd.arg("--headless")
            .arg("--convert-to")
            .arg("pdf")
            .arg("--outdir")
            .arg(&output_dir)
            .arg(&input_path);

        let output = cmd.output()
            .with_context(|| format!("Failed to execute LibreOffice: {:?}", soffice_path))?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            let stdout = String::from_utf8_lossy(&output.stdout);
            anyhow::bail!(
                "LibreOffice conversion failed:\nstdout: {}\nstderr: {}",
                stdout,
                stderr
            );
        }

        println!("Conversion completed successfully");
        Ok(soffice_path)
    }

    /// Check if LibreOffice is available (for UI status)
    pub async fn is_available(&self) -> bool {
        Self::find_libreoffice().is_some()
    }
}
