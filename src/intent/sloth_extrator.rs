use std::collections::HashMap;
use regex::{Regex, RegexBuilder};

use crate::intent::sloth::{DefaultSlotManager, SlotDefinition};

#[derive(Debug, Clone)]
pub struct ExtractedSlots {
    pub(crate) intent: String,
    pub(crate) slots: HashMap<String, String>,
}

pub struct SlotExtractor<'a> {
    default_slots: &'a DefaultSlotManager,
}

impl<'a> SlotExtractor<'a> {
    pub(crate) fn new(default_slots: &'a DefaultSlotManager) -> Self {
        SlotExtractor { default_slots }
    }

    fn validate_and_process_slot(
        &self,
        val_text: String,
        defn: &SlotDefinition,
    ) -> Option<String> {
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
                if processed.is_none() {
                    return None;
                }
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
                    if processed.is_none() {
                        return None;
                    }
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

    fn pattern_to_regex(
        &self,
        pattern: &str,
    ) -> Option<Regex> {
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
                                regex_str.push_str(&format!("(?P<{}>{})",&group_name, alt));
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