mod version;
mod utils;
mod intent;

use std::error::Error;
use std::fs;
use crate::intent::intent_manager::IntentManager;
use crate::intent::recognizer::Recognizer;
use crate::utils::cli;

/*
Architecture
Todo:
   - Get Args
   - Create Init Actions
   - Add listener for Wake word
   - Add intent recognition
   - Skills Runner
   - Skill Interface And Tools

Protocols:
   - Skills -> Alex
   - Alex -> (MessageBus (Neon) <-> Avi Message Bus)
   - MessageBus -> (Neon (Audio | Speech) | GUI | Enclosure)
*/

fn create_sample_intent_files() -> Result<(), Box<dyn Error>> {
    let hotel_intent = r#"{
      "intent": "find_hotel",
      "patterns": [
        "find me a hotel in {default/locations}",
        "book hotel at {default/locations} for {date}"
      ],
      "regex_patterns": [
        "hotel (?P<location>\\w+) (?P<checkin>\\d{4}-\\d{2}-\\d{2}) to (?P<checkout>\\d{4}-\\d{2}-\\d{2})"
      ],
      "slots": {
        "date": ["2025-04-15", "2025-04-16"],
        "checkin": "*",
        "checkout": "*"
      }
    }"#;

    let flight_intent = r#"{
      "intent": "book_flight",
      "patterns": [
        "book a flight to {default/locations}",
        "find flights to {default/locations} for {date}"
      ],
      "regex_patterns": [
        "flight from (?P<origin>\\w+) to (?P<destination>\\w+)"
      ],
      "slots": {
        "date": ["2025-04-15", "2025-04-16"],
        "origin": "*",
        "destination": "*"
      }
    }"#;

    // Create directory if it doesn't exist
    fs::create_dir_all("intents")?;

    // Write files
    fs::write("intents/find_hotel.json", hotel_intent)?;
    fs::write("intents/book_flight.json", flight_intent)?;

    Ok(())
}

fn main() {
    cli::header();
    create_sample_intent_files().expect("TODO: panic message");
    let mut im = IntentManager::new();

    im.load_intent("intents/find_hotel.json").expect("TODO: panic message");
    im.load_intent("intents/book_flight.json").expect("TODO: panic message");

    let rec = Recognizer::new(&im);

    let examples = [
        "find me a hotel in Tokyo",
        "hotel Paris 2025-05-01 to 2025-05-03",
        "book hotel at london for 2025-04-15",
        "book me a flight from londom to Paris"
    ];

    for example in &examples {
        println!("User: {}", example);

        let matches = rec.recognize(example);
        if matches.is_empty() {
            println!("Sorry, I didn't understand.");
        } else {
            for m in matches {
                println!("→ Intent: {}", m.intent);
                println!("  Slots: {:?}", m.slots);
            }
        }
        println!();
    }
}
