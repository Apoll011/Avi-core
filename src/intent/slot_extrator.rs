use crate::intent::slot::{DefaultSlotManager, SlotDefinition};
use regex::{Regex, RegexBuilder};
use rhai::{Array, Dynamic, Map};
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct ExtractedSlots {
    pub(crate) intent: String,
    pub(crate) slots: HashMap<String, String>,
}

impl ExtractedSlots {
    pub(crate) fn get_name(&mut self) -> String {
        self.intent.clone()
    }

    pub(crate) fn get_slots(&mut self) -> HashMap<String, String> {
        self.slots.clone()
    }

    pub(crate) fn get(&mut self, slot: &str) -> Dynamic {
        self.slots.get(slot).cloned().unwrap_or_default().into()
    }

    pub(crate) fn get_raw(&mut self, slot: &str) -> Dynamic {
        self.get(slot) // For now behaves same as get; can differ if raw format needed
    }

    pub(crate) fn require(&mut self, slot: &str) -> Dynamic {
        self.slots
            .get(slot)
            .map(|v| v.clone().into())
            .unwrap_or_else(|| panic!("Required slot '{}' not found", slot))
    }

    pub(crate) fn optional(&mut self, slot: &str) -> Dynamic {
        self.get(slot)
    }

    pub(crate) fn exists(&mut self, slot1: &str, slot2: &str) -> bool {
        self.slots.contains_key(slot1) && self.slots.contains_key(slot2)
    }

    pub(crate) fn equal(&mut self, slot: &str, value: &str) -> bool {
        self.slots.get(slot).is_some_and(|v| v == value)
    }

    pub(crate) fn in_list(&mut self, slot: &str, list: Array) -> bool {
        match self.slots.get(slot) {
            Some(value) => list.iter().any(|item| item.to_string() == *value),
            None => false,
        }
    }

    pub(crate) fn in_dict(&mut self, slot: &str, map: Map) -> bool {
        match self.slots.get(slot) {
            Some(value) => map.contains_key(value.as_str()),
            None => false,
        }
    }

    pub(crate) fn obj(&mut self, slot: &str) -> Dynamic {
        self.get(slot) // Can be enhanced to return structured data
    }

    pub(crate) fn count(&mut self) -> i64 {
        self.slots.len() as i64
    }

    pub(crate) fn all(&mut self) -> Map {
        let mut map = Map::new();
        for (k, v) in &self.slots {
            map.insert(k.into(), v.clone().into());
        }
        map
    }

    pub(crate) fn match_pattern(&mut self, slot: &str, pattern: &str) -> bool {
        use regex::Regex;
        match self.slots.get(slot) {
            Some(value) => Regex::new(pattern).is_ok_and(|re| re.is_match(value)),
            None => false,
        }
    }

    pub(crate) fn is_type(&mut self, slot: &str, type_name: &str) -> bool {
        match self.slots.get(slot) {
            Some(value) => match type_name {
                "int" => value.parse::<i64>().is_ok(),
                "float" => value.parse::<f64>().is_ok(),
                "bool" => value.parse::<bool>().is_ok(),
                "string" => true,
                _ => false,
            },
            None => false,
        }
    }
}

pub struct SlotExtractor<'a> {
    default_slots: &'a DefaultSlotManager,
}

impl<'a> SlotExtractor<'a> {
    pub(crate) fn new(default_slots: &'a DefaultSlotManager) -> Self {
        SlotExtractor { default_slots }
    }

    fn validate_and_process_slot(&self, val_text: String, defn: &SlotDefinition) -> Option<String> {
        match defn {
            SlotDefinition::Enumeration { values } => {
                if values.iter().any(|v| v.eq_ignore_ascii_case(&val_text)) {
                    Some(val_text)
                } else {
                    None
                }
            }
            SlotDefinition::CatchAll => Some(val_text),
            SlotDefinition::CatchProcess { processor } => processor(val_text),
        }
    }

