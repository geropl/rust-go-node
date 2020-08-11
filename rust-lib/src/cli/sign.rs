use licensorlib as lib;

use std::convert::TryFrom;
use std::time::Instant;
use std::fs;

use clap::Clap;
use rsa::{pem, RSAPrivateKey};

#[derive(Clap)]
pub struct SignParams {
    // All those #[clap(long)] make "domain" -> "--domain" to be compatible to the Go version

    /// domain for which the license is valid
    #[clap(long)]
    domain: String,

    /// ID of the license
    #[clap(long)]
    id: String,

    /// license level, must be one of team, enterprise
    #[clap(long)]
    level: String,

    /// number of seats the license is valid for
    #[clap(long, default_value = "5")]
    seats: i32,

    /// path to the private key to sign the license with
    #[clap(short = "k")]
    key: String,

    /// time the license is valid for (defaults to one year or 365*24*60*60 seconds)
    #[clap(long = "valid-for", default_value = "31536000", parse(try_from_str = parse_duration))]
    valid_for: std::time::Duration
}

fn parse_duration(valid_for: &str) -> Result<std::time::Duration, std::num::ParseIntError> {
    let secs = u64::from_str_radix(valid_for, 10)?;
    Ok(std::time::Duration::from_secs(secs))
}

pub fn sign(params: SignParams) -> Result<(), anyhow::Error> {
    // read and parse private key from file
    let fc = fs::read_to_string(params.key)?;
    let pem = pem::parse(fc.as_bytes())?;
    if pem.tag != "PRIVATE KEY" { // !!! We use a PKCS8 header but PKCS1 content
        return Err(anyhow!("unknown PEM block type {}", pem.tag));
    }
    let private_key = RSAPrivateKey::from_pkcs1(&pem.contents)?;

    // construct license
    let level = lib::LicenseLevel::try_from(params.level)?;
    let _valid_until = Instant::now().checked_add(params.valid_for)
        .ok_or_else(|| anyhow!("error calculating valid_until"))?;
    let license = lib::License {
        domain: params.domain,
        id: params.id,
        seats: params.seats,
        level,
        valid_until: "".to_string(),       // TODO serde for Instant!
    };
    
    // sign
    let result = lib::sign(&license, &private_key)?;

    println!("{}", result.serialize()?);
    Ok(())
}