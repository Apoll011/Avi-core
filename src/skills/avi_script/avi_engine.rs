use std::env::set_current_dir;
use std::error::Error;
use std::path::{Path, PathBuf};
use rhai::{Array, Engine, Scope};
use crate::skills::avi_script::engine::create_avi_script_engine;
use crate::skills::avi_script::modules::register_modules;

pub fn get_avi_script_engine() -> Result<Engine, Box<dyn Error>>
{
    let engine = create_avi_script_engine(register_modules);

    Ok(engine?)
}

pub fn run_avi_script(engine: &Engine, filename: &str, skill_path: PathBuf, scope: &mut Scope) -> Result<(), Box<dyn Error>>{
    let _guard = set_current_dir(skill_path).expect("Failed to set temporary CWD");

    engine.run_file_with_scope(scope, filename.into())?;

    let _guard = set_current_dir(Path::new("../")).expect("Failed to set temporary CWD");

    Ok(())
}