    pub(crate) fn extract_from_pattern(
        &self,
        pattern: &str,
        text: &str,
        intent_name: &str,
        intent_slots: &HashMap<String, SlotDefinition>,
    ) -> Option<ExtractedSlots> {
        let regex = self.pattern_to_regex(pattern)?;
        let captures = regex.captures(text)?;

        let mut slots = HashMap::new();

        for name in regex.capture_names().flatten() {
            if let Some(val) = captures.name(name) {
                let mut val_text = val.as_str().to_string();
                let defn;

                if name.starts_with("default_") {
                    let default_name = &name[8..]; // Skip "default_"
                    defn = self.default_slots.get(default_name)?;
                } else {
                    defn = intent_slots.get(name)?;
                }

                let processed = self.validate_and_process_slot(val.as_str().to_string(), defn);
                processed.as_ref()?;
                val_text = processed.unwrap().as_str().to_string();

                slots.insert(name.to_string(), val_text);
            }
        }

        Some(ExtractedSlots {
            intent: intent_name.to_string(),
            slots,
        })
    }

    pub(crate) fn extract_from_regex(
        &self,
        regex_pattern: &str,
        text: &str,
        intent_name: &str,
        intent_slots: &HashMap<String, SlotDefinition>,
    ) -> Option<ExtractedSlots> {
        let regex = RegexBuilder::new(regex_pattern)
            .case_insensitive(true)
            .build()
            .ok()?;

        let captures = regex.captures(text)?;

        let mut slots = HashMap::new();

        for name in regex.capture_names().flatten() {
            if let Some(val) = captures.name(name) {
                let mut val_text = val.as_str().to_string();

                // Intent-specific slot
                if let Some(defn) = intent_slots.get(name) {
                    let processed = self.validate_and_process_slot(val.as_str().to_string(), defn);
                    processed.as_ref()?;
                    val_text = processed.unwrap().as_str().to_string();
                }

                slots.insert(name.to_string(), val_text);
            }
        }

        Some(ExtractedSlots {
            intent: intent_name.to_string(),
            slots,
        })
    }

    fn pattern_to_regex(&self, pattern: &str) -> Option<Regex> {
        let mut regex_str = "^".to_string();
        let mut current_pos = 0;

        // Simple pattern tokenizer
        let chars: Vec<char> = pattern.chars().collect();

        while current_pos < chars.len() {
            if chars[current_pos] == '{' {
                // Find the closing brace
                let start_pos = current_pos;
                current_pos += 1;

                while current_pos < chars.len() && chars[current_pos] != '}' {
                    current_pos += 1;
                }

                if current_pos >= chars.len() {
                    // Unclosed brace, treat as literal
                    regex_str.push_str(&regex::escape(&pattern[start_pos..current_pos]));
                } else {
                    // Extract slot name
                    let slot_name = &pattern[(start_pos + 1)..current_pos];

                    if slot_name.starts_with("default/") {
                        let parts: Vec<&str> = slot_name.split('/').collect();
                        if parts.len() != 2 {
                            return None;
                        }

                        let default_name = parts[1];
                        let defn = self.default_slots.get(default_name)?;

                        // Use default_ prefix in the capture group to distinguish it
                        let group_name = format!("default_{}", default_name);

                        match defn {
                            SlotDefinition::Enumeration { values } => {
                                let alt = values.join("|");
                                regex_str.push_str(&format!("(?P<{}>{})", &group_name, alt));
                            }
                            SlotDefinition::CatchAll => {
                                regex_str.push_str(&format!("(?P<{}>.+?)", &group_name));
                            }
                            SlotDefinition::CatchProcess { .. } => {
                                regex_str.push_str(&format!("(?P<{}>.+?)", &group_name));
                            }
                        }
                    } else {
                        // Intent-specific slot
                        regex_str.push_str(&format!("(?P<{}>.+?)", slot_name));
                    }

                    current_pos += 1; // Skip closing brace
                }
            } else {
                // Regular character
                regex_str.push_str(&regex::escape(&chars[current_pos].to_string()));
                current_pos += 1;
            }
        }

        regex_str.push('$');

        RegexBuilder::new(&regex_str)
            .case_insensitive(true)
            .build()
            .ok()
    }
}
