use rhai::{Dynamic, Engine, EvalAltResult, EvalContext, Expression, ImmutableString};
use rhai::packages::Package;
use rhai_fs::FilesystemPackage;
use rhai_url::UrlPackage;
use crate::intent::intent::Intent;

// TODO:
/*
-----------
on_intent "get_weather" |intent| {
    let city = intent.slots.get("city") ?? "your location";
    say("Let me check the weather in " + city + ".");
}
-----------

*/
fn on_intent_syntax_handler(
    context: &mut EvalContext,
    inputs: &[Expression]
) -> Result<Dynamic, Box<EvalAltResult>> {
    let intent_name = inputs[0].get_string_value().unwrap().to_string();
    let block = &inputs[1];

    if context.scope().get_value::<ImmutableString>("INTENT_NAME").unwrap().eq_ignore_ascii_case(&*intent_name) {
        let scope = context.scope_mut();
        scope.push_constant("name", intent_name);
        scope.push_constant("intent", scope.get_value::<Intent>("INTENT").unwrap());

        let _ = context.eval_expression_tree(block);
    }
    
    Ok(Dynamic::UNIT)
}
pub fn create_avi_script_engine(modules_register: fn(&mut Engine) -> Result<(), Box<EvalAltResult>>) -> Result<Engine, Box<dyn std::error::Error>> {
    let mut engine = Engine::new();

    engine.register_custom_syntax(
        ["whenis", "$expr$", "?", "$expr$", ":", "$expr$"],
        false,
        |context, inputs| match context.eval_expression_tree(&inputs[0])?.as_bool() {
            Ok(true) => context.eval_expression_tree(&inputs[1]),
            Ok(false) => context.eval_expression_tree(&inputs[2]),
            Err(typ) => Err(Box::new(EvalAltResult::ErrorMismatchDataType(
                "bool".to_string(),
                typ.to_string(),
                inputs[0].position(),
            ))),
        },
    )?;

    engine.register_custom_syntax(
        ["on_intent", "$string$", "$block$"],
        false,
        on_intent_syntax_handler
    )?;

    engine.register_custom_operator("or", 160)?
        .register_fn("or", |a: Dynamic, b: Dynamic| {
            if a.to_string().is_empty() {
                b
            } else {
                a
            }
        });

    engine.register_custom_operator("@@", 160)?
        .register_fn("@@", |a: ImmutableString, b: ImmutableString| format!("{}{}", a, b));

    modules_register(&mut engine).expect("A module did not load successfully!!");

    let fs = FilesystemPackage::new();
    fs.register_into_engine(&mut engine);

    let url = UrlPackage::new();
    url.register_into_engine(&mut engine);

    Ok(engine)
}