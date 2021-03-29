use super::BuildSystem;

#[derive(Debug)]
pub struct Cmake;

impl BuildSystem for Cmake {
    fn config_name(&self) -> &'static str {
        "CMakeLists.txt"
    }
}
