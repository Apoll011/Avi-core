use crate::intent::slot::SlotDefinition;
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Debug, Deserialize)]
pub struct IntentFile {
    pub(crate) intent: String,
    #[serde(default)]
    pub(crate) patterns: Vec<String>,
    #[serde(default)]
    pub(crate) regex_patterns: Vec<String>,
    pub(crate) slots: HashMap<String, serde_json::Value>,
}

#[derive(Clone)]
pub struct Intent {
    pub(crate) name: String,
    pub(crate) patterns: Vec<String>,
    pub(crate) regex_patterns: Vec<String>,
    pub(crate) slots: HashMap<String, SlotDefinition>,
}
