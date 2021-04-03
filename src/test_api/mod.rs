use std::os::raw::c_char;

mod ffi_handler;
pub mod parser;

pub struct TestResult {
    pub file: *const c_char,
    pub msg: *const c_char,
    pub line: u64,
    pub success: bool,
}

unsafe impl Send for TestResult {}

pub use ffi_handler::FfiHandler;
