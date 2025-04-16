use std::error::Error;
use rhai::{Array, Engine, Scope};
use crate::skills::avi_script::engine::create_avi_script_engine;
use crate::skills::avi_script::modules::register_modules;

pub fn get_avi_script_engine() -> Result<Engine, Box<dyn Error>>
{
    let engine = create_avi_script_engine(register_modules);

    Ok(engine?)
}

pub fn run_avi_script(engine: Engine, path: String) -> Result<(), Box<dyn Error>>{
    let mut scope = Scope::new();

    let supported_languages: Array = vec!["pt".into(), "en".into()];

    scope.push_constant("SKILL_NAME", "test")
        .push_constant("CURRENT_LANGUAGE", "pt")
        .push_constant("SUPPORTED_LANGUAGES", supported_languages);

    engine.run_file_with_scope(&mut scope, path.into())?;

    Ok(())
}