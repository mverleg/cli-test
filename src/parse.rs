use ::std::path::Path;
use ::std::path::PathBuf;

use ::log::debug;

#[derive(Debug)]
pub struct CliTest {
    pub path: PathBuf,
}

impl CliTest {
    pub fn parse(code: &str, path: &Path) -> Result<Self, String> {
        let lines = code.lines().collect::<Vec<_>>();
        let mut ix = 0;
        match lines.get(0) {
            Some(first) => {
                if first.starts_with("#") {
                    ix += 1
                }
            }
            None => {
                return Err(format!("empty test at '{}'", path.to_string_lossy()))
            }
        }
        let mut test = CliTest {
            path: path.to_owned()
        };
        let mut block = Vec::new();
        loop {
            let Some(line) = lines.get(ix) else {
                break
            };
            if line.starts_with(' ') || line.starts_with('\t') || line.is_empty() {
                block.push(line);
            } else {
                let keyword = match line.split_once(' ').map(|(head, tail)| head) {
                    Some(head) => head,
                    None => line,
                }.to_lowercase();
                todo!("keyword {ix}: '{}'", keyword)
            }
            ix += 1
        }
        debug!("parsed test: {test:?}");
        Ok(test)
    }
}
