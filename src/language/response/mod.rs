use serde::Deserialize;
use std::collections::HashMap;
use std::error::Error;
use std::fs;
use std::path::Path;

#[derive(Clone, Debug, Deserialize)]
pub struct Response {
    pub(crate) id: String,
    #[serde(default)]
    pub(crate) response: HashMap<String, Vec<String>>,
}

impl Response {
    pub fn is_valid(&self) -> bool {
        !self.response.is_empty() && !self.id.is_empty()
    }
}

pub(crate) fn get_response(
    skill_path: String,
    id: String,
    lang: String,
) -> Result<String, Box<dyn Error>> {
    let file_path = Path::new(&skill_path).join(Path::new(&id).file_name().unwrap());
    let content = fs::read_to_string(file_path)?;
    let data: Response = serde_json::from_str(&content)?;

    if !data.is_valid() {
        return Err("Intent must have at least one pattern or regex pattern".into());
    }

    Ok(data.response[&lang][0].clone())
}
