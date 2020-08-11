use std::fs;
use std::path::PathBuf;

use rsa::{pem, RSAPrivateKey, RSAPublicKey};

pub fn genkey() -> Result<(), anyhow::Error> {
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