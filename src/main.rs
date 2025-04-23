mod broker;
mod intent;
mod skills;
mod utils;
mod version;

use crate::broker::utils::start_mqtt;
use crate::intent::engine::IntentEngine;
use crate::intent::recognizer::Recognizer;
use crate::skills::manager::SkillManager;
use crate::utils::cli;
use crate::utils::cli::input;
/*
Protocols:
   - Skills -> Alex
   - Alex -> (MessageBus (Neon) <-> Avi Message Bus)
   - MessageBus -> (Neon (Audio | Speech) | GUI | Enclosure)
*/

fn main() {
    cli::header();
    let mut im = IntentEngine::new();

    let mqtt = start_mqtt();

    let mut skill_manager = SkillManager::new();

    skill_manager
        .load_skills_from_directory("skills", &mut im)
        .unwrap();

    let rec = Recognizer::new(&im);

    loop {
        let inp = input("Your prompt: ");
        let matches = rec.recognize(&inp);
        if matches.is_empty() {
            println!("Sorry, I didn't understand.");
        } else {
            for m in matches {
                skill_manager
                    .process_intent(m)
                    .expect("Error processing the intent!");
            }
        }
    }
}
