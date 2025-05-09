use crate::intent::slot_extrator::ExtractedSlots;
use crate::utils::json::{dynamic_to_json, json_to_dynamic};
use rhai::packages::Package;
use rhai::{Dynamic, Engine, EvalAltResult, EvalContext, Expression, ImmutableString, Position};
use rhai_fs::FilesystemPackage;
use rhai_url::UrlPackage;
use serde_json::Value;

fn on_intent_syntax_handler(
    context: &mut EvalContext,
    inputs: &[Expression],
) -> Result<Dynamic, Box<EvalAltResult>> {
    let intent_name = inputs[0].get_string_value().unwrap().to_string();
    let block = &inputs[1];

    let i_name = context.scope().get_value::<ImmutableString>("INTENT_NAME");

    if i_name.is_some() && i_name.unwrap().eq_ignore_ascii_case(&intent_name) {
        let scope = context.scope_mut();
        scope.push_constant("name", intent_name);
        scope.push_constant(
            "intent",
            scope.get_value::<ExtractedSlots>("INTENT").unwrap(),
        );

        let _ = context.eval_expression_tree(block);
    }

    Ok(Dynamic::UNIT)
}

fn on_start_syntax_handler(
    context: &mut EvalContext,
    inputs: &[Expression],
) -> Result<Dynamic, Box<EvalAltResult>> {
    let block = &inputs[0];

    if context.scope().get_value::<bool>("STARTED").is_none() {
        let scope = context.scope_mut();
        scope.push_constant("STARTED", true);

        let _ = context.eval_expression_tree(block);
    }

    Ok(Dynamic::UNIT)
}

fn on_end_syntax_handler(
    context: &mut EvalContext,
    inputs: &[Expression],
) -> Result<Dynamic, Box<EvalAltResult>> {
    let block = &inputs[0];

    let e_name = context.scope().get_value::<bool>("END");

    if e_name.is_some() && e_name.unwrap() {
        let _ = context.eval_expression_tree(block);
    }

    Ok(Dynamic::UNIT)
}

pub fn register_json_functions(engine: &mut Engine) {
    engine
        .register_fn("parse_json", parse_json)
        .register_fn("to_json", to_json);
}

fn parse_json(json_str: &str) -> Result<Dynamic, Box<EvalAltResult>> {
    match serde_json::from_str::<Value>(json_str) {
        Ok(value) => Ok(json_to_dynamic(value)),
        Err(err) => Err(Box::new(EvalAltResult::ErrorRuntime(
            format!("JSON parse error: {}", err).into(),
            Position::NONE,
        ))),
    }
}

fn to_json(value: Dynamic) -> Result<String, Box<EvalAltResult>> {
    match dynamic_to_json(value) {
        Ok(json_value) => match serde_json::to_string_pretty(&json_value) {
            Ok(json_str) => Ok(json_str),
            Err(err) => Err(Box::new(EvalAltResult::ErrorRuntime(
                format!("JSON stringify error: {}", err).into(),
                Position::NONE,
            ))),
        },
        Err(err) => Err(Box::new(EvalAltResult::ErrorRuntime(
            err.to_string().parse().unwrap(),
            Position::NONE,
        ))),
    }
}

pub fn create_avi_script_engine(
    modules_register: fn(&mut Engine) -> Result<(), Box<EvalAltResult>>,
) -> Result<Engine, Box<dyn std::error::Error>> {
    let mut engine = Engine::new();

    engine.register_custom_syntax(
        ["on_intent", "$string$", "$block$"],
        false,
        on_intent_syntax_handler,
    )?;

    engine.register_custom_syntax(["on_start", "$block$"], false, on_start_syntax_handler)?;

    engine.register_custom_syntax(["on_end", "$block$"], false, on_end_syntax_handler)?;

    engine
        .register_custom_operator("or", 160)?
        .register_fn(
            "or",
            |a: Dynamic, b: Dynamic| {
                if a.to_string().is_empty() { b } else { a }
            },
        );

    engine
        .register_custom_operator("@@", 160)?
        .register_fn("@@", |a: ImmutableString, b: ImmutableString| {
            format!("{}{}", a, b)
        });

    modules_register(&mut engine).expect("A module did not load successfully!!");

    register_json_functions(&mut engine);

    engine
        .register_type_with_name::<ExtractedSlots>("Intent")
        .register_get("name", ExtractedSlots::get_name)
        .register_get("slots", ExtractedSlots::get_slots)
        .register_fn("get", ExtractedSlots::get)
        .register_fn("get_raw", ExtractedSlots::get_raw)
        .register_fn("require", ExtractedSlots::require)
        .register_fn("optional", ExtractedSlots::optional)
        .register_fn("exists", ExtractedSlots::exists)
        .register_fn("equal", ExtractedSlots::equal)
        .register_fn("in_list", ExtractedSlots::in_list)
        .register_fn("in_dict", ExtractedSlots::in_dict)
        .register_fn("obj", ExtractedSlots::obj)
        .register_fn("count", ExtractedSlots::count)
        .register_fn("all", ExtractedSlots::all)
        .register_fn("match_pattern", ExtractedSlots::match_pattern)
        .register_fn("is_type", ExtractedSlots::is_type);

    let fs = FilesystemPackage::new();
    fs.register_into_engine(&mut engine);

    let url = UrlPackage::new();
    url.register_into_engine(&mut engine);

    Ok(engine)
}
