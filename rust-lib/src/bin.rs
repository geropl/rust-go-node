mod cli;

#[macro_use]
extern crate anyhow;

#[cfg(feature = "cli")]
fn main() -> Result<(), anyhow::Error> {
    cli::run()
}