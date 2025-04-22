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

        let mut installed_scripts = Vec::new();

        for (name, content) in &self.embedded_scripts {
            let script_path = self.get_script_path(name);

            if !script_path.exists() {
                let mut file = File::create(&script_path)?;
                file.write_all(content.as_bytes())?;
            }

            installed_scripts.push(name.clone());
        }

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
                        if file_name_str.ends_with(".rhai") {
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

    manager.register_scripts(&[(
        "config.avi",
        r#"
fn load_file_as_map(file_path) {
    let content = read_string(open_file(file_path));
    if content.len() == 0 {
        throw `Config file '${file_path}' is empty or unreadable`;
    }

    return parse_json(content);
}

fn load_configs(file_path) {
    let data = load_file_as_map(file_path);
    if "configs" in data {
        return data["configs"];
    } else {
        throw `No 'configs' section found in ${file_path}`;
    }
}

fn load_constants(file_path) {
    let data = load_file_as_map(file_path);
    if "constants" in data {
        return data["constants"];
    } else {
        throw `No 'constants' section found in ${file_path}`;
    }
}

// =================== Config Access ===================

fn get_config(key) {
    let configs = load_configs("skill.config");
    if key in configs {
        return configs[key]["value"];
    } else {
        throw `Configuration key '${key}' not found`;
    }
}

fn set_config(key, value) {
    let configs = load_configs("skill.config");

    if key in configs {
        let config_type = configs[key]["type"];
        let value_type = type_of_value(value);

        if !is_compatible_type(value, config_type) {
            throw `Type mismatch: Expected ${config_type} but got ${value_type} for key '${key}'`;
        }

        configs[key]["value"] = value;
    } else {
        configs[key] = #{
            "value": value,
            "default": value,
            "type": type_of_value(value),
            "label": key,
            "description": ""
        };
    }

    return configs; // caller must save this if they want to persist it
}

fn has_config(key) {
    let configs = load_configs("skill.config");
    return key in configs;
}

fn reset_config(key) {
    let configs = load_configs("skill.config");
    if key in configs {
        configs[key]["value"] = configs[key]["default"];
        return configs;
    } else {
        throw `Configuration key '${key}' not found`;
    }
}

fn type_of_config(key) {
    let configs = load_configs("skill.config");
    if key in configs {
        return configs[key]["type"];
    } else {
        throw `Configuration key '${key}' not found`;
    }
}

fn get_all_config_keys() {
    return load_configs("skill.config").keys();
}

// =================== Constants ===================

fn get_const(key) {
    let constants = load_constants("skill.config");
    if key in constants {
        return constants[key];
    } else {
        throw `Constant '${key}' not found`;
    }
}

fn has_const(key) {
    let constants = load_constants("skill.config");
    return key in constants;
}

fn get_all_const_keys() {
    return load_constants("skill.config").keys();
}

// =================== Helpers ===================

fn type_of_value(value) {
    let type_name = type_of(value);

    if type_name == "i64" || type_name == "decimal" {
        return "int";
    } else if type_name == "f64" {
        return "float";
    } else if type_name == "string" {
        return "string";
    } else if type_name == "bool" {
        return "bool";
    } else if type_name == "array" {
        return "list";
    } else if type_name == "map" {
        return "map";
    } else {
        return type_name;
    }
}

fn is_compatible_type(value, expected_type) {
    let actual_type = type_of_value(value);

    if actual_type == expected_type {
        return true;
    }

    if expected_type == "float" && actual_type == "int" {
        return true;
    }

    return false;
}
            "#,
    )]);

    manager.install_scripts()?;

    Ok(manager)
}
