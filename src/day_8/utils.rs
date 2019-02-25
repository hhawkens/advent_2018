use super::types::*;

pub fn get_license(license_text: &str) -> License {
    license_text.split(" ").map(|s| s.parse().unwrap()).collect()
}
