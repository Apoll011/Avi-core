use crate::skills::avi_script::avi_engine::get_avi_script_engine;
use rhai::{Array, ImmutableString, Scope};
use std::path::{Path, PathBuf};

pub struct SkillMetadata {
    pub name: ImmutableString,
    pub id: ImmutableString,
    pub version: ImmutableString,
    pub author: ImmutableString,
    pub description: ImmutableString,
    pub languages: Array,
    pub license: ImmutableString,
}

impl SkillMetadata {
    pub fn load(path: &Path) -> SkillMetadata {
        let mut scope = Scope::new();
        let engine = get_avi_script_engine().unwrap();

        engine
            .run_file_with_scope(&mut scope, PathBuf::from(path).join("metadata.avi"))
            .expect("Error running metadata.avi!");

        SkillMetadata {
            name: scope.get_value::<ImmutableString>("name").unwrap(),
            id: scope.get_value::<ImmutableString>("id").unwrap(),
            version: scope.get_value::<ImmutableString>("version").unwrap(),
            author: scope.get_value::<ImmutableString>("author").unwrap(),
            description: scope.get_value::<ImmutableString>("description").unwrap(),
            languages: scope.get_value::<Array>("languages").unwrap(),
            license: scope.get_value::<ImmutableString>("license").unwrap(),
        }
    }
}
