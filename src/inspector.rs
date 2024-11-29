pub struct InspectorClient(v8::inspector::V8InspectorClientBase);

impl InspectorClient {
    pub fn new() -> Self {
        Self(v8::inspector::V8InspectorClientBase::new::<Self>())
    }
}

impl v8::inspector::V8InspectorClientImpl for InspectorClient {
    // required
    fn base(&self) -> &v8::inspector::V8InspectorClientBase {
        &self.0
    }

    // required
    fn base_mut(&mut self) -> &mut v8::inspector::V8InspectorClientBase {
        &mut self.0
    }

    // required
    unsafe fn base_ptr(this: *const Self) -> *const v8::inspector::V8InspectorClientBase
    where Self: Sized {
        &(&*this).0
    }

    fn console_api_message(
        &mut self,
        _context_group_id: i32,
        _level: i32,
        message: &v8::inspector::StringView,
        _url: &v8::inspector::StringView,
        _line_number: u32,
        _column_number: u32,
        _stack_trace: &mut v8::inspector::V8StackTrace,
    ) {
        println!("[console_api_message] {message}");
    }
}
