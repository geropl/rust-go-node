use licensorlib as licensor;
// TODO Is there a way to generate bindings for re-exports without having to wrap those?
use licensor::Feature;

use std::convert::TryFrom;

use wasm_bindgen::prelude::*;
use chrono::Duration;

#[wasm_bindgen]
pub struct Evaluator {
    eval: licensor::Evaluator,
}

#[wasm_bindgen]
impl Evaluator {
    /// This creates a new Evaluator for the given key and domain to check against
    #[wasm_bindgen(js_name = "createFromLicenseKey")]
    pub fn create_from_license_key(key: String, domain: String) -> Result<Evaluator, JsValue> {
        let eval = licensor::from_license_key_bytes(key, domain)
            .or_else(map_to_js_err)?;
        Ok(Evaluator{
            eval,
        })
    }

    /// Validates the given license key. Returns true if the license is valid for the given domain
    pub fn validate(&self) -> Result<bool, JsValue> {
        self.eval.validate()
            .map(|_| true)
            .or_else(map_to_js_err)
    }

    /// Returns true if the given license is valid and enables the given feature
    pub fn enabled(&self, feature: String) -> bool {
        let feature = match Feature::try_from(feature.as_str()) {
            Ok(f) => f,
            Err(_) => return false,
        };
        self.eval.enabled(&feature)
    }

    /// Returns true if either:
    ///  - the license has no restrictions on seats
    ///  - the license permits at least the given number of seats
    #[wasm_bindgen(js_name = "hasEnoughSeats")]
    pub fn has_enough_seats(&self, seats: i32) -> bool {
        self.eval.has_enough_seats(seats)
    }

    /// Retrn true if:
    ///  - the license permits the use of prebuilds
    ///  - the accumulated time spent doing prebuilds does not exceed the one defined in the license
    #[wasm_bindgen(js_name = "canUsePrebuild")]
    pub fn can_use_prebuild(&self, total_prebuild_time_spent_seconds: i64) -> bool {
        let total_pbt = Duration::seconds(total_prebuild_time_spent_seconds);
        self.eval.can_use_prebuild(&total_pbt)
    }

    /// Returns a string representation of the license (for debugging purposes only)
    pub fn inspect(&self) -> Result<String, JsValue> {
        licensor::inspect_str(&self.eval)
            .or_else(map_to_js_err)
    }
}

fn map_to_js_err<T, E: std::fmt::Display>(err: E) -> Result<T, JsValue> {
    Err(JsValue::from_str(&format!("{}", err)))
}
