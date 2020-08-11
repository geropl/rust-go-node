mod genkey;
mod sign;
mod validate;

use genkey::genkey;
use sign::{SignParams, sign};
use validate::{ValidateParams, validate};

use clap::Clap;


#[derive(Clap)]
#[clap(
    name = "licensor",
    about = "CLI for signing licenses"
)]
struct Root {
    #[clap(subcommand)]
    subcmd: SubCmd,
}

#[derive(Clap)]
enum SubCmd {
    #[clap(
        name = "genkey",
        about = "Generates a public/private key for signing licenses"
    )]
    GenKey{},
    #[clap(
        name = "sign",
        about = "Signs a license"
    )]
    Sign(SignParams),
    #[clap(
        name = "validate",
        about = "Validates a license - reads from stdin if no argument is provided"
    )]
    Validate(ValidateParams),
}

pub fn run() -> Result<(), anyhow::Error> {
    let root: Root = Root::parse();
    match root.subcmd {
        SubCmd::GenKey{} => genkey(),
        SubCmd::Sign(params) => sign(params),
        SubCmd::Validate(params) => validate(params),
    }
}