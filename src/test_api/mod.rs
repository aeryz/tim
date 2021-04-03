use std::ffi::CString;

mod ffi_handler;
pub mod parser;

pub struct TestResult {
    pub file: Option<CString>,
    pub msg: Option<CString>,
    pub line: u64,
    pub success: bool,
}

pub use ffi_handler::FfiHandler;
