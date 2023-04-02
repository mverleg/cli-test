use ::std::path::PathBuf;

use ::clap::Parser;
use ::clap::Subcommand;
use ::env_logger;

#[derive(Parser, Debug)]
#[command()]
pub struct Args {
    /// Path of the file to test. By default searches all '*.clts' files
    #[arg()]
    pub path: Option<PathBuf>,
}

#[test]
fn test_cli_args() {
    Args::try_parse_from(&["cli-test", "file.test"]).unwrap();
    Args::try_parse_from(&["cli-test"]).unwrap();
}
