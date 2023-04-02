use crate::scan::find_cli_tests;

pub use self::cli::Args;

mod cli;
mod scan;

pub fn cli_test(args: &Args) -> Result<(), String> {
    assert!(!args.roots.is_empty());
    let pths = match args.path.as_ref() {
        Some(pth) => vec![pth.to_owned()],
        None => find_cli_tests(&args.roots, args.max_depth, args.minimum_tests)?,
    };
    todo!()
}
