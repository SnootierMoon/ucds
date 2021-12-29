use crate::error::UcdFileParseError;
use anyhow::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

pub const UCD_DATA_PATH: &str = "/usr/share/unicode/UnicodeData.txt";

pub struct UcdData {
    pub data: Vec<(char, String)>,
}

impl UcdData {
    pub fn new<P: AsRef<Path>>(ucd_path: P) -> Result<Self, Error> {
        let mut data = Vec::new();

        let ucd = File::open(ucd_path)?;
        let read = BufReader::new(ucd);
        for line in read.lines() {
            let line = line?;
            let mut stuff = line.split(';');

            let hex_code = stuff.next().ok_or(UcdFileParseError)?;
            let name = stuff.next().ok_or(UcdFileParseError)?;

            let code_point = u32::from_str_radix(hex_code, 16)?;
            let ch = match char::from_u32(code_point) {
                Some(valid) => valid,
                None => continue,
            };

            data.push((ch, name.to_owned()));
        }

        Ok(Self { data })
    }

    pub fn search(&self, query: &str) -> impl Iterator<Item = (char, &str)> {
        let uppercase_query = query.to_ascii_uppercase();

        self.data.iter().filter_map(move |(ch, name)| {
            name.contains(&uppercase_query)
                .then(|| (*ch, name.as_str()))
        })
    }
}
