mod node;

pub type LicenseEntry = usize;
pub type MetaDataEntry = LicenseEntry;
pub type License = Vec<LicenseEntry>;

#[derive(Debug)]
pub struct Node {
    children: Vec<Node>,
    meta_data: Vec<MetaDataEntry>
}

#[derive(Debug)]
pub struct Tree {
    pub root: Node
}
