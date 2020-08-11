use licensorlib as lib;

use clap::Clap;

#[derive(Clap)]
pub struct ValidateParams {
    /// the license to validate. Reads from stdin if not provided
    license: Option<String>,

    /// domain to evaluate the license against
    #[clap(long)]
    domain: String,
}

pub fn validate(params: ValidateParams) -> Result<(), anyhow::Error> {
    let license_str = match params.license {
        Some(license_str) => license_str,
        None => {
            use std::io::{stdin, Read};

            let mut buffer = String::new();
            stdin().read_to_string(&mut buffer)?;
            buffer
        }
    };

    let evaluator = lib::Evaluator::from_license_key_bytes(license_str.as_bytes(), params.domain.as_str())?;
    evaluator.validate()?;

    println!("{:?}", evaluator.inspect());

    Ok(())
}
