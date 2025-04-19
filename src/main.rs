mod version;
mod utils;
mod intent;
mod skills;

use crate::intent::engine::IntentEngine;
use crate::intent::intent::Intent;
use crate::intent::recognizer::Recognizer;
use crate::skills::avi_script::avi_engine::{get_avi_script_engine};
use crate::skills::skill::Skill;
use crate::skills::utils::load_skill;
use crate::utils::cli;
use crate::utils::cli::input;
/*
Architecture
Todo:
   - Get Args
   - Create Init Actions
   - Add listener for Wake word
   - Skills Runner
   - Skill Interface And Tools

Protocols:
   - Skills -> Alex
   - Alex -> (MessageBus (Neon) <-> Avi Message Bus)
   - MessageBus -> (Neon (Audio | Speech) | GUI | Enclosure)
*/

fn main() {
    cli::header();
    let mut im = IntentEngine::new();

    im.load_intent("intents/find_hotel.json").expect("TODO: panic message");

    let rec = Recognizer::new(&im);

    /*
    im.load_intent("intents/book_flight.json").expect("TODO: panic message");


    loop {
        let inp = input("Your prompt: ");
        let matches = rec.recognize(&*inp);
        if matches.is_empty() {
            println!("Sorry, I didn't understand.");
        } else {
            for m in matches {
                println!("→ Intent: {}", m.intent);
                println!("  Slots: {:?}", m.slots);
            }
        }
    }*/

    let engine = get_avi_script_engine().unwrap();

    let mut skill = load_skill("my_skill").expect("Failed to load skill");
    
    skill.on_intent(rec.recognize("find me a hotel in paris")[0].clone(), &engine).expect("REASON")
}
