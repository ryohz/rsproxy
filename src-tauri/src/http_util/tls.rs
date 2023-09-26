use rcgen::generate_simple_self_signed;

use super::error::HttpUtilError;

pub async fn generate_certificate() -> Result<(), HttpUtilError> {
    let subject_alt_names = vec!["localhost:8080".to_string(), "127.0.0.1:8080".to_string()];
    let cert = generate_simple_self_signed(subject_alt_names).unwrap();
    println!("{}", cert.serialize_pem().unwrap());
    println!("{}", cert.serialize_private_key_pem());
    Ok(())
}

// openssl req -newkey rsa:2048 -nodes -keyout key.pem -x509 -days 365 -out cert.pem
