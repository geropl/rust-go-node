use std::collections::HashMap;
use std::time::Duration;

use serde::{Serialize, Deserialize};
use sha2::{Sha256, Digest};
use rsa::{RSAPrivateKey, RSAPublicKey, PublicKey};
use rsa::padding::PaddingScheme;
#[macro_use]
extern crate anyhow;
#[macro_use]
extern crate lazy_static;

#[derive(Clone, Debug)]
#[derive(Deserialize, Serialize)]   // json
#[derive(PartialEq, Eq, Hash)]  // allowances
pub enum LicenseLevel {
    Team,
    Enterprise,
}

impl std::convert::TryFrom<String> for LicenseLevel {
    type Error = anyhow::Error;
    fn try_from(s: String) -> Result<LicenseLevel, Self::Error> {
        match s.as_str() {
            "team" => Ok(LicenseLevel::Team),
            "enterprise" => Ok(LicenseLevel::Enterprise),
            _ => Err(anyhow!("unknown LicenseLevel type: {}", s)),
        }
    }
}

impl LicenseLevel {
    fn allowance(&self) -> Option<&Allowance> {
        ALLOWANCE_MAP.get(self)
    }
}

#[derive(PartialEq)]
pub enum Feature {
    AdminDashboard,
    Prebuild,
    SetTimeout,
    Snapshot,
    WorkspaceSharing,
}

impl std::string::ToString for Feature {
    fn to_string(&self) -> String {
        match self {
            Feature::AdminDashboard => "admin-dashboard",
            Feature::Prebuild => "prebuild",
            Feature::SetTimeout => "set-timeout",
            Feature::Snapshot => "snapshot",
            Feature::WorkspaceSharing => "workspace-sharing",
        }.into()
    }
}

struct Allowance {
    features: Vec<Feature>,

	// Total prebuild time that can be used at a certain level.
	// If zero the prebuild time is unlimited.
    prebuild_time: Duration,
}

impl Allowance {
    fn is_allowed(&self, feature: &Feature) -> bool {
        self.features.iter().any(|f| *f == *feature)
    }
}

lazy_static!{
    static ref ALLOWANCE_MAP: HashMap<LicenseLevel, Allowance> = vec![
        (LicenseLevel::Team, Allowance {
            prebuild_time: Duration::from_secs(50 * 60 * 60),   // 50h
            features: vec![
                Feature::Prebuild,
            ],
        }),
        (LicenseLevel::Enterprise, Allowance {
            prebuild_time: Duration::from_secs(0),
            features: vec![
                Feature::Prebuild,

                Feature::AdminDashboard,
                Feature::SetTimeout,
                Feature::Snapshot,
                Feature::WorkspaceSharing,
            ],
        }),
    ].into_iter().collect();
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct License {
    pub id: String,
    pub domain: String,
    pub level: LicenseLevel,
    pub valid_until: String,    //std::time::Instant,

	/// seats == 0 means there's no seat limit
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

    pub fn padding() -> PaddingScheme {
        PaddingScheme::new_pkcs1v15_sign(Some(rsa::hash::Hash::SHA2_256))
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
    let signature = private_key.sign_blinded(&mut rng, SignedLicense::padding(), &hashed)?;

    Ok(SignedLicense {
        license: license.clone(),
        signature,
    })
}

pub struct Evaluator {
    signed_license: SignedLicense,
}

impl Evaluator {
    pub fn from_license_key_bytes(key_bytes: &[u8]) -> Result<Evaluator, anyhow::Error> {
        let decoded_bytes = SignedLicense::decode(&key_bytes)
            .map_err(|_| anyhow!("cannot decode key: {:?}", key_bytes))?;
        
        let signed_license = serde_json::from_slice(&decoded_bytes)
            .map_err(|e| anyhow!("cannot unmarshal key: {}", e))?;

        Ok(Evaluator {
            signed_license,
        })
    }

    /// returns Ok(()) if the license is valid and an error with an explanation otherwise
    pub fn validate(&self, domain: &str) -> Result<(), anyhow::Error> {
        let key_bytes_wo_signature = serde_json::to_vec(&self.signed_license.license)
            .map_err(|e| anyhow!("cannot remarshal key: {}", e))?;
        let hashed = {
            let mut hasher = Sha256::new();
            hasher.update(key_bytes_wo_signature);
            hasher.finalize()
        };

        // try to find a public key which allows to match the hashed license with its signature
        let public_keys: Vec<RSAPublicKey> = vec![];
        let mut found_matching_key = false;
        for public_key in public_keys.iter() {
            if public_key.verify(SignedLicense::padding(), &hashed, &self.signed_license.signature).is_ok() {
                found_matching_key = true;
                break;
            }
        }
        if !found_matching_key {
            return Err(anyhow!("cannot verify key"))
        }

        // validate license's content
        if self.signed_license.license.domain != domain {
            return Err(anyhow!("wrong domain ({}), expected {}", self.signed_license.license.domain, domain));
        }

        // // TODO Instant
        // if self.signed_license.license.valid_until.Before(std::time::Instant::now()) {
        //     return Err(anyhow!("not valid anymore"));
        // }

        Ok(())
    }

    /// determines if a feature is enabled by the license
    pub fn enabled(&self, feature: &Feature) -> bool {
        match self.signed_license.license.level.allowance() {
            None => false,
            Some(allowance) => allowance.is_allowed(feature),
        }
    }

    /// returns true if the license supports at least the give amount of seats
    pub fn has_enough_seats(&self, seats: i32) -> bool {
        self.signed_license.license.seats == 0
            || seats <= self.signed_license.license.seats
    }

    pub fn can_use_prebuild(&self, total_prebuild_time_spent: Duration) -> bool {
        if !self.enabled(&Feature::Prebuild) {
            return false;
        }

        let allowance = match self.signed_license.license.level.allowance() {
            None => return false,
            Some(a) => a,
        };
        if allowance.prebuild_time.as_secs() == 0 {
            // allowed prebuild time == 0 means the prebuild time is not limited
            return true;
        }
        if total_prebuild_time_spent <= allowance.prebuild_time {
            return true;
        }

        false
    }

    pub fn inspect(&self) -> License {
        self.signed_license.license.clone()
    }
}


#[cfg(test)]
mod tests {
    // use super::*;
}
