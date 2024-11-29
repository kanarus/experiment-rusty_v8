mod inspector;

extern "C" fn handle_promise(
    hook_type: v8::PromiseHookType,
    promise: v8::Local<'_, v8::Promise>,
    value: v8::Local<'_, v8::Value>
) {
    match hook_type {
        v8::PromiseHookType::Init => {
            println!("[handle_promise] Init: promise = `{promise:?}` / value = `{value:?}`");
        }
        v8::PromiseHookType::Before => {
            println!("[handle_promise] Before: promise = `{promise:?}` / value = `{value:?}`");
        }
        v8::PromiseHookType::After => {
            println!("[handle_promise] After: promise = `{promise:?}` / value = `{value:?}`");
        }
        v8::PromiseHookType::Resolve => {
            println!("[handle_promise] Resolve: promise = `{promise:?}` / value = `{value:?}`");
        }
    }
}

fn main() {
    v8::V8::initialize_platform(v8::new_default_platform(0, false).make_shared());
    v8::V8::initialize();

    {
        let isolate = &mut v8::Isolate::new(v8::CreateParams::default());

        let inspector = &mut v8::inspector::V8Inspector::create(
            isolate,
            &mut self::inspector::InspectorClient::new()
        );

        let handle_scope = &mut v8::HandleScope::new(isolate);
        handle_scope.set_promise_hook(handle_promise);
        let context = v8::Context::new(handle_scope, Default::default());
        let scope = &mut v8::ContextScope::new(handle_scope, context);

        inspector.context_created(
            context,
            1,
            v8::inspector::StringView::empty(),
            v8::inspector::StringView::U8(v8::inspector::CharacterArray::from("aux_data".as_bytes()))
        );

         {
             let code = v8::String::new(scope, r#"
                 function wrapper(task) {
                     console.log('wrapper called');
                     task();
                 }
             "#).unwrap();
 
             let result = v8::Script::compile(scope, code, None)
                 .unwrap()
                 .run(scope)
                 .unwrap();
 
             println!("result = `{}`", result.to_rust_string_lossy(scope));
         }

        {
            let code = v8::String::new(scope, r#"
                console.log('Hello, ' + 'world!');

                wrapper(() => {
                    console.log('Hello from wrapper');
                });
            "#).unwrap();

            let result = v8::Script::compile(scope, code, None)
                .unwrap()
                .run(scope)
                .unwrap();

            println!("result = `{}`", result.to_rust_string_lossy(scope));
        }
    }

    unsafe {v8::V8::dispose()};
    v8::V8::dispose_platform();

    println!("everything is disposed");
}
