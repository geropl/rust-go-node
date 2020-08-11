mod keys;

use std::collections::HashMap;
use std::time::Duration;

use serde::{Serialize, Deserialize};
use sha2::{Sha256, Digest};
use rsa::{RSAPrivateKey, PublicKey};
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

#[derive(PartialEq, Clone)]
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
        self.features.iter()
            .any(|f| *f == *feature)
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

/// It would be more rust-y to have domain, valid_until and seats as Option<...>, but for now keep it for being backwards compatible
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct License {
    pub id: String,

    /// "" means: no restriction
    pub domain: String,
    pub level: LicenseLevel,

    /// "" means: no restriction
    pub valid_until: String,    //std::time::Instant,

	/// seats == 0 means there's no seat limit
    pub seats: i32,
}

lazy_static!{
    static ref DEFAULT_LICENSE: License = License {
        id: "default-license".to_string(),
        level: LicenseLevel::Team,
        // Seats, Domain, ValidUntil are free for all
        domain: "".to_string(),
        seats: 0,
        valid_until: "".to_string(),
    };
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SignedLicense {
    pub license: License,
    pub signature: Vec<u8>,
}

impl SignedLicense {
    pub fn deserialize(bytes: &[u8]) -> Result<SignedLicense, anyhow::Error> {
        let decoded_bytes = base64::decode(&bytes)
            .map_err(|_| anyhow!("cannot decode key: {:?}", bytes))?;
        
        serde_json::from_slice(&decoded_bytes)
            .map_err(|e| anyhow!("cannot unmarshal key: {}", e))
    }

    pub fn serialize(&self) -> Result<String, anyhow::Error> {
        let serialized = serde_json::to_string(self)?;
        Ok(base64::encode(serialized))
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
    license: License,

    /// if no signed_license is given we assume having the default license. cmp. is_default_license
    signed_license: Option<SignedLicense>,

    /// if we have a signed license, this is the domain we want to validate against
    domain: Option<String>,
}

impl Evaluator {
    pub fn with_default_license() -> Evaluator {
        Evaluator {
            license: DEFAULT_LICENSE.clone(),
            signed_license: None,
            domain: None,
        }
    }

    pub fn from_license_key_bytes(key_bytes: &[u8], domain: &str) -> Result<Evaluator, anyhow::Error> {
        let signed_license = SignedLicense::deserialize(key_bytes)?;

        Ok(Evaluator {
            license: signed_license.license.clone(),
            signed_license: Some(signed_license),
            domain: Some(domain.to_string()),
        })
    }

    fn is_default_license(&self) -> bool {
        self.signed_license.is_none()
    }

    /// returns Ok(()) if the license is valid and an error with an explanation otherwise
    pub fn validate(&self) -> Result<(), anyhow::Error> {
        if self.is_default_license() {
            // n case of the default license we do not check for anything
            return Ok(())
        }

        let key_bytes_wo_signature = serde_json::to_string(&self.license)
            .map_err(|e| anyhow!("cannot remarshal key: {}", e))?;
        let hashed = {
            let mut hasher = Sha256::new();
            hasher.update(key_bytes_wo_signature);
            hasher.finalize()
        };

        // get public keys
        #[cfg(test)]
        let public_keys = keys::PUBLIC_KEYS_TEST.lock().unwrap();
        #[cfg(not(test))]
        let public_keys = &keys::PUBLIC_KEYS;

        // try to find a public key which allows to match the hashed license with its signature
        let signed_license = self.signed_license.as_ref().unwrap();
        let mut found_matching_key = false;
        for public_key in public_keys.iter() {
            if public_key.verify(SignedLicense::padding(), &hashed, &signed_license.signature).is_ok() {
                found_matching_key = true;
                break;
            }
        }
        if !found_matching_key {
            return Err(anyhow!("cannot verify key"))
        }

        // validate license's content
        let domain = self.domain.as_ref()
            .ok_or_else(|| anyhow!("expected domain to be set"))?;
        if &self.license.domain != domain {
            return Err(anyhow!("wrong domain ({}), expected {}", self.license.domain, domain));
        }

        // // TODO Instant
        // if self.license.valid_until.Before(std::time::Instant::now()) {
        //     return Err(anyhow!("not valid anymore"));
        // }

        Ok(())
    }

    /// determines if a feature is enabled by the license
    pub fn enabled(&self, feature: &Feature) -> bool {
        if self.validate().is_err() {
            return false;
        }

        match self.license.level.allowance() {
            None => false,
            Some(allowance) => allowance.is_allowed(feature),
        }
    }

    /// returns true if the license supports at least the give amount of seats
    pub fn has_enough_seats(&self, seats: i32) -> bool {
        self.license.seats == 0
            || seats <= self.license.seats
    }

    pub fn can_use_prebuild(&self, total_prebuild_time_spent: &Duration) -> bool {
        if !self.enabled(&Feature::Prebuild) {
            return false;
        }

        let allowance = match self.license.level.allowance() {
            None => return false,
            Some(a) => a,
        };
        if allowance.prebuild_time.as_secs() == 0 {
            // allowed prebuild time == 0 means the prebuild time is not limited
            return true;
        }
        if total_prebuild_time_spent <= &allowance.prebuild_time {
            return true;
        }

        false
    }

    pub fn inspect(&self) -> License {
        self.license.clone()
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    use rsa::RSAPublicKey;

    use std::time;

    const SOME_SEATS: i32 = 5;
    const SOME_DOMAIN: &str = "foobar.com";
    const SOME_ID: &str = "730d5134-768c-4a05-b7cd-ecf3757cada9";

    struct LicenseTest {
        name: String,
        license: Option<License>,
        validate: Box<dyn FnOnce(Evaluator) -> Result<(), anyhow::Error>>,
    }

    impl LicenseTest {
        fn run(self) -> Result<(), anyhow::Error> {
            let name = self.name.clone();
            println!("=== \"{}\" running...", name);
            let result = self.do_run();
            if result.is_err() {
                println!("=== \"{}\" FAILED", name);
            } else {
                println!("=== \"{}\" done.", name);
            }
            result
        }

        fn do_run(self) -> Result<(), anyhow::Error> {
            let evaluator = match &self.license {
                None => Evaluator::with_default_license(),
                Some(license) => {
                    let mut rng = rand::rngs::OsRng;
                    let private_key = RSAPrivateKey::new(&mut rng, 2048)
                        .map_err(|e| anyhow!("cannot generate key: {}", e))?;

                    // store the public key in a place where it is picked up during validation (test only)
                    let public_key = RSAPublicKey::from(&private_key);
                    {
                        // as this is for testing only we don't care about poisened locks/panics and just unwrap
                        let mut data = keys::PUBLIC_KEYS_TEST.lock().unwrap();
                        data.push(public_key);
                    }

                    let signed_license = sign(&license, &private_key)
                        .map_err(|e| anyhow!("cannot sign license: {}", e))?;
                    let serialized = signed_license.serialize()?;

                    Evaluator::from_license_key_bytes(serialized.as_bytes(), SOME_DOMAIN)?
                },
            };

            (self.validate)(evaluator)
        }
    }

    #[test]
    pub fn seats() -> Result<(), anyhow::Error> {
        struct Test {
            name: String,
            licensed: i32,
            probe: i32,
            within_limits: bool,
            #[allow(dead_code)]
            default_license: bool,
            invalid_license: bool,
        }
        let tests: Vec<Test> = vec![
            ("unlimited seats", 0, 1000, true, false, false),
            ("within limited seats", 50, 40, true, false, false),
            ("within limited seats (edge)", 50, 50, true, false, false),
            ("beyond limited seats", 50, 150, false, false, false),
            ("beyond limited seats (edge)", 50, 51, false, false, false),
            // ("invalid license", 50, 50, false, false, true), // TODO Instant!
        ].into_iter()
            .map(|t| Test{ name: t.0.to_string(), licensed: t.1, probe: t.2, within_limits: t.3, default_license: t.4, invalid_license: t.5 })
            .collect();

        for test in tests.into_iter() {
            let six_hours = Duration::from_secs(6 * 60 * 60);
            let _valid_until = match test.invalid_license {
                true => time::Instant::now().checked_sub(six_hours),
                false => time::Instant::now().checked_add(six_hours),
            }.ok_or_else(|| anyhow!("error calculating 'valid_until': out of bounds"))?;

            let license_test = LicenseTest {
                name: test.name.clone(),
                license: Some(License {
                    id: SOME_ID.to_string(),
                    domain: SOME_DOMAIN.to_string(),
                    level: LicenseLevel::Team,
                    seats: test.licensed,
                    valid_until: "".to_string(),    // TODO Instant
                }),
                validate: Box::new(move |evaluator| {
                    let within_limits = evaluator.has_enough_seats(test.probe);
                    if within_limits != test.within_limits {
                        return Err(anyhow!("'{}': has_enough_seats did not behave as expected: lic={} probe={} expected={} actual={}", test.name, test.licensed, test.probe, test.within_limits, within_limits));
                    }
                    Ok(())
                }),
            };
            license_test.run()?;
        }

        Ok(())
    }

    #[test]
    pub fn features() -> Result<(), anyhow::Error> {
        struct Test {
            name: String,
            default_license: bool,
            level: LicenseLevel,
            features: Vec<Feature>,
        }
        let tests = vec![
            Test {
                name: "no license".to_string(),
                default_license: true,
                level: LicenseLevel::Team,
                features: vec![Feature::Prebuild],
            },
            // Move the gist of this test to a deserialization test (e.g., what happens on odd LicenseLevels (666))
            // Test {
            //     name: "invalid license level".to_string(),
            //     default_license: false,
            //     level: LicenseLevel::(666),
            //     features: vec![Feature::Prebuild],
            // },
            Test {
                name: "enterprise license".to_string(),
                default_license: false,
                level: LicenseLevel::Enterprise,
                features: vec![
                    Feature::AdminDashboard,
                    Feature::SetTimeout,
                    Feature::WorkspaceSharing,
                    Feature::Snapshot,
                    Feature::Prebuild,
                ],
            }
        ];

        for test in tests.into_iter() {
            let license = if test.default_license {
                None
            } else {
                Some(License {
                    id: SOME_ID.to_string(),
                    domain: SOME_DOMAIN.to_string(),
                    level: test.level.clone(),
                    seats: SOME_SEATS,
                    valid_until: "".to_string(),    //time::Instant::now().checked_add(6 * 60 * 60)?,
                })
            };
            let license_test = LicenseTest {
                name: test.name.clone(),
                license,
                validate: Box::new(move |evaluator| {
                    let allowance = ALLOWANCE_MAP.get(&LicenseLevel::Enterprise)
                        .ok_or_else(|| anyhow!("expected allowance for Enterprise level!"))?;
                    let expected_features = allowance.features.clone();

                    for feature in test.features.iter() {
                        if !evaluator.enabled(feature) {
                            return Err(anyhow!("license does not enable {}, but should", feature.to_string()));
                        }
                    }

                    let unavailable_features: Vec<&Feature> = expected_features.iter()
                        .filter(|ef| !test.features.iter().any(|af| af == *ef))
                        .collect();
                    for feature in unavailable_features.iter() {
                        if evaluator.enabled(feature) {
                            return Err(anyhow!("license does enable {}, but shouldn't", feature.to_string()));
                        }
                    }

                    Ok(())
                }),
            };
            license_test.run()?;
        }

        Ok(())
    }


    #[test]
    pub fn can_use_prebuild() -> Result<(), anyhow::Error> {
        struct Test {
            name: String,
            license: Option<License>,
            used_time: Duration,
            expected: bool,
        }

        let hours_to_seconds: u64 = 60 * 60;
        let broken_license = License {
            id: "".to_string(),
            domain: "".to_string(),
            level: LicenseLevel::Enterprise,
            seats: 0,
            valid_until: "".to_string(),
        };
	    let enterprise_license = License {
            domain: SOME_DOMAIN.to_string(),
            id: SOME_ID.to_string(),
            level: LicenseLevel::Enterprise,
            seats: 0,
            valid_until: "".to_string(),    //time::Instant::now().checked_add(6 * hours_to_seconds)
        };
        let tests:Vec<Test> = vec![
            ("default license ok", None, Duration::from_secs(0), true),
            ("default license not ok", None, Duration::from_secs(250 * hours_to_seconds), false),
            #[allow(clippy::identity_op)]
            ("enterprise license a", Some(enterprise_license.clone()), Duration::from_secs(1 * hours_to_seconds), true),
            ("enterprise license b", Some(enterprise_license), Duration::from_secs(500 * hours_to_seconds), true),
            // ("enterprise license c", Some(enterprise_license), Duration::from_secs(-1 * hours_to_seconds), true),
            #[allow(clippy::erasing_op)]
            ("broken license", Some(broken_license), Duration::from_secs(0 * hours_to_seconds), false),
        ].into_iter()
            .map(|t| Test{ name: t.0.to_string(), license: t.1, used_time: t.2, expected: t.3 })
            .collect();

        for test in tests {
            let license_test = LicenseTest {
                name: test.name.clone(),
                license: test.license.clone(),
                validate: Box::new(move |evaluator| {
                    let actual = evaluator.can_use_prebuild(&test.used_time);
                    if test.expected != actual {
				        return Err(anyhow!("can_use_prebuild returned unexpected value: expected {}, got {}", test.expected, actual));
                    }
                    Ok(())
                }),
            };
            license_test.run()?;
        }
        Ok(())
    }
}
