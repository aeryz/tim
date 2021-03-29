use crate::{build_system, config::AppConfig, error::TimError};
use walkdir::WalkDir;

pub struct App {
    config: AppConfig,
}

impl App {
    pub fn new() -> anyhow::Result<Self> {
        Ok(App {
            config: AppConfig::parse_args()?,
        })
    }

    pub fn run(mut self) -> anyhow::Result<()> {
        let _ = self.discover_project()?;

        let build_system = self.config.build_system.unwrap();
        println!("Build system: {:?}", build_system);

        Ok(())
    }

    fn discover_project(&mut self) -> anyhow::Result<()> {
        let mut discover_bs = self.config.build_system.is_none();
        let discover_tests = self.config.tests.is_empty();

        let mut found_tests = self.config.tests.len();

        if !discover_tests {
            for exc in &self.config.excludes {
                self.config.tests.remove(exc);
            }
        }

        for entry in WalkDir::new(&self.config.project_path) {
            let entry = entry?;
            let metadata = entry.metadata()?;

            if metadata.is_dir() {
                continue;
            } else {
                // TODO: Test postfix should be configurable
                let file_name = entry.file_name().to_str().unwrap();

                if discover_tests
                    && file_name.ends_with("_test.c")
                    && !self.config.excludes.contains(file_name)
                {
                    self.config
                        .tests
                        .insert(entry.path().to_str().unwrap().to_string());
                } else if found_tests != 0 && !discover_tests && file_name.ends_with("_test.c") {
                    if self.config.tests.remove(file_name) {
                        found_tests -= 1;
                        self.config
                            .tests
                            .insert(entry.path().to_str().unwrap().to_string());
                    }
                }

                if discover_bs {
                    if let Some(bs) = build_system::from_config(file_name) {
                        discover_bs = false;
                        self.config.build_system = Some(bs);
                    }
                }
            }
        }

        if self.config.build_system.is_none() {
            Err(TimError::BuildSystemCannotBeFound.into())
        } else {
            Ok(())
        }
    }
}
