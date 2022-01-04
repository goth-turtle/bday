#![warn(clippy::expect_used)]
#![warn(clippy::unwrap_used)]

use std::error::Error;
use bday::Bday;

mod bday;

fn run() -> Result<(), Box<dyn Error>> {
    let bday = Bday::init()?;
    bday.run()
}

fn main() {
    if let Err(e) = run() {
        println!("{}", e);
    };
}

