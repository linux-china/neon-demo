use neon::prelude::*;

fn hello(mut cx: FunctionContext) -> JsResult<JsString> {
    let name = cx.argument::<JsString>(0)?.value();
    let result = cx.string(format!("hello {}!", name));
    Ok(result)
}

register_module!(mut cx, {
    cx.export_function("hello", hello)
});
