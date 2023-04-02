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
    /// Minimum number of tests expected. Set to 0 to succeed even if there are no tests
    #[arg(long, default_value = "1", conflicts_with = "path")]
    pub minimum_tests: u32,

    /// Copy the content of these directories to the test directory, as test data for the tests
    #[arg(short = 's', long)]
    pub source_dir: Vec<PathBuf>,
    //TODO @mark: impl
    /// Parent directory in which to place the test directories
    #[arg(long)]
    pub test_dir: Option<PathBuf>,
    //TODO @mark: impl
    /// Number of tests to run in parallel. Not always faster, since more test directories need to be maintained
    #[arg(short = 'P', long, default_value = "1", conflicts_with = "path")]
    pub parallel: u16,
    //TODO @mark: impl
}

#[test]
fn test_cli_args() {
    Args::try_parse_from(&["cli-test", "file.test", "-n", "2", "-r", "examples", "--root", "cli_tests", "--minimum_tests", "1"]).unwrap();
    Args::try_parse_from(&["cli-test"]).unwrap();
}
