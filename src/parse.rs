use ::std::path::Path;
use ::std::path::PathBuf;
use std::collections::HashSet;

use ::log::debug;

use crate::fail;

static INITIAL_PLACEHOLDER: &'static str = "::init::";

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
        let mut keywords_seen = HashSet::new();
        let mut prev_keyword = INITIAL_PLACEHOLDER.to_owned();
        let mut block: Vec<String> = Vec::new();
        loop {
            let Some(line) = lines.get(ix) else {
                break
            };
            let loc_str = format!("{}:{}", path.to_str().unwrap(), ix + 1);
            if let Err(err) = handle_line(line, &mut test, &mut prev_keyword, &mut block, &mut keywords_seen) {
                return Err(format!("parse error at {}:{}: {err}", path.to_str().unwrap(), ix + 1))
            }
            ix += 1
        }
        debug!("parsed test: {test:?}");
        Ok(test)
    }
}


fn handle_line(
        line: &str,
        test: &mut CliTest,
        prev_keyword: &mut String,
        block: &mut Vec<String>,
        keywords_seen: &mut HashSet<String>
) -> Result<(), String> {
    if line.starts_with(' ') || line.starts_with('\t') || line.is_empty() {
        //TODO @mark: de-indent
        block.push((*line).to_owned())
    } else if line.starts_with("#") {
        // do nothing, just skip parsing
    } else {
        let (mut line_keyword, code) = match line.split_once(' ') {
            Some((head, tail)) => (head.to_uppercase(), tail.to_owned()),
            None => (line.to_uppercase(), "".to_owned()),
        };
        if !line_keyword.ends_with(':') {
            fail!("found a line starting with '{line_keyword}' ('{line}'), but not followed by a colon (:); \
                    cli-test keywords must be followed by a colon, and embedded code should be indented")
        }
        line_keyword.pop();
        if BLOCK_OPTIONS.contains(&line_keyword.as_str()) {
            let is_handled_before = keywords_seen.insert(prev_keyword.clone());
            handle_keyword(&prev_keyword, block, test, is_handled_before)?;
            *prev_keyword = line_keyword;
            *block = Vec::new();
            if !code.is_empty() {
                block.push(code);
            }
        } else {
            fail!("found unknown keyword '{line_keyword}' in '{line}'; try one of ['{}'] \
                    if this is a cli-test keyword, or indent it if it is embedded code",
                    BLOCK_OPTIONS.iter().map(|s| *s).collect::<Vec<_>>().join("', '"))
        }
    }
    Ok(())
}

fn handle_keyword(
    prev_keyword: &str,
    block: &Vec<String>,
    test: &mut CliTest,
    is_handled_before: bool,
) -> Result<(), String> {
    match prev_keyword {
        "test" => {
            // if is_handled_before {
            //     fail!("more than one TEST keyword")
            // }
            test.test = block.clone()
        },
        s if s == INITIAL_PLACEHOLDER => {
            fail!("encountered code before the first keyword; use a keyword like 'TEST' before embedding code")
        }
        unknown => unimplemented!("keyword='{unknown}'"),
    }
    Ok(())
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
