use licensorlib as lib;

use std::convert::TryFrom;
use std::fs;

use clap::Clap;
use rsa::{pem, RSAPrivateKey};
use chrono::Utc;

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

    /// days the license is valid for (defaults to one year or 365 days)
    #[clap(long = "valid-for", default_value = "365", parse(try_from_str = parse_duration))]
    valid_for: chrono::Duration
}

fn parse_duration(valid_for: &str) -> Result<chrono::Duration, std::num::ParseIntError> {
    let days = i64::from_str_radix(valid_for, 10)?;
    Ok(chrono::Duration::days(days))
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
    let level = lib::LicenseLevel::try_from(params.level.as_str())?;
    let valid_until = Utc::now().checked_add_signed(params.valid_for)
        .ok_or_else(|| anyhow!("error calculating valid_until"))?;
    let license = lib::License {
        domain: params.domain,
        id: params.id,
        seats: Some(params.seats),
        level,
        valid_until,
    };
    
    // sign
    let result = lib::sign(&license, &private_key)?;

    println!("{}", result.serialize()?);
    Ok(())
}