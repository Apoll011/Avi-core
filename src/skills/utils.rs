use std::path::Path;
use rhai::{Array, Scope};
use crate::skills::skill::Skill;
use crate::skills::skill_metadata::SkillMetadata;

fn is_valid_skill_folder(path: &str) -> bool {
    let folder = Path::new(path);
    if !folder.exists() || !folder.is_dir() {
        return false;
    }
    
    let required_folders = vec!["config", "intents", "responses"];
    for req_folder in required_folders {
        if !folder.join(req_folder).exists() || !folder.join(req_folder).is_dir(){
            return false;
        }
    }
    
    let required_files = vec!["skill.avi", "metadata.avi", "config/default.json", "config/runtime.json"];
    for req_files in required_files {
        if !folder.join(req_files).exists() || !folder.join(req_files).is_file() {
            return false;
        }
    }

    let mut valid = false;
    if let Ok(entries) = folder.join("intents").read_dir() {
        for entry in entries.flatten() {
            if let Some(ext) = entry.path().extension() {
                if ext == "intent" {
                    valid = true;
                    break;
                }
            }
        }
    }

    valid
}

pub fn load_skill(path: &str) -> Result<Skill, &'static str> {
    if !is_valid_skill_folder(path) {
        return Err("Not a valid skill!");
    }

    let folder = Path::new(path);

    let metadata = SkillMetadata::load(folder);

    let mut scope = Scope::new();

    let supported_languages: Array = vec!["pt".into(), "en".into()];

    scope.push_constant("SKILL_NAME", metadata.name.clone())
        .push_constant("SKILL_ID", metadata.id.clone())
        .push_constant("SKILL_VERSION", metadata.version.clone())
        .push_constant("SKILL_AUTHOR", metadata.author.clone())
        .push_constant("CURRENT_LANGUAGE", "pt")
        .push_constant("SUPPORTED_LANGUAGES", supported_languages);

    Ok(Skill::new(path, metadata, scope))
}

