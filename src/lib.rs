use ::log::debug;

use crate::parse::CliTest;
use crate::scan::find_cli_tests;

pub use self::cli::Args;

mod cli;
mod scan;
mod parse;

pub fn cli_test(args: &Args) -> Result<(), String> {
    assert!(!args.roots.is_empty());
    let tests = collect_tests(args)?;
    todo!()
}

fn collect_tests(args: &Args) -> Result<Vec<CliTest>, String> {
    let paths = match args.path.as_ref() {
        Some(pth) => {
            debug!("requested cli-test for file {}, not scanning for more tests", pth.to_string_lossy());
            vec![pth.to_owned()]
        },
        None => find_cli_tests(&args.roots, args.max_depth, args.minimum_tests)?,
    };
    let mut tests = Vec::with_capacity(paths.len());
    for path in paths {
        tests.push(CliTest {
            path,
        })
    }
    Ok(tests)
}
