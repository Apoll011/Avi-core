use rhai::{export_module, exported_module, Engine, EvalAltResult};
use rhai::plugin::*;

use uuid::Uuid;


#[export_module]
mod speak {
    pub fn say(key: &str, context: rhai::Map) {}
    pub fn text(message: &str) {}
    pub fn translated(key: &str, context: rhai::Map) {}

    pub fn voice(name: &str) {}
    pub fn repeat() {}
    pub fn pause(seconds: i64) {}
}

#[export_module]
mod ask {
    pub fn question(
        key: &str,
        callback: rhai::FnPtr,
        context: rhai::Map,
        expected: rhai::Dynamic,
    ) {}

    pub fn on_input(callback: rhai::FnPtr, expected: rhai::Dynamic) {}

    pub fn confirm(callback: rhai::FnPtr) {}
    pub fn cancel(callback: rhai::FnPtr) {}
    pub fn number_input(prompt: &str, callback: rhai::FnPtr) {}
}

#[export_module]
mod intent {
    pub fn get(slot: &str) -> rhai::Dynamic { ().into() }
    pub fn get_raw(slot: &str) -> rhai::Dynamic { ().into() }
    pub fn require(slot: &str) -> rhai::Dynamic { ().into() }
    pub fn optional(slot: &str) -> rhai::Dynamic { ().into() }

    pub fn exists(slot1: &str, slot2: &str) -> bool { false }
    pub fn equal(slot: &str, value: &str) -> bool { false }

    pub fn in_list(slot: &str, list: rhai::Array) -> bool { false }
    pub fn in_dict(slot: &str, map: rhai::Map) -> bool { false }

    pub fn obj(slot: &str) -> rhai::Dynamic { ().into() }
    pub fn count() -> i64 { 0 }
    pub fn all() -> rhai::Map { rhai::Map::new() }

    pub fn match_pattern(slot: &str, pattern: &str) -> bool { false }
    pub fn is_type(slot: &str, type_name: &str) -> bool { false }
}

#[export_module]
mod assets {
    pub fn get(file: &str) -> String { "".into() }
    pub fn exists(file: &str) -> bool { false }

    pub fn read_text(file: &str) -> String { "".into() }
    pub fn read_json(file: &str) -> rhai::Map { rhai::Map::new() }

    pub fn image(file: &str) -> rhai::Dynamic { ().into() }
}

#[export_module]
mod audio {
    pub fn play(file: &str) {}
    pub fn stop() {}
    pub fn is_playing() -> bool { false }

    pub fn volume(level: i64) {}
    pub fn mute() {}
    pub fn unmute() {}
}

#[export_module]
mod context {
    pub fn save(name: &str, value: rhai::Dynamic) {}
    pub fn load(name: &str) -> rhai::Dynamic { ().into() }
    pub fn clear(name: &str) {}
}

#[export_module]
mod config {
    pub fn get(name: &str) -> rhai::Dynamic { ().into() }
    pub fn set(name: &str, value: rhai::Dynamic) {}

    pub fn has(name: &str) -> bool { false }
    pub fn type_of(name: &str) -> String { "".into() }
}

#[export_module]
mod setting {
    pub fn get(name: &str) -> rhai::Dynamic { ().into() }
    pub fn set(name: &str, value: rhai::Dynamic) {}

    pub fn save() {}
    pub fn list() -> rhai::Array { rhai::Array::new() }
}

#[export_module]
mod http {
    pub fn call(route: &str, method: &str, params: rhai::Map) -> rhai::Dynamic { ().into() }

    pub fn get(route: &str, params: rhai::Map) -> rhai::Dynamic { ().into() }
    pub fn post(route: &str, body: rhai::Map) -> rhai::Dynamic { ().into() }

    pub fn status() -> i64 { 200 }
}

#[export_module]
mod events {
    pub fn emit(name: &str, payload: rhai::Map) {}

    pub fn listen(name: &str, callback: rhai::FnPtr) {}
}

#[export_module]
mod utils {
    pub fn uuid() -> String { Uuid::new_v4().into() }
}

#[export_module]
mod translation {
    /// Get a translation by key.
    /// Optionally provide a context map to fill placeholders.
    ///
    /// translation.get("greet_user", #{ "user": "Alex" })
    pub fn get(key: &str, context: rhai::Map) -> String {
        // Placeholder logic; real implementation would call translation system.
        let mut result = format!("Translated: {}", key);
        for (k, v) in context.iter() {
            result = result.replace(&format!("{{{}}}", k), &v.to_string());
        }
        result
    }

    /// Get a translation with no formatting.
    pub fn get_raw(key: &str) -> String {
        format!("Translated: {}", key)
    }

    /// Check if a translation key exists.
    pub fn exists(key: &str) -> bool {
        // Example hardcoded check
        key == "hello" || key == "bye" || key == "name_prompt"
    }

    /// Get a translation or fallback if not found.
    pub fn get_or(key: &str, fallback: &str) -> String {
        if exists(key) {
            get_raw(key)
        } else {
            fallback.to_string()
        }
    }

    /// Replace placeholders in an already translated string.
    /// Useful for post-formatting custom values.
    pub fn format_with_placeholders(base: &str, context: rhai::Map) -> String {
        let mut result = base.to_string();
        for (k, v) in context.iter() {
            result = result.replace(&format!("{{{}}}", k), &v.to_string());
        }
        result
    }
}


pub fn register_modules(engine: &mut Engine) -> Result<(), Box<EvalAltResult>> {
    engine.register_static_module("speak", exported_module!(speak).into())
        .register_static_module("ask", exported_module!(ask).into())
        .register_static_module("intent", exported_module!(intent).into())
        .register_static_module("assets", exported_module!(assets).into())
        .register_static_module("audio", exported_module!(audio).into())
        .register_static_module("context", exported_module!(context).into())
        .register_static_module("config", exported_module!(config).into())
        .register_static_module("setting", exported_module!(setting).into())
        .register_static_module("events", exported_module!(events).into())
        .register_static_module("translation", exported_module!(translation).into())
        .register_static_module("utils", exported_module!(utils).into())
        .register_static_module("http", exported_module!(http).into());

    Ok(())
}