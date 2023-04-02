use ::std::path::PathBuf;

use ::walkdir::WalkDir;

pub use self::cli::Args;

mod cli;

pub fn cli_test(args: &Args) -> Result<(), String> {
    let pths = match args.path.as_ref() {
        Some(pth) => vec![pth.to_owned()],
        None => find_cli_tests(PathBuf::from("."))?,
    };
    todo!()
}

fn find_cli_tests(root: PathBuf) -> Result<Vec<PathBuf>, String> {
    let walker = WalkDir::new(root)
        .max_depth(128)
        .min_depth(1)
        .follow_links(true);
    let results = Vec::new();
    for entry_res in walker.into_iter() {
        let entry = entry_res.map_err(|err| format!("count not scan for test files, err: {err}"))?;
        if let Some(name) = entry.file_name() {
            if name.ends_with(".clts") {
                todo!()
            }
        }
    }
    Ok(results)
}
