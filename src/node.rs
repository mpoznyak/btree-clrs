use std::ops::Deref;

///BTree node
/// # Fields
/// * `keys` 2t - 1 keys
/// * `children` 2t
/// * `leaf` - leaf attribute
#[derive(Debug, PartialEq)]
pub struct Node {
    pub keys: Vec<usize>,
    pub children: Vec<Node>,
    pub leaf: bool,
}

impl Node {
    pub fn empty() -> Self {
        Self {
            keys: Vec::<usize>::new(),
            children: Vec::<Node>::new(),
            leaf: true,
        }
    }
    pub fn new(keys: Vec<usize>,
               children: Vec<Node>,
               leaf: bool) -> Self {
        Self {
            keys,
            children,
            leaf,
        }
    }
}