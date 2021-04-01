use std::sync::mpsc;

use {
    crate::{
        build_system::{self, BuildSystem},
        config::AppConfig,
        error::TimError,
        FfiHandler,
    },
    regex::Regex,
    std::{
        collections::HashSet,
        env, fs,
        fs::File,
        io::{BufRead, BufReader},
        path::{Path, PathBuf},
        thread,
        thread::JoinHandle,
    },
    walkdir::WalkDir,
};

pub struct App {
    config: AppConfig,
}

impl App {
    pub fn new() -> anyhow::Result<Self> {
        let app = App {
            config: AppConfig::parse_args()?,
        };

        if !app.config.working_dir.exists() {
            fs::create_dir(&app.config.working_dir)?;
        }

        env::set_current_dir(&app.config.working_dir)?;

        Ok(app)
    }

    pub fn run(mut self) -> anyhow::Result<()> {
        self.discover_project()?;

        let builder_thread = App::spawn_builder(
            self.config.build_system.unwrap(),
            self.config.project_path.clone(),
        );
        let parser_thread = App::spawn_parser(self.config.tests);

        let _ = builder_thread.join().unwrap()?;
        let test_names = parser_thread.join().unwrap()?;

        let (tx, rx) = mpsc::channel();
        let runner_thread = App::spawn_runner(test_names, tx);

        while let Ok(data) = rx
            .recv()
            .map_err(|err| TimError::UnexpectedError(err.into()))
        {
            // TODO: Do some stuff with the results
        }

        Ok(())
    }

    #[inline]
    fn spawn_builder(
        build_system: Box<dyn BuildSystem>,
        project_path: PathBuf,
    ) -> JoinHandle<anyhow::Result<()>> {
        thread::spawn(move || build_system.build(project_path))
    }

    #[inline]
    fn spawn_parser(test_paths: HashSet<PathBuf>) -> JoinHandle<anyhow::Result<HashSet<String>>> {
        thread::spawn(move || {
            let mut test_names = HashSet::new();
            for test_path in test_paths {
                test_names.extend(App::parse_test_names(test_path)?);
            }
            Ok(test_names)
        })
    }

    #[inline]
    fn spawn_runner(
        test_names: HashSet<String>,
        sender: mpsc::Sender<anyhow::Result<()>>,
    ) -> JoinHandle<()> {
        /*
         * spawn MAX_THREADS threads in a pool and run the tests, send the results via sender
         */
        unimplemented!()
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
