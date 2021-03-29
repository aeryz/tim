use crate::TimError;

use super::BuildSystem;
use std::process::Command;

#[derive(Debug)]
pub struct Cmake;

impl BuildSystem for Cmake {
    fn config_name(&self) -> &'static str {
        "CMakeLists.txt"
    }

    fn build(&self, config_path: std::path::PathBuf) -> anyhow::Result<()> {
        let status = Command::new("cmake")
            .arg("-DTIM_TEST_BUILD=")
            .arg(config_path)
            .status()?;

        if !status.success() {
            return Err(TimError::BuildSystemError.into());
        }

        let status = Command::new("make").status()?;

        if !status.success() {
            return Err(TimError::BuildSystemError.into());
        }

        Ok(())
    }
}
