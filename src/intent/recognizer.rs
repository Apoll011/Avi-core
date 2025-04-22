use crate::intent::engine::IntentEngine;
use crate::intent::slot_extrator::{ExtractedSlots, SlotExtractor};

pub struct Recognizer<'a> {
    intent_manager: &'a IntentEngine,
    slot_extractor: SlotExtractor<'a>,
}

impl<'a> Recognizer<'a> {
    pub(crate) fn new(intent_manager: &'a IntentEngine) -> Self {
        Recognizer {
            intent_manager,
            slot_extractor: SlotExtractor::new(&intent_manager.default_slots),
        }
    }

    pub(crate) fn recognize(&self, text: &str) -> Vec<ExtractedSlots> {
        let mut results = Vec::new();

        for intent in &self.intent_manager.intents {
            // 1) Try plain patterns
            for pat in &intent.patterns {
                if let Some(slots) =
                    self.slot_extractor
                        .extract_from_pattern(pat, text, &intent.name, &intent.slots)
                {
                    results.push(slots);
                }
            }

            // 2) Try regex patterns
            for rx in &intent.regex_patterns {
                if let Some(slots) =
                    self.slot_extractor
                        .extract_from_regex(rx, text, &intent.name, &intent.slots)
                {
                    results.push(slots);
                }
            }
        }

        results
    }
}
