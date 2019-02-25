use crate::day_8::types::*;

type NodeIndex = usize;

impl Node {
    /// Creates a new root node from a license, which contains all child nodes
    pub fn new(license: &License) -> Node {
        let (root, _) = Node::get_node(&license, 0);
        root
    }

    /// Gets a node from given starting index (and all of its children, recursively)
    fn get_node(license: &License, node_start_index: NodeIndex) -> (Node, NodeIndex) {
        let mut current_index = node_start_index;
        let num_children = license[current_index];

        current_index += 1; // Index is now at meta entry count
        let num_meta_entries = license[current_index];

        match num_children {
            0 => {
                current_index += 1; // Index is now at the first meta entry
                let (meta_data, current_index) = Node::get_meta_entries(license, current_index, num_meta_entries);
                (Node { children: vec![], meta_data }, current_index)
            },
            non_zero => {
                current_index += 1; // Index is now at start of the next child node
                let mut children = vec![];
                for i in 0..non_zero {
                    let (child, index) = Node::get_node(license, current_index);
                    children.push(child);
                    current_index = index;
                }
                let (meta_data, current_index) = Node::get_meta_entries(license, current_index, num_meta_entries);
                (Node { children, meta_data }, current_index)
            },
        }
    }

    /// Returns the meta entries from given starting index (also returns the next index after the meta entries)
    fn get_meta_entries(license: &License, first_meta_entry_index: NodeIndex, num_meta_entries: usize) -> (Vec<MetaDataEntry>, NodeIndex) {
        let mut meta_entries = Vec::new();
        let mut current_index = first_meta_entry_index;

        for _ in 0..num_meta_entries {
            meta_entries.push(license[current_index]);
            current_index += 1;
        }

        (meta_entries, current_index)
    }
}
