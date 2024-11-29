fn main() {
    v8::V8::initialize_platform(v8::new_default_platform(0, false).make_shared());
    v8::V8::initialize();

    let mut isolate = v8::Isolate::new(v8::CreateParams::default());
    let mut handle_scope = v8::HandleScope::new(&mut isolate);
    let context = v8::Context::new(&mut handle_scope, Default::default());
    let mut scope = v8::ContextScope::new(&mut handle_scope, context);

    let code = v8::String::new(&mut scope, r#"
        'Hello, ' + 'world!'
    "#).unwrap();
    let result = v8::Script::compile(&mut scope, code, None)
        .unwrap()
        .run(&mut scope)
        .unwrap();
    println!("result = `{}`", result.to_rust_string_lossy(&mut scope));

    drop(scope);
    drop(handle_scope);
    drop(isolate);
    unsafe {v8::V8::dispose()};
    v8::V8::dispose_platform();

    println!("everything is disposed");
}
