use crate::shiori;
use std::io::stdin;

pub struct StdInParseSource {}

impl StdInParseSource {
    pub fn new() -> StdInParseSource {
        StdInParseSource {}
    }
}

impl shiori::parse_interface::ParseSource for StdInParseSource {
    fn next_line(&self) -> String {
        let mut buf = String::new();
        stdin().read_line(&mut buf).expect("read line error.");

        return buf.trim_end().to_string();
    }
}
