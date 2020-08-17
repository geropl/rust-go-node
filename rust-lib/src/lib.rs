pub mod licensor;
mod keys;

pub use licensor::*;

#[macro_use]
extern crate anyhow;
#[macro_use]
extern crate lazy_static;

pub fn from_license_key_bytes(key: String, domain: String) -> Result<Evaluator, anyhow::Error> {
    if key.is_empty() {
        return Ok(Evaluator::with_default_license());
    }
    Evaluator::from_license_key_bytes(key.as_bytes(), &domain)
}

pub fn inspect_str(evaluator: &Evaluator) -> Result<String, anyhow::Error> {
    let license = evaluator.inspect();
    serde_json::to_string(&license)
        .map_err(|e| e.into())
}