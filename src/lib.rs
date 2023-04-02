use ::std::path::PathBuf;
use std::collections::HashSet;

use ::walkdir::WalkDir;
use log::debug;

pub use self::cli::Args;

mod cli;

pub fn cli_test(args: &Args) -> Result<(), String> {
    let pths = match args.path.as_ref() {
        Some(pth) => vec![pth.to_owned()],
        None => find_cli_tests(PathBuf::from("."))?,
    };
    todo!()
}

fn find_cli_tests(roots: &[PathBuf], max_depth: u32) -> Result<Vec<PathBuf>, String> {
    let mut results = Vec::new();
    let mut seen = HashSet::new();
    for root in roots {
        let walker = WalkDir::new(root)
            .max_depth(max_depth as usize)
            .min_depth(1)
            .follow_links(true);
        for entry_res in walker.into_iter() {
            let entry = entry_res.map_err(|err| format!("count not scan for test files, err: {err}"))?;
            if let Some(name) = entry.file_name().to_str() {
                if ! seen.insert(entry.path()) {
                    debug!("skipping duplicate test {}", entry.path().to_string_lossy());
                    continue
                }
                if name.ends_with(".clts") && entry.path().is_file() {
                    results.push(entry.into_path())
                }
            }
        }
    }
    debug!("found {} tests", results.len());
    Ok(results)
}
