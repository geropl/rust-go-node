use serde::{Serialize, Deserialize};
use sha2::{Sha256, Digest};
use rsa::RSAPrivateKey;
use rsa::padding::PaddingScheme;
#[macro_use]
extern crate anyhow;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum LicenseLevel {
    LevelTeam,
    LevelEnterprise,
}

impl std::string::ToString for LicenseLevel {
    fn to_string(&self) -> String {
        match &self {
            LicenseLevel::LevelTeam => "team",
            LicenseLevel::LevelEnterprise => "enterprise"
        }.into()
    }
}

impl std::convert::TryFrom<String> for LicenseLevel {
    type Error = anyhow::Error;
    fn try_from(s: String) -> Result<LicenseLevel, Self::Error> {
        match s.as_str() {
            "team" => Ok(LicenseLevel::LevelTeam),
            "enterprise" => Ok(LicenseLevel::LevelEnterprise),
            _ => Err(anyhow!(format!("Unknown LicenseLevel type: {}", s))),
        }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct License {
    pub id: String,
    pub domain: String,
    pub level: LicenseLevel,
    pub valid_until: String,    //std::time::Instant,
    pub seats: i32,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SignedLicense {
    pub license: License,
    pub signature: Vec<u8>,
}

impl SignedLicense {
    pub fn encode(&self) -> String {
        base64::encode(&self.signature)
    }

    pub fn decode(bytes: &[u8]) -> Result<Vec<u8>, base64::DecodeError> {
        base64::decode(bytes)
    }
}

/// Sign signs a license so that it can be used with the evaluator
pub fn sign(license: &License, private_key: &RSAPrivateKey) -> Result<SignedLicense, anyhow::Error> {
    // to json
    let raw_str = serde_json::to_string(license)?;

    // hash with sha256
    let hashed = {
        let mut hasher = Sha256::new();
        hasher.update(raw_str);
        hasher.finalize()
    };

    // sign with PKCS1 v1.5
    let mut rng = rand::rngs::OsRng;
    let padding = PaddingScheme::new_pkcs1v15_sign(Some(rsa::hash::Hash::SHA2_256));
    let signature = private_key.sign_blinded(&mut rng, padding, &hashed)?;

    Ok(SignedLicense {
        license: license.clone(),
        signature,
    })
}

// pub struct Evaluator {
// }

// impl Evaluator {
//     pub fn from_license(key_bytes: Vec<u8>) -> Result<Evaluator, anyhow::Error> {
//         // SignedLicense::decode(bytes: &[u8])

//         todo!()
//         // Ok(Evaluator {

//         // })
//     }
// }


#[cfg(test)]
mod tests {
    // use super::*;
}
