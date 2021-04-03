use {
    crate::{
        build_system::{self, BuildSystem},
        config::AppConfig,
        error::TimError,
        test_api::{parser, FfiHandler, TestResult},
    },
    std::{
        collections::HashSet,
        env, fs,
        path::{Path, PathBuf},
        sync::{mpsc, Arc},
        thread,
        thread::JoinHandle,
    },
    threadpool::ThreadPool,
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

        // Run the build system
        let builder_thread = App::spawn_builder(
            self.config.build_system.unwrap(),
            self.config.project_path.clone(),
        );

        // Parse the test names
        let parser_thread = App::spawn_parser(self.config.tests);

        let _ = builder_thread.join().unwrap()?;
        let test_names = parser_thread.join().unwrap()?;

        let (tx, rx) = mpsc::channel();

        thread::spawn(move || App::run_tests(test_names, tx));

        while let Ok(test_res) = rx.recv() {
            let test_res = test_res?;
            let test_name = test_res.0;
            let test_res = test_res.1;
            if test_res.success {
                println!("[ + ] {} succeeded.", test_name);
            } else {
                println!("[ - ] {} failed.", test_name);
                if let Some(fname) = test_res.file {
                    println!("\tFile: {}", fname.into_string()?);
                }
                println!("\tLine: {}", test_res.line);
            }
            if let Some(msg) = test_res.msg {
                println!("\tMessage: {}", msg.into_string()?);
            }
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
                test_names.extend(parser::parse_test_names(test_path)?);
            }
            Ok(test_names)
        })
    }

    #[inline]
    fn run_tests(
        test_names: HashSet<String>,
        sender: mpsc::Sender<anyhow::Result<(String, TestResult)>>,
    ) {
        let ffi_handler = match unsafe { FfiHandler::load(PathBuf::from("tim-test-lib")) } {
            Ok(handler) => Arc::new(handler),
            Err(e) => {
                let _ = sender.send(Err(e));
                return;
            }
        };

        let pool = ThreadPool::new(4 /* TODO: Make this configurable */);

        for test in test_names {
            let sender_ = sender.clone();
            let ffi_handler_ = ffi_handler.clone();
            pool.execute(move || {
                let _ = sender_.send(unsafe { ffi_handler_.run(&test) }.map(|res| (test, res)));
            });
        }
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
