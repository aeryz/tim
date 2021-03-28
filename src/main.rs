use tim::build_system;
use tim::config::AppConfig;
use walkdir::WalkDir;

fn discover_project(config: &mut AppConfig) -> anyhow::Result<()> {
    let mut discover_bs = config.build_system.is_none();
    let discover_tests = config.tests.is_empty();

    if !discover_tests && !discover_bs {
        return Ok(());
    }

    for entry in WalkDir::new(&config.project_path) {
        let entry = entry?;
        let metadata = entry.metadata()?;

        if metadata.is_dir() {
            continue;
        } else {
            // TODO: Test postfix should be configurable
            let file_name = entry.file_name().to_str().unwrap();
            if discover_tests && file_name.ends_with("_test.c") {
                config
                    .tests
                    .insert(entry.path().to_str().unwrap().to_string());
            } else if discover_bs {
                if let Some(bs) = build_system::from_config(file_name) {
                    discover_bs = false;
                    config.build_system = Some(bs);
                }
            }
        }
    }
    Ok(())
}

fn main() -> anyhow::Result<()> {
    let mut config = AppConfig::parse_args()?;

    discover_project(&mut config)?;

    if config.tests.is_empty() {
        println!("No test is found. Aborting..");
    } else if config.build_system.is_none() {
        println!("Build system cannot be detected. Aborting..");
    } else {
        println!("TESTS: {:?}", config.tests);
        println!("BUILD SYSTEM: {:?}", config.build_system);
    }

    Ok(())
}
