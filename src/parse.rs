use ::std::cell::LazyCell;
use ::std::collections::HashSet;
use ::std::path::Path;
use ::std::path::PathBuf;

use ::log::debug;

use ::regex::Regex;

use crate::fail;

static INITIAL_PLACEHOLDER: &'static str = "::init::";

thread_local! {
    static LEADING_SPACE_RE: LazyCell<Regex> = LazyCell::new(|| Regex::new(r"^(\s|$)").unwrap());
    static EMPTY_RE: LazyCell<Regex> = LazyCell::new(|| Regex::new(r"^\s*$").unwrap());
}

#[derive(Debug)]
pub struct TestCase {
    pub cases: Vec<String>,
    pub exit_code: Vec<String>,
    pub out: Vec<String>,
    pub err: Vec<String>,
}

#[derive(Debug)]
pub struct CliTest {
    pub path: PathBuf,
    pub exit_code: Vec<String>,
    pub out: Vec<String>,
    pub err: Vec<String>,
    pub cases: Vec<TestCase>,
}

static BLOCK_OPTIONS: [&'static str; 4] = [
    "TEST",
    "EXIT_CODE",
    "OUT",
    "ERR",
];

impl CliTest {
    pub fn parse(code: &str, path: &Path) -> Result<Self, String> {
        let lines = code.lines().collect::<Vec<_>>();
        let mut ix = 0;
        let mut test = CliTest {
            path: path.to_owned(),
            exit_code: Vec::new(),
            out: Vec::new(),
            err: Vec::new(),
            cases: Vec::new(),
        };
        let mut keywords_seen = HashSet::new();
        let mut prev_keyword = INITIAL_PLACEHOLDER.to_owned();
        let mut block: Vec<String> = Vec::new();
        loop {
            let Some(line) = lines.get(ix) else {
                break
            };
            if let Err(err) = handle_line(line, &mut test, &mut prev_keyword, &mut block, &mut keywords_seen) {
                return Err(format!("parse error at {}:{}: {err} (in line '{}')", path.to_str().unwrap(), ix + 1, line))
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
    if LEADING_SPACE_RE.with(|re| re.is_match(line)) {
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
            fail!("found a line starting with '{line_keyword}' but not followed by a colon (:); \
                    cli-test keywords must be followed by a colon, and embedded code should be indented")
        }
        line_keyword.pop();
        if BLOCK_OPTIONS.contains(&line_keyword.as_str()) {
            let is_handled_before = keywords_seen.insert(prev_keyword.clone());
            //TODO @mark: is_handled_before not that useful with multiple test cases
            handle_keyword(&prev_keyword, block, test, is_handled_before)?;
            *prev_keyword = line_keyword;
            *block = Vec::new();
            block.push(code);
        } else {
            fail!("found unknown keyword '{line_keyword}'; try one of ['{}'] \
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
        "TEST" => {
            test.cases.push(TestCase {
                cases: block.clone(),
                exit_code: Vec::new(),
                out: Vec::new(),
                err: Vec::new(),
            })
        },
        "EXIT_CODE" => {
            match test.cases.last_mut() {
                Some(cur) => {
                    //TODO @mark: one per test
                    (*cur).exit_code = block.clone()
                }
                None => {
                    if is_handled_before {
                        fail!("EXIT_CODE may appear only once before the first test case")
                    }
                    test.exit_code = block.clone()
                }
            }
        }
        "OUT" => {
            match test.cases.last_mut() {
                Some(cur) => {
                    //TODO @mark: one per test
                    (*cur).out = block.clone()
                }
                None => {
                    if is_handled_before {
                        fail!("OUT may appear only once before the first test case")
                    }
                    test.err = block.clone()
                }
            }
        }
        "ERR" => {
            match test.cases.last_mut() {
                Some(cur) => {
                    //TODO @mark: one per test
                    (*cur).err = block.clone()
                }
                None => {
                    if is_handled_before {
                        fail!("ERR may appear only once before the first test case")
                    }
                    test.err = block.clone()
                }
            }
        }
        s if s == INITIAL_PLACEHOLDER => {
            if ! EMPTY_RE.with(|re| block.iter().all(|line| re.is_match(line))) {
                debug!("found {} lines before first keyword:\n  {}", block.len(), block.join("  \n"));
                fail!("encountered code before the first keyword; use a keyword like 'TEST' before embedding code")
            }
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
