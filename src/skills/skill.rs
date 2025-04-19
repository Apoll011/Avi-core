
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
use std::path::{Path};
use rhai::{Engine, Scope};
use crate::intent::sloth_extrator::ExtractedSlots;
use crate::skills::avi_script::avi_engine::run_avi_script;
use crate::skills::skill_metadata::SkillMetadata;

pub struct Skill<'a> {
    path: String,
    metadata: SkillMetadata,
    scope: Scope<'a>
}

impl<'a> Skill<'a> {
    pub(crate) fn new(path: &str, metadata: SkillMetadata, scope: Scope<'a>) -> Self {
        Skill {
            path: path.to_string(),
            metadata,
            scope
        }
    }

    pub(crate) fn on_intent(&mut self, intent: ExtractedSlots, engine: &Engine) -> Result<(), &'static str> {
        self.scope.push_constant("INTENT_NAME", intent.intent.clone())
            .push_constant("INTENT", intent.clone());

        let skill_path = Path::new(&self.path).join("skill.avi");
        let skill_path_file = skill_path.to_str().unwrap();
        
        run_avi_script(engine, skill_path_file, &mut self.scope).expect("Skill error!!");

        Ok(())
    }
}

