use std::env::set_current_dir;
use std::error::Error;
use std::path::{Path, PathBuf};
use rhai::{Engine, Scope};
use crate::skills::avi_script::engine::create_avi_script_engine;
use crate::skills::avi_script::modules::register_modules;

pub fn get_avi_script_engine() -> Result<Engine, Box<dyn Error>>
{
    let engine = create_avi_script_engine(register_modules);

    engine
}

pub fn run_avi_script(engine: &Engine, filename: &str, skill_path: PathBuf, scope: &mut Scope) -> Result<(), Box<dyn Error>>{
    set_current_dir(skill_path).expect("Failed to set temporary CWD");

    engine.run_file_with_scope(scope, filename.into())?;

    set_current_dir(Path::new("../")).expect("Failed to set temporary CWD");

    Ok(())
}