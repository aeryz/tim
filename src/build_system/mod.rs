mod cmake;

pub use cmake::Cmake;

#[derive(Debug)]
pub enum BuildSystemType {
    Cmake,
}

pub trait BuildSystem: std::fmt::Debug {
    fn config_name(&self) -> &'static str;
}

#[inline]
pub fn from_config(config_name: &str) -> Option<Box<dyn BuildSystem>> {
    match config_name {
        "CMakeLists.txt" => Some(Box::new(Cmake)),
        _ => None,
    }
}

impl From<&str> for Box<dyn BuildSystem> {
    fn from(value: &str) -> Box<dyn BuildSystem> {
        match value {
            "cmake" => Box::new(Cmake),
            _ => panic!("Unexpected build system."),
        }
    }
}
