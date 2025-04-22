use crate::skills::avi_script::engine::create_avi_script_engine;
use crate::skills::avi_script::modules::register_modules;
use rhai::{Engine, Scope};
use std::env::{set_current_dir, current_dir};
use std::error::Error;
use std::path::{Path, PathBuf};

pub fn get_avi_script_engine() -> Result<Engine, Box<dyn Error>> {
    create_avi_script_engine(register_modules)
}

pub fn run_avi_script(
    engine: &Engine,
    filename: &str,
    skill_path: PathBuf,
    scope: &mut Scope,
) -> Result<(), Box<dyn Error>> {
    let root_path = current_dir().unwrap_or_default();

    set_current_dir(skill_path).expect("Failed to set temporary CWD");

    engine.run_file_with_scope(scope, filename.into())?;

    set_current_dir(Path::new(&root_path)).expect("Failed to set temporary CWD");

    Ok(())
}
