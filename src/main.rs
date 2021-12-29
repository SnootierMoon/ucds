use anyhow::Error;
use data::{UcdData, UCD_DATA_PATH};

mod data;
mod error;

fn main() -> Result<(), Error> {
    let d = UcdData::new(UCD_DATA_PATH)?;

    let stdin = std::io::stdin();
    let mut input = String::new();

    stdin.read_line(&mut input);

    for (ch, name) in d.search(input.trim()) {
        println!("{}: {}", name, ch);
    }

    Ok(())
}
