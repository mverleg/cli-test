use ::std::path::PathBuf;

use ::clap::Parser;

#[derive(Parser, Debug)]
#[command()]
pub struct Args {
    /// Path of the file to test. By default searches all '*.clts' files
    #[arg()]
    pub path: Option<PathBuf>,
    /// How many levels of directories to recurse into, at most (for performance)
    #[arg(short = 'n', long, default_value="1000000", conflicts_with = "path")]
    pub max_depth: u32,
    /// Root directory within which to search for tests
    #[arg(short = 'r', long = "root", conflicts_with = "path")]
    pub roots: Vec<PathBuf>,
    /// Minimum number of tests expected. Set to 1 to fail when no tests are found
    #[arg(long, default_value = "0", conflicts_with = "path")]
    pub minimum_tests: u32,
}

#[test]
fn test_cli_args() {
    Args::try_parse_from(&["cli-test", "file.test", "-n", "2", "-r", "examples", "--root", "cli_tests", "--minimum_tests", "1"]).unwrap();
    Args::try_parse_from(&["cli-test"]).unwrap();
}
