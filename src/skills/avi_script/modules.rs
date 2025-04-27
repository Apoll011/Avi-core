use rhai::module_resolvers::{FileModuleResolver, ModuleResolversCollection, StaticModuleResolver};
use rhai::plugin::*;
use rhai::{Engine, EvalAltResult, ImmutableString, export_module, exported_module};

use uuid::Uuid;

use std::time::Instant;

use crate::skills::avi_script::avi_librarymanager::initialize_rhai_library;

#[export_module]
mod speak {
    pub fn say(key: &str, context: rhai::Map) {}
    pub fn text(message: &str) {}
    pub fn translated(key: &str, context: rhai::Map) {}
}

#[export_module]
mod ask {
    pub fn question(key: &str, callback: rhai::FnPtr, context: rhai::Map, expected: rhai::Dynamic) {
    }

    pub fn on_input(callback: rhai::FnPtr, expected: rhai::Dynamic) {}

    pub fn confirm(callback: rhai::FnPtr) {}
    pub fn cancel(callback: rhai::FnPtr) {}
    pub fn number_input(prompt: &str, callback: rhai::FnPtr) {}
}

#[export_module]
mod assets {
    pub fn get(file: &str) -> String {
        "".into()
    }
    pub fn exists(file: &str) -> bool {
        false
    }

    pub fn read_text(file: &str) -> String {
        "".into()
    }
    pub fn read_json(file: &str) -> rhai::Map {
        rhai::Map::new()
    }

    pub mod audio {
        pub fn play(file: &str) {}
        pub fn stop() {}
        pub fn is_playing() -> bool {
            false
        }

        pub fn volume(level: i64) {}
        pub fn mute() {}
        pub fn unmute() {}
    }
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

#[export_module]
mod context {
    pub fn save(name: &str, value: rhai::Dynamic) {}
    pub fn load(name: &str) -> rhai::Dynamic {
        ().into()
    }
    pub fn clear(name: &str) {}
}

#[export_module]
mod http {
    pub fn call(route: &str, method: &str, params: rhai::Map) -> rhai::Dynamic {
        ().into()
    }

    pub fn get(route: &str, params: rhai::Map) -> rhai::Dynamic {
        ().into()
    }
    pub fn post(route: &str, body: rhai::Map) -> rhai::Dynamic {
        ().into()
    }

    pub fn status() -> i64 {
        200
    }
}

#[export_module]
mod events {
    pub fn emit(name: &str, payload: rhai::Map) {}

    pub fn listen(name: &str, callback: rhai::FnPtr) {}
}

#[export_module]
mod utils {
    #[rhai_fn(volatile)]
    pub fn uuid() -> String {
        Uuid::new_v4().into()
    }

    pub fn env_var(key: &str) -> String {
        std::env::var(key).unwrap_or_else(|_| String::new())
    }

    pub fn env_os() -> String {
        if cfg!(target_os = "windows") {
            "windows".to_string()
        } else if cfg!(target_os = "linux") {
            "linux".to_string()
        } else {
            "unknown".to_string()
        }
    }

    pub fn trim(s: &str) -> String {
        s.trim().into()
    }

    pub fn lowercase(s: &str) -> String {
        s.to_lowercase()
    }

    pub fn uppercase(s: &str) -> String {
        s.to_uppercase()
    }

    pub fn starts_with(a: &str, b: &str) -> bool {
        a.starts_with(b)
    }

    pub fn ends_with(a: &str, b: &str) -> bool {
        a.ends_with(b)
    }

    pub fn sleep(ms: i64) {
        std::thread::sleep(std::time::Duration::from_millis(ms as u64));
    }

    pub fn now() -> i64 {
        let start = Instant::now();
        start.elapsed().as_millis() as i64
    }
}

pub fn register_modules(engine: &mut Engine) -> Result<(), Box<EvalAltResult>> {
    let mut resolvers = ModuleResolversCollection::new();

    let mut static_resolver = StaticModuleResolver::new();
    static_resolver.insert("http", exported_module!(http));
    static_resolver.insert("speak", exported_module!(speak));
    static_resolver.insert("ask", exported_module!(ask));
    static_resolver.insert("events", exported_module!(events));
    static_resolver.insert("context", exported_module!(context));
    static_resolver.insert("translation", exported_module!(translation));
    static_resolver.insert("assets", exported_module!(assets));
    let file_resolver = FileModuleResolver::new_with_extension("avi");

    let lib_manager = initialize_rhai_library().unwrap();

    let lib_resolver =
        FileModuleResolver::new_with_path_and_extension(lib_manager.library_dir(), "avi");

    resolvers += file_resolver;
    resolvers += static_resolver;
    resolvers += lib_resolver;

    engine.set_module_resolver(resolvers);

    engine.register_global_module(exported_module!(utils).into());

    Ok(())
}
