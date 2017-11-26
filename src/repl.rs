extern crate libc;

use std::ffi::CString;
use std::ffi::CStr;

#[link(name = "readline")]
extern {
    fn readline(prompt: *const libc::c_char) -> *const libc::c_char;
    fn add_history(entry: *const libc::c_char);
}

unsafe fn handle_input(input_pointer: *const i8) -> Option<String> {
    let utf8_bytes = CStr::from_ptr(input_pointer).to_bytes();
    let input = String::from_utf8(utf8_bytes.to_vec()).ok().unwrap();

    if input.len() > 0 {
        add_history(input_pointer);
    }

    Some(input)
}

fn ask(prompt: &str) -> Option<String> {
    let prompt_string = CString::new(prompt).unwrap();

    unsafe {
        let input_pointer = readline(prompt_string.as_ptr());
        if input_pointer.is_null() {
            None
        } else {
            handle_input(input_pointer)
        }

    }
}

pub fn start<F: Fn(String) -> Result<String, String>>(prompt: &str, f: F) {
    loop {
        match ask(prompt) {
            Some(input) => {
                if input.len() > 0 {
                    let result = f(input);
                    println!("{}", result.unwrap_or_else(|e| format!("Error: {}", e)));
                }
            },
            None => return
        };
    };
}
