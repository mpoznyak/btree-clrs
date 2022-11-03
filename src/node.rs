use std::cell::RefCell;
use std::ops::Deref;
use std::rc::Rc;

///BTree node
/// # Fields
/// * `keys` 2t - 1 keys
/// * `children` 2t
/// * `leaf` - leaf attribute
#[derive(Debug, PartialEq)]
pub struct Node {
    pub keys: Vec<usize>,
    pub children: Vec<Rc<RefCell<Node>>>,
    pub leaf: bool,
}

impl Node {
    pub fn empty() -> Self {
        Self {
            keys: Vec::<usize>::new(),
            children: Vec::<Rc<RefCell<Node>>>::new(),
            leaf: true,
        }
    }
    pub fn new(keys: Vec<usize>,
               children: Vec<Rc<RefCell<Node>>>,
               leaf: bool) -> Self {
        Self {
            keys,
            children,
            leaf,
        }
    }
}