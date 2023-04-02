#![feature(variant_count)]
#![feature(lazy_cell)]

use ::std::fs::read_to_string;

use ::log::debug;

use crate::parse::CliTest;
use crate::scan::find_cli_tests;

pub use self::cli::Args;

mod common;
mod cli;
mod scan;
mod parse;

pub fn cli_test(args: &Args) -> Result<(), String> {
    assert!(!args.roots.is_empty());
    let tests = collect_tests(args)?;
    todo!("run")
}

fn collect_tests(args: &Args) -> Result<Vec<CliTest>, String> {
    if 1==1 { return Err("cli-test is under development and not ready for use yet".to_owned()) }
    let paths = match args.path.as_ref() {
        Some(pth) => {
            debug!("requested cli-test for file {}, not scanning for more tests", pth.to_string_lossy());
            vec![pth.to_owned()]
        },
        None => find_cli_tests(&args.roots, args.max_depth, args.minimum_tests)?,
    };
    let mut tests = Vec::with_capacity(paths.len());
    for path in paths {
        let content = read_to_string(&path)
            .map_err(|err| format!("could not read cli-test at '{}', err {err}", path.to_string_lossy()))?;
        tests.push(CliTest::parse(&content, &path)?)
    }
    Ok(tests)
}
