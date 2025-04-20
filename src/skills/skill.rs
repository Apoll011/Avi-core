
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
use std::env::set_current_dir;
use std::path::{Path};
use std::time::Instant;
use rhai::{Engine, Scope};
use crate::intent::slot_extrator::ExtractedSlots;
use crate::skills::avi_script::avi_engine::{get_avi_script_engine, run_avi_script};
use crate::skills::skill_metadata::SkillMetadata;

pub struct Skill<'a> {
    path: String,
    metadata: SkillMetadata,
    engine: Engine,
    scope: Scope<'a>
}

impl<'a> Skill<'a> {
    pub(crate) fn new(path: &str, metadata: SkillMetadata, scope: Scope<'a>) -> Self {
        let _guard = set_current_dir(Path::new(path)).expect("Failed to set temporary CWD");
        
        let engine = get_avi_script_engine().unwrap();

        let skill = Skill {
            path: path.to_string(),
            metadata,
            engine,
            scope
        };
        
        skill
    }
    
    pub(crate) fn metadata(&self) -> &SkillMetadata {
        &self.metadata
    }
    
    pub(crate) fn start(&mut self) {
        run_avi_script(&self.engine, "skill.avi", &mut self.scope).expect("Skill Start error!!");
    }

    pub(crate) fn stop(&mut self) {
        self.scope.push_constant("END", true);
        run_avi_script(&self.engine, "skill.avi", &mut self.scope).expect("Skill End error!!");
    }
    
    pub(crate) fn on_intent(&mut self, intent: ExtractedSlots) -> Result<(), &'static str> {
        self.scope.push_constant("INTENT_NAME", intent.intent.clone())
            .push_constant("INTENT", intent.clone());
        
        run_avi_script(&self.engine, "skill.avi", &mut self.scope).expect("Skill error!!");

        Ok(())
    }
}

