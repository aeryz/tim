use crate::{build_system::BuildSystem, TimError};
use clap::{load_yaml, App};
use std::{collections::HashSet, env, path::PathBuf};

#[derive(Debug)]
pub struct AppConfig {
    pub build_system: Option<Box<dyn BuildSystem>>,
    pub project_path: PathBuf,
    pub working_dir: PathBuf,
    pub tests: HashSet<PathBuf>,
    pub excludes: HashSet<PathBuf>,
}

fn exists(path: &str) -> anyhow::Result<PathBuf> {
    let buf = PathBuf::from(path);
    if buf.exists() {
        Ok(buf)
    } else {
        Err(TimError::PathDoesNotExits(buf).into())
    }
}

impl AppConfig {
    pub fn parse_args() -> anyhow::Result<Self> {
        let yaml = load_yaml!("cli_args.yaml");
        let m = App::from_yaml(yaml).get_matches();

        Ok(AppConfig {
            build_system: m.value_of("build_system").map(From::from),
            project_path: exists(m.value_of("project_path").unwrap().into())?,
            working_dir: match m.value_of("working_dir") {
                Some(wd) => exists(wd)?,
                None => format!("{}/build", env::current_dir()?.to_str().unwrap()).into(),
            },
            tests: match m.values_of("tests") {
                Some(tests) => tests.map(PathBuf::from).collect(),
                None => HashSet::new(),
            },
            excludes: match m.values_of("excludes") {
                Some(excludes) => excludes.map(PathBuf::from).collect(),
                None => HashSet::new(),
            },
        })
    }
}
