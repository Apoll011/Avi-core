/*
    ```
my_skill/
├── skill.avi
├── metadata.avi
├── config/
│   ├── default.json
│   └── runtime.json
├── intents/
│   └── (at least one file).intent
├── responses/
├── assets/
├── extensions/
├── tests/
└── README.md
```

```avi
name = "Greeting Skill"
id = "greet.skill"
version = "1.0.0"
author = "You"
description = "Greets the user and responds with kind words."
language = ["en", "es"]
license = "MIT"
```
*/
use crate::intent::engine::IntentEngine;
use crate::intent::slot_extrator::ExtractedSlots;
use crate::skills::avi_script::avi_engine::{get_avi_script_engine, run_avi_script};
use crate::skills::skill_metadata::SkillMetadata;
use rhai::{Engine, Scope};
use std::fs;
use std::path::{Path, PathBuf};

pub struct Skill<'a> {
    path: String,
    metadata: SkillMetadata,
    engine: Engine,
    scope: Scope<'a>,
}

impl<'a> Skill<'a> {
    pub(crate) fn new(path: &str, metadata: SkillMetadata, scope: Scope<'a>) -> Self {
        let engine = get_avi_script_engine().unwrap();

        Skill {
            path: path.to_string(),
            metadata,
            engine,
            scope,
        }
    }

    pub(crate) fn metadata(&self) -> &SkillMetadata {
        &self.metadata
    }

    pub(crate) fn start(&mut self) {
        run_avi_script(&self.engine, "skill.avi", self.get_path(), &mut self.scope)
            .expect("Skill Start error!!");
    }

    pub(crate) fn load_intents(&mut self, intent_engine: &mut IntentEngine) -> Vec<String> {
        let name = self.metadata.name.clone();
        let intents_path = std::env::current_dir()
            .unwrap()
            .join(self.get_path())
            .join("intents");

        let mut extracted_names = Vec::<String>::new();

        if let Ok(entries) = fs::read_dir(&intents_path) {
            for entry in entries.flatten() {
                match intent_engine.load_intent(entry.path()) {
                    Ok(slots) => {
                        extracted_names.push(slots);
                    }
                    Err(err) => {
                        eprintln!("Error importing intents for skill {}: {}", name, err);
                        continue;
                    }
                }
            }
        } else {
            eprintln!(
                "Could not read intents directory for skill {}, {:?}",
                name, intents_path
            );
        }

        extracted_names
    }

    pub(crate) fn get_path(&self) -> PathBuf {
        Path::new(self.path.as_str()).to_path_buf()
    }

    pub(crate) fn stop(&mut self) {
        self.scope.push_constant("END", true);
        run_avi_script(&self.engine, "skill.avi", self.get_path(), &mut self.scope)
            .expect("Skill End error!!");
    }

    pub(crate) fn on_intent(&mut self, intent: ExtractedSlots) -> Result<(), &'static str> {
        self.scope
            .push_constant("INTENT_NAME", intent.intent.clone())
            .push_constant("INTENT", intent.clone());

        run_avi_script(&self.engine, "skill.avi", self.get_path(), &mut self.scope)
            .expect("Skill error!!");

        Ok(())
    }
}
