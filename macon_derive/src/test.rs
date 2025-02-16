pub mod data {
    use std::{fs::File, io::BufReader, path::{Path, PathBuf}};

    use serde::de::DeserializeOwned;

    pub fn path(testcase: &str, filename: &str) -> PathBuf {
        [
            "tests",
            testcase,
            filename,
        ]
        .into_iter()
        .collect()
    }

    pub fn load_yaml<T: DeserializeOwned>(testcase: &str, basename: &str) -> T {
        let filename = format!("{}.yaml", basename);
        let filepath = path(testcase, &filename);
        let file = File::open(&filepath)
            .expect(&format!("unable to open file {:?} for testcase {:?} ({:?})", filename, testcase, filepath));
        let reader = BufReader::new(file);
        serde_yaml::from_reader(reader)
            .expect(&format!("unable to load YAML for file {:?} for testcase {:?} ({:?})", filename, testcase, filepath))
    }
}
