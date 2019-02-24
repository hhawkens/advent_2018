pub fn get_license(license_text: &str) -> Vec<u32> {
    license_text.split(" ").map(|s| s.parse().unwrap()).collect()
}
