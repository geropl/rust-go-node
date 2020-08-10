use rsa::RSAPublicKey;
use rsa::pem;

lazy_static!{
    static ref KEYS: Vec<&'static str> = vec![
        // Demo key - remove before publishing this code
        "-----BEGIN PUBLIC KEY-----
    MIIBCgKCAQEAtHhBNA9J7mh301CMP4Hfvv0OLMWDG3FjwR9nUAg3z5SFYnUz4tnP
    NB7gDFNXUIUpetKUyyoAwAWwQsu4/zt9XDg6G25jiHZ/inEfI3xQV2tUhJm+zVLg
    7RCUpVjbUZthaIhGyYm0Oa/Lqa8q/hInqP/Hlvgga+yfBurrYyhdaJFWpgF/m2ha
    yFgEEE/427F/BP/qNfJN+v/ojtsJMM81/jGWH6Tm0bxoWa5nQPsGF7h0MjLc5pYp
    NOrioO8lNSNu1Fz8cYwATxmdgA+0scS/pXyNcP1U9ELjpUAXaUdhthViQ4d5hXj2
    48DoltWJYg1Vgjj2eeYKr7JiJjrXlZoaFwIDAQAB
    -----END PUBLIC KEY-----",
        // TOOD: add trial license key here
        // TODO: add actual production license key here
    ];

    pub static ref PUBLIC_KEYS: Vec<RSAPublicKey> = KEYS.iter().map(|key| {
        let pem = pem::parse(key.as_bytes())
            .expect("invalid public licensor key");
        if pem.tag != "PUBLIC KEY" {
            panic!("unknown PEM block type {}", pem.tag);
        }
        RSAPublicKey::from_pkcs1(&pem.contents)
            .expect("unable to parse public key")
    }).collect();
}

/// The following is used for testing purposes only
#[cfg(test)]
use std::sync::{Arc, Mutex};

#[cfg(test)]
lazy_static!{
    pub static ref PUBLIC_KEYS_TEST: Arc<Mutex<Vec<RSAPublicKey>>> = Arc::new(Mutex::new(vec![]));
}