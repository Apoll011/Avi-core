use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum SlotDefinition {
    #[serde(rename = "enumeration")]
    Enumeration { values: Vec<String> },

    #[serde(rename = "catch_all")]
    CatchAll,
}

impl SlotDefinition {
    pub(crate) fn new_enumeration(values: Vec<String>) -> Self {
        SlotDefinition::Enumeration { values }
    }

    pub(crate) fn new_catch_all() -> Self {
        SlotDefinition::CatchAll
    }
}

pub struct DefaultSlotManager {
    defaults: HashMap<String, SlotDefinition>,
}

impl DefaultSlotManager {
    pub(crate) fn new() -> Self {
        let mut defaults = HashMap::new();

        defaults.insert(
            "locations".to_string(),
            SlotDefinition::new_enumeration(vec![
                "new york".to_string(),
                "london".to_string(),
                "paris".to_string(),
                "tokyo".to_string(),
            ]),
        );

        defaults.insert("dates".to_string(), SlotDefinition::new_catch_all());

        DefaultSlotManager { defaults }
    }

    pub(crate) fn get(&self, slot_name: &str) -> Option<&SlotDefinition> {
        self.defaults.get(slot_name)
    }
}