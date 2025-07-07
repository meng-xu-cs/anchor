use std::path::{PathBuf, MAIN_SEPARATOR};

use walkdir::WalkDir;

use crate::config::PROJECT_PATH;

/// A single test case
pub struct TestCase {
    pub name: String,
    pub path: PathBuf,
}

/// List available test cases in the Anchor root directory
pub fn list() -> Vec<TestCase> {
    let mut cases = vec![];

    // discover the cases
    for entry in WalkDir::new(PROJECT_PATH.as_path()) {
        let entry = entry.unwrap_or_else(|e| {
            panic!(
                "unable to traverse root directory {}: {e}",
                PROJECT_PATH.display()
            )
        });
        if entry.file_name().to_str() == Some("Anchor.toml") {
            let path = entry
                .path()
                .parent()
                .unwrap_or_else(|| panic!("Anchor.toml should have a parent directory"))
                .to_path_buf();
            let name = path
                .strip_prefix(PROJECT_PATH.as_path())
                .unwrap_or_else(|e| {
                    panic!(
                        "test case {} does not start with prefix {}: {e}",
                        entry.path().display(),
                        PROJECT_PATH.display()
                    )
                })
                .to_str()
                .unwrap_or_else(|| panic!("non-ascii segment of testcase name"))
                .replace(MAIN_SEPARATOR, "-");

            cases.push(TestCase { name, path });
        }
    }

    // sort by name before returning
    cases.sort_by(|a, b| a.name.cmp(&b.name));
    cases
}
