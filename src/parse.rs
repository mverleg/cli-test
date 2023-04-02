use ::std::path::PathBuf;

#[derive(Debug)]
pub struct CliTest {
    pub path: PathBuf,
}

impl CliTest {
    pub fn parse(code: &str) -> Result<Self, String> {
        unimplemented!()
    }
}
