use {
    crate::{build_system, config::AppConfig, error::TimError},
    regex::Regex,
    std::{
        collections::HashSet,
        env, fs,
        fs::File,
        io::{BufRead, BufReader},
        path::{Path, PathBuf},
        sync::{mpsc, Arc, Mutex},
        thread,
    },
    walkdir::WalkDir,
};

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
        if !self.config.working_dir.exists() {
            fs::create_dir(&self.config.working_dir)?;
        }

        env::set_current_dir(&self.config.working_dir)?;

        self.discover_project()?;

        let build_system = self.config.build_system.unwrap();
        println!("Build system: {:?}", build_system);
        println!("Tests: {:?}", self.config.tests);

        let test_fns = Arc::new(Mutex::new(HashSet::new()));

        let (tx, rx) = mpsc::channel();

        {
            let project_path = self.config.project_path;
            let tx_ = tx.clone();
            thread::spawn(move || {
                if let Err(err) = build_system.build(project_path) {
                    tx_.send(err).unwrap();
                }
            });
        }

        {
            let test_fns_ = test_fns.clone();
            let tests = self.config.tests;
            thread::spawn(move || {
                for test in tests {
                    let test_names = match Self::parse_test_names(test) {
                        Ok(names) => names,
                        Err(err) => {
                            tx.send(err).unwrap();
                            break;
                        }
                    };
                    test_fns_.lock().unwrap().extend(test_names);
                }
            });
        }

        if let Ok(err) = rx.recv() {
            Err(err)
        } else {
            println!("Tests: {:?}", test_fns);
            Ok(())
        }
    }

    fn parse_test_names(file_path: PathBuf) -> anyhow::Result<HashSet<String>> {
        let mut ret = HashSet::new();
        let file = File::open(file_path)?;
        for line in BufReader::new(file).lines() {
            let line = line?;
            let line = line.trim_start();
            if line.chars().next() != Some('T') {
                continue;
            }
            if let Some(captures) = Regex::new(r"TIM_TEST *\((\w+)\).*")?.captures(&line) {
                if captures.len() > 1 {
                    ret.insert(captures.get(1).unwrap().as_str().to_string());
                }
            }
        }
        Ok(ret)
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

        println!("Discover tests: {}", discover_tests);

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
                    && !self.config.excludes.contains(Path::new(file_name))
                {
                    println!("?????");
                    self.config.tests.insert(entry.path().to_owned());
                } else if found_tests != 0 && !discover_tests && file_name.ends_with("_test.c") {
                    if self.config.tests.remove(Path::new(file_name)) {
                        found_tests -= 1;
                        self.config.tests.insert(entry.path().to_owned());
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
