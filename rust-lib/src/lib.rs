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

// use std::sync::{Arc, Mutex};
// lazy_static!{

//     static ref INSTANCES: Arc<Mutex<Vec<licensor::Evaluator>>> = Arc::new(Mutex::new(vec![]));
// }

// pub fn init_with_default_license() -> usize {
//     let new_instance = licensor::Evaluator::with_default_license();
//     let mut instances = INSTANCES.lock().unwrap();
//     instances.push(new_instance);
//     instances.len() - 1
// }

// pub fn init(key: String, domain: String) -> Result<usize, anyhow::Error> {
//     let new_instance = licensor::Evaluator::from_license_key_bytes(key.as_bytes(), &domain)?;
//     let mut instances = INSTANCES.lock().unwrap();
//     instances.push(new_instance);
//     Ok(instances.len() - 1)
// }

// pub fn validate(id: usize) -> Result<bool, anyhow::Error> {
//     let lock = lock();
//     let instance = get_instance(&lock, id)?;

//     instance.validate()
//         .map(|_| true)
// }

// pub fn enabled(id: usize, feature: String) -> Result<bool, anyhow::Error> {
//     let lock = lock();
//     let instance = get_instance(&lock, id)?;

//     use std::convert::TryFrom;
//     let feature = licensor::Feature::try_from(feature.as_str())?;
//     Ok(instance.enabled(&feature))
// }

// pub fn has_enough_seats(id: usize, seats: i32) -> Result<bool, anyhow::Error> {
//     let lock = lock();
//     let instance = get_instance(&lock, id)?;

//     Ok(instance.has_enough_seats(seats))
// }

// pub fn can_use_prebuild(id: usize, total_seconds_used: i64) -> Result<bool, anyhow::Error> {
//     let lock = lock();
//     let instance = get_instance(&lock, id)?;

//     Ok(instance.can_use_prebuild(&chrono::Duration::seconds(total_seconds_used)))
// }

// pub fn inspect(id: usize) -> Result<String, anyhow::Error> {
//     let lock = lock();
//     let instance = get_instance(&lock, id)?;
    
//     let license = instance.inspect();
//     let serialized = serde_json::to_string_pretty(&license)?;
//     Ok(serialized)
// }

// fn lock() -> std::sync::MutexGuard<'static, Vec<licensor::Evaluator>> {
//     INSTANCES.lock().unwrap()
// }
// fn get_instance<'a>(instances: &'a std::sync::MutexGuard<'static, Vec<licensor::Evaluator>>, id: usize) -> Result<&'a licensor::Evaluator, anyhow::Error> {
//     instances.get(id)
//         .ok_or_else(|| anyhow!("invalid instance ID: {}", id))
// }