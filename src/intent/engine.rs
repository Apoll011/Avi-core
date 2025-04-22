use crate::intent::intent::{Intent, IntentFile};
use crate::intent::slot::{DefaultSlotManager, SlotDefinition};
use serde_json;
use std::collections::HashMap;
use std::error::Error;
use std::fs;
use std::path::Path;

pub struct IntentEngine {
    pub(crate) intents: Vec<Intent>,
    pub(crate) default_slots: DefaultSlotManager,
}

impl IntentEngine {
    pub(crate) fn new() -> Self {
        IntentEngine {
            intents: Vec::new(),
            default_slots: DefaultSlotManager::new(),
        }
    }

    pub(crate) fn load_intent<P: AsRef<Path>>(
        &mut self,
        file_path: P,
    ) -> Result<String, Box<dyn Error>> {
        let content = fs::read_to_string(file_path)?;
        let data: IntentFile = serde_json::from_str(&content)?;

        // Check if the intent has at least one pattern or regex pattern
        if !data.is_valid() {
            return Err("Intent must have at least one pattern or regex pattern".into());
        }

        let slots = self.parse_slot_defs(&data.slots).unwrap_or_default();
        let intent = Intent {
            name: data.intent,
            patterns: data.patterns,
            regex_patterns: data.regex_patterns,
            slots,
        };
        let name = intent.name.clone();
        self.intents.push(intent);
        Ok(name)
    }

    fn parse_slot_defs(
        &self,
        raw_slots: &HashMap<String, serde_json::Value>,
    ) -> Result<HashMap<String, SlotDefinition>, Box<dyn Error>> {
        let mut defs = HashMap::new();
        for (slot, val) in raw_slots {
            if val.is_string() && val.as_str().unwrap() == "*" {
                defs.insert(slot.clone(), SlotDefinition::new_catch_all());
            } else if val.is_array() {
                let values: Vec<String> = serde_json::from_value(val.clone())?;
                defs.insert(slot.clone(), SlotDefinition::new_enumeration(values));
            } else {
                return Err(format!("Invalid slot definition for {}: {:?}", slot, val).into());
            }
        }

        Ok(defs)
    }
}
