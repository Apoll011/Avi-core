use std::collections::HashMap;
use std::fs;
use std::path::Path;

use crate::intent::engine::IntentEngine;
use crate::intent::slot_extrator::ExtractedSlots;
use crate::skills::skill::Skill;
use crate::skills::utils::load_skill;

pub struct SkillManager<'a> {
    skills: Vec<Skill<'a>>,
    intent_map: HashMap<String, usize>,
}

impl<'a> SkillManager<'a> {
    pub fn new() -> Self {
        SkillManager {
            skills: Vec::new(),
            intent_map: HashMap::new(),
        }
    }

    pub fn load_skill(&mut self, path: String) -> Result<&mut Skill<'a>, &'static str> {
        self.skills.push(load_skill(path).unwrap());

        Ok(self.skills.last_mut().unwrap())
    }

    pub fn load_skills_from_directory(
        &mut self,
        directory: &str,
        intent_engine: &mut IntentEngine,
    ) -> Result<&mut Self, &'static str> {
        let dir_path = Path::new(directory);
        if !dir_path.is_dir() {
            return Err("Invalid directory path");
        }

        if let Ok(entries) = fs::read_dir(dir_path) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.is_dir() {
                    if let Some(path_str) = path.to_str() {
                        let path_string = path_str.to_string();
                        match self.load_skill(path_string) {
                            Ok(_) => {
                                let skill_index = self.skills.len() - 1;
                                let skill = &mut self.skills[skill_index];

                                // Load intents as strings and map them to this skill
                                let intent_names = skill.load_intents(intent_engine);

                                for intent_name in intent_names {
                                    self.intent_map.insert(intent_name, skill_index);
                                }

                                // Start the skill
                                skill.start();
                            }
                            Err(e) => {
                                eprintln!("Failed to load skill at {:?}: {}", path, e);
                            }
                        }
                    }
                }
            }
        }

        Ok(self)
    }

    pub fn process_intent(&mut self, slots: ExtractedSlots) -> Result<(), &'static str> {
        // Extract just the intent name string from the ExtractedSlots
        let intent_name = &slots.intent;

        // Find the skill that handles this intent
        if let Some(&skill_index) = self.intent_map.get(intent_name) {
            if let Some(skill) = self.skills.get_mut(skill_index) {
                return skill.on_intent(slots);
            }
        }

        Err("No skill found for this intent")
    }

    pub fn stop_all(&mut self) -> &mut Self {
        for skill in &mut self.skills {
            skill.stop();
        }
        self
    }
}
