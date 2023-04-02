use ::std::cell::LazyCell;
use ::std::collections::hash_map::Entry;
use ::std::collections::HashMap;
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
    pub cases: String,
    pub exit_code: String,
    pub out: String,
    pub err: String,
}

#[derive(Debug)]
pub struct CliTest {
    pub path: PathBuf,
    pub exit_code: String,
    pub out: String,
    pub err: String,
    pub cases: Vec<TestCase>,
}

static BLOCK_OPTIONS: [&'static str; 4] = [
    "TEST",
    "EXIT_CODE",
    "OUT",
    "ERR",
];

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum SeenBefore {
    Never,
    BeforeThisCase,
    ThisCase,
}

impl SeenBefore {
    pub fn fail_if_seen_globally(self, name: &str) -> Result<(), String> {
        if self != SeenBefore::Never {
            //TODO @mark: this msg isn't ideal since it is also used if it's allowed to repeat per-test
            return Err(format!("encountered {name} more than once, must be unique"))
        }
        Ok(())
    }

    pub fn fail_if_seen_for_test(self, name: &str) -> Result<(), String> {
        if self == SeenBefore::ThisCase {
            return Err(format!("encountered {name} more than once in this testcase, must be unique per testcase"))
        }
        Ok(())
    }
}

impl CliTest {
    pub fn parse(code: &str, path: &Path) -> Result<Self, String> {
        let lines = code.lines().collect::<Vec<_>>();
        let mut ix = 0;
        let mut test = CliTest {
            path: path.to_owned(),
            exit_code: "".to_owned(),
            out: "".to_owned(),
            err: "".to_owned(),
            cases: Vec::new(),
        };
        let mut seen_for_test = HashMap::new();
        let mut prev_keyword = INITIAL_PLACEHOLDER.to_owned();
        let mut block: Vec<String> = Vec::new();
        loop {
            let Some(line) = lines.get(ix) else {
                break
            };
            if let Err(err) = handle_line(line, &mut test, &mut prev_keyword, &mut block, &mut seen_for_test) {
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
        seen_for_test: &mut HashMap<String, usize>
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
            let seen_before = register_seen(prev_keyword, seen_for_test, test.cases.len());
            //TODO @mark: is_handled_before not that useful with multiple test cases
            handle_keyword(&prev_keyword, block, test, seen_before)?;
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

fn register_seen(prev_keyword: &str, seen_for_test: &mut HashMap<String, usize>, cur_case: usize) -> SeenBefore {
    match seen_for_test.entry(prev_keyword.to_owned()) {
        Entry::Occupied(mut entry) => {
            let prev = *entry.get();
            *entry.get_mut() = cur_case;
            if prev == cur_case {
                SeenBefore::ThisCase
            } else {
                SeenBefore::BeforeThisCase
            }
        }
        Entry::Vacant(vacancy) => {
            vacancy.insert(cur_case);
            SeenBefore::Never
        }
    }
}

fn handle_keyword(
    prev_keyword: &str,
    block: &Vec<String>,
    test: &mut CliTest,
    seen_before: SeenBefore,
) -> Result<(), String> {
    let code = combine_dedent(block);
    match prev_keyword {
        "TEST" => {
            test.cases.push(TestCase {
                cases: code,
                exit_code: "".to_owned(),
                out: "".to_owned(),
                err: "".to_owned(),
            })
        },
        "EXIT_CODE" => {
            match test.cases.last_mut() {
                Some(cur) => {
                    seen_before.fail_if_seen_for_test("EXIT_CODE")?;
                    (*cur).exit_code = code
                }
                None => {
                    seen_before.fail_if_seen_globally("EXIT_CODE")?;
                    test.exit_code = code
                }
            }
        }
        "OUT" => {
            match test.cases.last_mut() {
                Some(cur) => {
                    seen_before.fail_if_seen_for_test("OUT")?;
                    (*cur).out = code
                }
                None => {
                    seen_before.fail_if_seen_globally("OUT")?;
                    test.err = code
                }
            }
        }
        "ERR" => {
            match test.cases.last_mut() {
                Some(cur) => {
                    seen_before.fail_if_seen_for_test("ERR")?;
                    (*cur).err = code
                }
                None => {
                    seen_before.fail_if_seen_globally("ERR")?;
                    test.err = code
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

fn char_indent(ch: char) -> Option<usize> {
    if ch == ' ' {
        Some(1)
    } else if ch == '\t' {
        Some(4)
    } else {
        None
    }
}

fn count_indent(line: &str) -> usize {
    let mut indent = 0;
    for ch in line.chars() {
        match char_indent(ch) {
            Some(ind) => indent += ind,
            None => break,
        }
    }
    return indent
}

fn combine_dedent(lines: &Vec<String>) -> String {
    debug_assert!(!lines.is_empty());
    let mut min_indent = usize::MAX;
    if lines.len() == 1 {
        min_indent = count_indent(&lines[0]);
    } else {
        for line in lines.iter().skip(1) {
            let indent = count_indent(line);
            if indent < min_indent {
                min_indent = indent
            }
        }
    }
    let mut code = String::new();
    for line in lines {
        let mut indent = 0;
        let mut ix = 0;
        for ch in line.chars() {
            if indent >= min_indent {
                code.push_str(&line[ix..])
            }
            match char_indent(ch) {
                Some(ind) => indent += ind,
                None => break,
            }
            ix += ch.len_utf8();
        }
    }
    code
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
