use ::std::path::Path;
use ::std::path::PathBuf;

use ::log::debug;

use crate::fail;

#[derive(Debug)]
pub struct CliTest {
    pub path: PathBuf,
    pub test: Vec<String>,
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
        let mut test = CliTest {
            path: path.to_owned(),
            test: Vec::new(),
        };
        let mut prev_keyword = "init".to_owned();
        let mut block: Vec<String> = Vec::new();
        loop {
            let Some(line) = lines.get(ix) else {
                break
            };
            if line.starts_with(' ') || line.starts_with('\t') || line.is_empty() {
                //TODO @mark: de-indent
                block.push((*line).to_owned());
            } else if line.starts_with("#") {
                // do nothing, just skip parsing
            } else {
                let (mut line_keyword, code) = match line.split_once(' ') {
                    Some((head, tail)) => (head.to_uppercase(), tail.to_owned()),
                    None => (line.to_uppercase(), "".to_owned()),
                };
                if ! line_keyword.ends_with(':') {
                    fail!("found a line starting with '{line_keyword}' at {}:{ix} : '{line}', but not followed by a colon (:); \
                        cli-test keywords must be followed by a colon, amd embedded code should be indented",
                        path.to_str().unwrap())
                }
                line_keyword.pop();
                if BLOCK_OPTIONS.contains(&line_keyword.as_str()) {
                    handle_keyword(prev_keyword, block, &mut test);
                    prev_keyword = line_keyword;
                    block = Vec::new();
                    if ! code.is_empty() {
                        block.push(code);
                    }
                } else {
                    fail!("found unknown keyword '{line_keyword}' at {}:{ix} : '{line}'; try one of ['{}'] \
                            if this is a cli-test keyword, or indent it if it is embedded code", path.to_str().unwrap(),
                            BLOCK_OPTIONS.iter().map(|s| *s).collect::<Vec<_>>().join("', '"))
                }
            }
            ix += 1
        }
        debug!("parsed test: {test:?}");
        Ok(test)
    }
}

fn handle_keyword(prev_keyword: String, block: Vec<String>, test: &mut CliTest) {
    match prev_keyword.as_str() {
        "test" => {
            test.test = block
        },
        unknown => unimplemented!("keyword='{unknown}'"),
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
