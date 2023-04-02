use ::clap::Parser;

use ::clitest::Args;
use ::clitest::cli_test;

fn main() {
    env_logger::init_from_env(env_logger::Env::default().filter_or(env_logger::DEFAULT_FILTER_ENV, "warn"));
    let args = Args::parse();
    cli_test(&args);
}
