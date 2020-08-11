mod cli;

#[macro_use]
extern crate anyhow;

fn main() -> Result<(), anyhow::Error> {
    cli::run()
}