pub type MetaDataEntry = usize;

pub struct Node<'a> {
    pub children: Vec<&'a Node<'a>>,
    pub meta_data: Vec<MetaDataEntry>
}

pub struct Tree<'a> {
    pub root: Node<'a>,
}
