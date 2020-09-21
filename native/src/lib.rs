use neon::prelude::*;

fn hello(mut cx: FunctionContext) -> JsResult<JsString> {
    let name = cx.argument::<JsString>(0)?.value();
    let result = cx.string(hello_impl(&name));
    Ok(result)
}

fn hello_impl(name: &str) -> String {
    format!("hello {}!", name)
}

register_module!(mut cx, {
    cx.export_function("hello", hello)
});

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hello() {
        println!("{}", hello_impl("Jackie"))
    }
}
