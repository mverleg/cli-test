use ::std::process::ExitCode;
use ::clap::Parser;

use ::clitest::Args;
use ::clitest::cli_test;

fn main() -> ExitCode {
    env_logger::init_from_env(env_logger::Env::default().filter_or(env_logger::DEFAULT_FILTER_ENV, "warn"));
    let args = Args::parse();
    match cli_test(&args) {
        Ok(()) => ExitCode::from(0),
        Err(err) => {
            eprintln!("{err}");
            ExitCode::from(1)
        }
    }
}
