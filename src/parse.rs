use ::std::path::Path;
use ::std::path::PathBuf;

use ::log::debug;

use crate::fail;

#[derive(Debug)]
pub struct CliTest {
    pub path: PathBuf,
}

static BLOCK_OPTIONS: [&'static str; 3] = [
    "TEST",
    "EXIT_CODE",
    "OUT",
];

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
                fail!("empty test at '{}'", path.to_string_lossy())
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
            } else if line.starts_with("#") {
                // do nothing, just skip parsing
            } else {
                let mut keyword = match line.split_once(' ').map(|(head, _tail)| head) {
                    Some(head) => head,
                    None => line,
                }.to_uppercase();
                if ! keyword.ends_with(':') {
                    fail!("found a line starting with '{keyword}' at {}:{ix} : '{line}', but not followed by a colon (:); \
                        cli-test keywords must be followed by a colon, amd embedded code should be indented",
                        path.to_str().unwrap())
                }
                keyword.pop();
                match keyword.as_str() {
                    "test" => unimplemented!(),
                    unknown => {
                        fail!("found unknown keyword '{keyword}' at {}:{ix} : '{line}'; try one of ['{}'] \
                            if this is a cli-test keyword, or indent it if it is embedded code", path.to_str().unwrap(),
                            BLOCK_OPTIONS.iter().map(|s| *s).collect::<Vec<_>>().join("', '"))
                    },
                }
            }
            ix += 1
        }
        debug!("parsed test: {test:?}");
        Ok(test)
    }
}

#[cfg(test)]
mod tests {
    use ::std::mem;

    use super::*;

    #[test]
    fn all_variants_have_option() {
        assert_eq!(mem::variant_count::<BlockType>(), BLOCK_OPTIONS.len())
    }

    #[test]
    fn all_options_are_parseable() {
        todo!()
    }
}
