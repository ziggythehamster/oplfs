use oplfs::cli;

use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    cli::run()
}
