use std::collections::HashMap;
use std::fs::{self, File};
use std::io::{self, Read, Write};
use std::path::{Path, PathBuf};

pub struct AviScriptLibraryManager {
    library_dir: PathBuf,
    embedded_scripts: HashMap<String, &'static str>,
}

impl AviScriptLibraryManager {
    pub fn new(library_dir: impl Into<PathBuf>) -> Self {
        Self {
            library_dir: library_dir.into(),
            embedded_scripts: HashMap::new(),
        }
    }

    pub fn register_script(&mut self, name: &str, content: &'static str) -> &mut Self {
        self.embedded_scripts.insert(name.to_string(), content);
        self
    }

    pub fn register_scripts(&mut self, scripts: &[(&str, &'static str)]) -> &mut Self {
        for (name, content) in scripts {
            self.embedded_scripts.insert(name.to_string(), *content);
        }
        self
    }

    fn ensure_library_dir(&self) -> io::Result<()> {
        if !self.library_dir.exists() {
            fs::create_dir_all(&self.library_dir)?;
        }
        Ok(())
    }

    pub fn get_script_path(&self, script_name: &str) -> PathBuf {
        self.library_dir.join(script_name)
    }

    pub fn install_scripts(&self) -> io::Result<Vec<String>> {
        self.ensure_library_dir()?;

        let installed_scripts = self.update_scripts(false)?;

        Ok(installed_scripts)
    }

    pub fn update_scripts(&self, force: bool) -> io::Result<Vec<String>> {
        self.ensure_library_dir()?;

        let mut updated_scripts = Vec::new();

        for (name, content) in &self.embedded_scripts {
            let script_path = self.get_script_path(name);

            let should_update = if force {
                true
            } else if !script_path.exists() {
                true
            } else {
                // Read existing file to compare content
                let mut existing_content = String::new();
                File::open(&script_path)?.read_to_string(&mut existing_content)?;
                existing_content != *content
            };

            if should_update {
                let mut file = File::create(&script_path)?;
                file.write_all(content.as_bytes())?;
                updated_scripts.push(name.clone());
            }
        }

        Ok(updated_scripts)
    }

    pub fn list_available_scripts(&self) -> io::Result<Vec<String>> {
        if !self.library_dir.exists() {
            return Ok(Vec::new());
        }

        let mut scripts = Vec::new();

        for entry in fs::read_dir(&self.library_dir)? {
            let entry = entry?;
            let path = entry.path();

            if path.is_file() {
                if let Some(file_name) = path.file_name() {
                    if let Some(file_name_str) = file_name.to_str() {
                        if file_name_str.ends_with(".avi") {
                            scripts.push(file_name_str.to_string());
                        }
                    }
                }
            }
        }

        Ok(scripts)
    }

    pub fn library_dir(&self) -> &Path {
        &self.library_dir
    }
}

pub fn initialize_rhai_library() -> io::Result<AviScriptLibraryManager> {
    let library_dir = if cfg!(windows) {
        // On Windows, use %APPDATA%\resultyour_app_name\rhai_scripts
        let mut path = dirs::data_local_dir().unwrap_or_else(|| PathBuf::from("."));
        path.push("avi");
        path.push("library");
        path
    } else {
        // On Unix, use ~/.local/share/your_app_name/rhai_scripts or similar
        let mut path = dirs::data_local_dir().unwrap_or_else(|| PathBuf::from("."));
        path.push("avi");
        path.push("library");
        path
    };

    let mut manager = AviScriptLibraryManager::new(library_dir);

    manager.register_scripts(&[("config.avi", include_str!("library/config.avi"))]);

    manager.install_scripts()?;

    Ok(manager)
}
