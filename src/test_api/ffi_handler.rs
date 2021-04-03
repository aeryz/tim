use {
    super::TestResult,
    libloading::Library,
    std::{
        ffi::CStr,
        os::raw::{c_char, c_uint, c_ulong},
        path::PathBuf,
        sync::Arc,
    },
};

pub struct FfiHandler {
    library: Library,
}

impl FfiHandler {
    pub unsafe fn load(path: PathBuf) -> anyhow::Result<Self> {
        Ok(FfiHandler {
            library: libloading::Library::new(path)?,
        })
    }

    pub unsafe fn run(self: Arc<Self>, test_name: String) -> anyhow::Result<TestResult> {
        Ok(self.library.get::<TestFnT>(test_name.as_bytes())?().into())
    }

    pub unsafe fn free(self: Arc<Self>, test_result: TestResult) -> anyhow::Result<()> {
        Ok(self.library.get::<FreeFnT>(b"tim_free")?(
            test_result.into(),
        ))
    }
}

type TestFnT = unsafe extern "C" fn() -> RawTestResult;
type FreeFnT = unsafe extern "C" fn(AllocatedPart) -> ();

#[repr(C)]
struct RawTestResult {
    file: *const c_char,
    msg: *const c_char,
    line: c_ulong,
    success: c_uint,
}

impl From<RawTestResult> for TestResult {
    fn from(raw_res: RawTestResult) -> Self {
        TestResult {
            file: if !raw_res.file.is_null() {
                // This is safe because the incoming string is definitely allocated,
                // otherwise it is NULL'ed.
                Some(unsafe { CStr::from_ptr(raw_res.file).into() })
            } else {
                None
            },
            msg: if !raw_res.msg.is_null() {
                // This is safe because the incoming string is definitely allocated,
                // otherwise it is NULL'ed.
                Some(unsafe { CStr::from_ptr(raw_res.msg).into() })
            } else {
                None
            },
            line: raw_res.line,
            success: if 0 == raw_res.success { false } else { true },
        }
    }
}

#[repr(C)]
struct AllocatedPart {
    file: *const c_char,
    msg: *const c_char,
}

impl Into<AllocatedPart> for TestResult {
    fn into(self) -> AllocatedPart {
        AllocatedPart {
            file: self.file.unwrap_or_default().as_ptr(),
            msg: self.msg.unwrap_or_default().as_ptr(),
        }
    }
}
