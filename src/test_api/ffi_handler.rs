use {
    super::TestResult,
    libloading::Library,
    std::{
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

    pub unsafe fn run(self: Arc<Self>, test_name: &str) -> anyhow::Result<TestResult> {
        Ok(self.library.get::<TestFnT>(test_name.as_bytes())?().into())
    }

    pub unsafe fn free(self: &Arc<Self>, test_result: TestResult) -> anyhow::Result<()> {
        if !test_result.msg.is_null() {
            Ok(self.library.get::<FreeFnT>(b"tim_free")?(test_result.msg))
        } else {
            Ok(())
        }
    }
}

type TestFnT = unsafe extern "C" fn() -> RawTestResult;
type FreeFnT = unsafe extern "C" fn(*const c_char) -> ();

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
            file: raw_res.file,
            msg: raw_res.msg,
            line: raw_res.line,
            success: if 0 == raw_res.success { false } else { true },
        }
    }
}
