use std::error::Error;
use rhai::{Array, Engine, Scope};
use crate::skills::avi_script::engine::create_avi_script_engine;
use crate::skills::avi_script::modules::register_modules;

pub fn get_avi_script_engine() -> Result<Engine, Box<dyn Error>>
{
    let engine = create_avi_script_engine(register_modules);

    Ok(engine?)
}

pub fn run_avi_script(engine: &Engine, path: &str, scope: &mut Scope) -> Result<(), Box<dyn Error>>{
    engine.run_file_with_scope(scope, path.into())?;

    Ok(())
}