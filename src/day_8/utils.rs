use super::types::*;

pub fn get_license_tree(license_text: &str) -> Tree {
    let license = get_license(license_text);
    Tree { root: Node::new(&license) }
}

fn get_license(license_text: &str) -> License {
    license_text.split(" ").map(|s| s.parse().unwrap()).collect()
}
