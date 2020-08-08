use licensorlib as lib;

use std::convert::TryFrom;
use std::time::Instant;
use std::fs;
use std::path::PathBuf;

use clap::Clap;
#[macro_use]
extern crate anyhow;
use rsa::{pem, RSAPrivateKey, RSAPublicKey};


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

#[derive(Clap)]
struct SignParams {
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

#[derive(Clap)]
struct ValidateParams {
    /// the license to validate. Reads from stdin if not provided
    license: Option<String>,

    /// domain to evaluate the license against
    #[clap(long)]
    domain: String,
}

fn parse_duration(valid_for: &str) -> Result<std::time::Duration, std::num::ParseIntError> {
    let secs = u64::from_str_radix(valid_for, 10)?;
    Ok(std::time::Duration::from_secs(secs))
}

fn main() -> Result<(), anyhow::Error> {
    let root: Root = Root::parse();
    match root.subcmd {
        SubCmd::GenKey{} => genkey(),
        SubCmd::Sign(params) => sign(params),
        SubCmd::Validate(params) => validate(params),
    }
}

fn genkey() -> Result<(), anyhow::Error> {
    use rsa::PrivateKeyEncoding;
    use rsa::PublicKeyEncoding;

    // generate private key
    let mut rng = rand::rngs::OsRng;
    let private_key = RSAPrivateKey::new(&mut rng, 2048)?;

    // encode and write private key
    let encoded_private_key = {
        let content = private_key.to_pkcs1()?;
        let pem = pem::Pem {
            tag: "PRIVATE KEY".to_string(), // !!! We use a PKCS8 header but PKCS1 content
            contents: content,
        };
        pem::encode(&pem)
    };
    fs::write(PathBuf::from("private_key.pem"), encoded_private_key)?;

    // encode and write public key
    let encoded_public_key = {
        let public_key = RSAPublicKey::from(private_key);
        let content = public_key.to_pkcs1()?;
        let pem = pem::Pem {
            tag: "PUBLIC KEY".to_string(), // !!! We use a PKCS8 header but PKCS1 content
            contents: content,
        };
        pem::encode(&pem)
    };
    fs::write(PathBuf::from("public_key.pem"), encoded_public_key)?;

    Ok(())
}

fn sign(params: SignParams) -> Result<(), anyhow::Error> {
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

    println!("{}", result.encode());
    Ok(())
}

fn validate(params: ValidateParams) -> Result<(), anyhow::Error> {
    let license_str = match params.license {
        Some(license_str) => license_str,
        None => {
            use std::io::{stdin, Read};

            let mut buffer = String::new();
            stdin().read_to_string(&mut buffer)?;
            buffer
        }
    };

    let evaluator = lib::Evaluator::from_license_key_bytes(license_str.as_bytes())?;
    evaluator.validate(params.domain.as_str())?;

    println!("{:?}", evaluator.inspect());

    Ok(())
}
