use common::{Request, Response};
use std::slice;

extern "C" {
    /// This function is implemented on the host.
    /// Accepts a pointer to a string and its length.
    fn respond(ptr: *const u8, len: usize);
}

/// This function is called by the host.
/// It's tricky to return a tuple of a pointer and a length, so we'll just
/// call the host function with the pointer and length of the string.
#[no_mangle]
pub extern "C" fn hello_string_from_rust(ptr: i32, len: i32) {
    let slice = unsafe { slice::from_raw_parts(ptr as _, len as _) };
    let request = Request::from_json(slice);

    let out_str = Response {
        message: "Hello, this is a message from guest".to_string(),
        number: request.number + 1,
    }
    .to_json();

    unsafe {
        respond(out_str.as_ptr(), out_str.len());
    }
}
