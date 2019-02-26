use crate::day_8::types::*;
use std::collections::VecDeque;

type NodeIndex = usize;

impl Node {
    /// Creates a new root node from a license, which contains all child nodes
    pub fn new(license: &[LicenseEntry]) -> Node {
        let (root, _) = Node::get_node(&license, 0);
        root
    }

    /// Gets a node from given starting index (and all of its children, recursively)
    fn get_node(license: &[LicenseEntry], node_start_index: NodeIndex) -> (Node, NodeIndex) {
        let mut current_index = node_start_index;
        let num_children = license[current_index];

        current_index += 1; // Index is now at meta entry count
        let num_meta_entries = license[current_index];

        current_index += 1; // Index is now at start of [the next child node] OR [meta data]
        match num_children {
            0 => {
                let (meta_data, current_index) =
                    Self::get_meta_entries(license, current_index, num_meta_entries);
                (
                    Node {
                        children: vec![],
                        meta_data,
                    },
                    current_index,
                )
            }
            non_zero => {
                let mut children = vec![];
                for _ in 0..non_zero {
                    let (child, index) = Self::get_node(license, current_index);
                    children.push(child);
                    current_index = index;
                }
                let (meta_data, current_index) =
                    Self::get_meta_entries(license, current_index, num_meta_entries);
                (
                    Node {
                        children,
                        meta_data,
                    },
                    current_index,
                )
            }
        }
    }

    /// Returns the meta entries from given starting index (also returns the next index after the meta entries)
    fn get_meta_entries(
        license: &[LicenseEntry],
        first_meta_entry_index: NodeIndex,
        num_meta_entries: usize,
    ) -> (Vec<MetaDataEntry>, NodeIndex) {
        let mut meta_entries = Vec::new();
        let mut current_index = first_meta_entry_index;

        for _ in 0..num_meta_entries {
            meta_entries.push(license[current_index]);
            current_index += 1;
        }

        (meta_entries, current_index)
    }

    fn has_children(&self) -> bool {
        !self.children.is_empty()
    }

    fn get_node_value(&self) -> MetaDataEntry {
        if self.has_children() {
            let mut child_meta_entries = 0;

            for mut child_index in self.meta_data.iter().cloned() {
                child_index -= 1; // This is done because indexing of the children starts at 1
                if child_index < self.children.len() {
                    child_meta_entries += self.children[child_index].get_node_value();
                }
            }
            child_meta_entries
        } else {
            self.meta_data.iter().sum()
        }
    }
}

/////////////////////////////////////////////////////   ITERATION   /////////////////////////////////////////////////////
pub struct RefNodeIterator<'a> {
    stack: VecDeque<&'a Node>,
}

impl<'a> Iterator for RefNodeIterator<'a> {
    type Item = &'a Node;

    fn next(&mut self) -> Option<Self::Item> {
        let next = self.stack.pop_back();
        if let Some(node) = next {
            for child in &node.children {
                self.stack.push_front(child);
            }
        }
        next
    }
}

impl<'a> IntoIterator for &'a Node {
    type Item = Self;
    type IntoIter = RefNodeIterator<'a>;

    fn into_iter(self) -> Self::IntoIter {
        let mut iter = RefNodeIterator {
            stack: VecDeque::new(),
        };
        iter.stack.push_front(self);
        iter
    }
}

/////////////////////////////////////////////////////   TREE   /////////////////////////////////////////////////////
impl Tree {
    /// Gets the sum of all meta data entries of all nodes
    pub fn meta_data_sum(&self) -> MetaDataEntry {
        self.root
            .into_iter()
            .map(|node| node.meta_data.iter().sum::<MetaDataEntry>())
            .sum()
    }

    /// Calculates the value of the root node
    pub fn root_node_value(&self) -> MetaDataEntry {
        self.root.get_node_value()
    }
}
