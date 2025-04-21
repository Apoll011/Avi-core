mod version;
mod utils;
mod intent;
mod skills;

use crate::intent::engine::IntentEngine;
use crate::intent::recognizer::Recognizer;
use crate::skills::utils::load_skill;
use crate::utils::cli;
use crate::utils::cli::input;
/*
Architecture
Todo:
   - Get Args
   - Create Init Actions
   - Add listener for Wake word
   - Skill Interface And Tools

Protocols:
   - Skills -> Alex
   - Alex -> (MessageBus (Neon) <-> Avi Message Bus)
   - MessageBus -> (Neon (Audio | Speech) | GUI | Enclosure)
*/

fn main() {
    cli::header();
    let mut im = IntentEngine::new();

    let mut skill = load_skill("my_skill").expect("Failed to load skill");
    skill.load_intents(&mut im);
    skill.start();

    let rec = Recognizer::new(&im);


    loop {
        let inp = input("Your prompt: ");
        let matches = rec.recognize(&*inp);
        if matches.is_empty() {
            println!("Sorry, I didn't understand.");
        } else {
            for m in matches {
                skill.on_intent(m).expect("REASON");
            }
        }
    }
}
