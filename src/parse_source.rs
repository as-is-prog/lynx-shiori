use crate::shiori;
use std::{
    fs::{self, File, OpenOptions},
    io::{self, stdin, BufReader, BufWriter, Read, Write},
};

pub struct StdInParseSource {}

impl StdInParseSource {
    pub fn new() -> StdInParseSource {
        let file = fs::File::create("lynx.log").unwrap();

        StdInParseSource {}
    }
}

impl shiori::parse_interface::ParseSource for StdInParseSource {
    fn next_line(&self) -> String {
        let mut buf = String::new();
        stdin().read_line(&mut buf).expect("read line error.");

        let mut file = OpenOptions::new().append(true).open("lynx.log").unwrap();
        file.write_all(buf.as_bytes()).unwrap();

        return buf.trim_end().to_string();
    }
}
