use {
    regex::Regex,
    std::{
        collections::HashSet,
        fs::File,
        io::{BufRead, BufReader},
        path::PathBuf,
    },
};

pub fn parse_test_names(file_path: PathBuf) -> anyhow::Result<HashSet<String>> {
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
