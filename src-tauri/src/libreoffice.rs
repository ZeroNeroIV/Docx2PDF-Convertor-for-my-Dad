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

    /// Check if LibreOffice is available in the system PATH
    fn find_system_libreoffice() -> Option<PathBuf> {
        let binary_name = Self::get_so_binary_name();
        
        // Try to find in PATH
        if let Ok(output) = Command::new("which").arg(binary_name).output() {
            if output.status.success() {
                let path = String::from_utf8_lossy(&output.stdout).trim().to_string();
                if !path.is_empty() {
                    return Some(PathBuf::from(path));
                }
            }
        }

        // Try common locations on Linux
        #[cfg(target_os = "linux")]
        {
            let common_paths = [
                "/usr/bin/soffice",
                "/usr/local/bin/soffice",
                "/opt/libreoffice/program/soffice",
            ];
            
            for path in &common_paths {
                if Path::new(path).exists() {
                    return Some(PathBuf::from(path));
                }
            }
        }

        None
    }

    pub async fn ensure_libreoffice(&self) -> Result<PathBuf> {
        // First, check if we already have a cached path
        if let Some(ref path) = self.lo_path {
            if path.exists() {
                return Ok(path.clone());
            }
        }

        // Check for bundled LibreOffice
        let lo_dir = Self::get_libreoffice_dir();
        let bundled_path = lo_dir.join(Self::get_so_binary_name());

        if bundled_path.exists() {
            return Ok(bundled_path);
        }

        // Check for system LibreOffice
        if let Some(system_path) = Self::find_system_libreoffice() {
            println!("Using system LibreOffice: {:?}", system_path);
            return Ok(system_path);
        }

        // Need to extract bundled LibreOffice
        self.extract_libreoffice().await?;

        if !bundled_path.exists() {
            anyhow::bail!("Failed to extract LibreOffice");
        }

        Ok(bundled_path)
    }

    async fn extract_libreoffice(&self) -> Result<()> {
        let lo_dir = Self::get_libreoffice_dir();
        fs::create_dir_all(&lo_dir)?;

        // Determine which bundled archive to use based on platform
        let archive_name = if cfg!(target_os = "windows") {
            "libreoffice-win.zip"
        } else if cfg!(target_os = "macos") {
            "libreoffice-mac.zip"
        } else {
            "libreoffice-linux.zip"
        };

        // Check if bundled archive exists
        let exe_dir = std::env::current_exe()
            .context("Failed to get current executable path")?
            .parent()
            .context("Failed to get executable directory")?
            .to_path_buf();

        let archive_path = exe_dir.join("libreoffice").join(&archive_name);

        if !archive_path.exists() {
            // For development, check in the project directory
            let project_archive = PathBuf::from("libreoffice").join(&archive_name);
            if project_archive.exists() {
                return self.extract_zip(&project_archive, &lo_dir).await;
            }

            // Check a few more locations
            let alt_paths = [
                exe_dir.join("../libreoffice").join(&archive_name),
                exe_dir.join("../../libreoffice").join(&archive_name),
                PathBuf::from("./libreoffice").join(&archive_name),
            ];

            for alt_path in &alt_paths {
                if alt_path.exists() {
                    return self.extract_zip(alt_path, &lo_dir).await;
                }
            }

            anyhow::bail!(
                "LibreOffice not found. Please either:\n\
                1. Install LibreOffice on your system, or\n\
                2. Bundle LibreOffice by placing {} in the libreoffice/ directory",
                archive_name
            );
        }

        self.extract_zip(&archive_path, &lo_dir).await
    }

    async fn extract_zip(&self, zip_path: &Path, output_dir: &Path) -> Result<()> {
        println!("Extracting LibreOffice from {:?}...", zip_path);

        // Use the zip crate to extract
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

            // Set permissions on Unix
            #[cfg(unix)]
            {
                use std::os::unix::fs::PermissionsExt;
                if let Some(mode) = file.unix_mode() {
                    fs::set_permissions(&outpath, fs::Permissions::from_mode(mode))?;
                }
            }
        }

        println!("LibreOffice extracted successfully to {:?}", output_dir);
        Ok(())
    }

    pub async fn convert_file(&self, input_path: &str, output_path: &str) -> Result<()> {
        let soffice_path = self.ensure_libreoffice().await?;
        
        let input_path = PathBuf::from(input_path);
        let output_dir = PathBuf::from(output_path)
            .parent()
            .map(|p| p.to_path_buf())
            .unwrap_or_else(|| PathBuf::from("."));

        // Ensure output directory exists
        fs::create_dir_all(&output_dir)?;

        println!("Converting {:?} to PDF...", input_path);

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

        println!("Conversion completed: {:?}", output_path);
        Ok(())
    }

    /// Check if LibreOffice is available (for UI status)
    pub async fn is_available(&self) -> bool {
        self.ensure_libreoffice().await.is_ok()
    }
}
