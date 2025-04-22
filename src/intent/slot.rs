use serde::Serialize;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize)]
#[serde(tag = "type")]
pub enum SlotDefinition {
    #[serde(rename = "enumeration")]
    Enumeration { values: Vec<String> },

    #[serde(rename = "catch_all")]
    CatchAll,

    #[serde(rename = "catch_process")]
    CatchProcess {
        #[serde(skip)]
        processor: fn(String) -> Option<String>,
    },
}

impl SlotDefinition {
    pub(crate) fn new_enumeration(values: Vec<String>) -> Self {
        SlotDefinition::Enumeration { values }
    }

    pub(crate) fn new_catch_all() -> Self {
        SlotDefinition::CatchAll
    }

    pub(crate) fn new_processor(processor: fn(String) -> Option<String>) -> Self {
        SlotDefinition::CatchProcess { processor }
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

        defaults.insert(
            "dates".to_string(),
            SlotDefinition::new_processor(|date| {
                println!("{}", date);
                Option::from(date)
            }),
        );

        DefaultSlotManager { defaults }
    }

    pub(crate) fn get(&self, slot_name: &str) -> Option<&SlotDefinition> {
        self.defaults.get(slot_name)
    }
}
