use std::mem;
use crate::node::Node;

#[derive(Debug, PartialEq)]
pub struct BTree {
    root: Node,
    degree: usize,
}

impl BTree {
    pub fn degree(degree: usize) -> Self {
        Self {
            root: Node::empty(),
            degree,
        }
    }

    pub fn order(order: usize) -> Self {
        Self {
            root: Node::empty(),
            degree: order / 2
        }
    }

    pub fn search(&self, key: usize) -> Option<(&Node, usize)> {
        BTree::search_from_node(&self.root, key)
    }

    pub fn insert(&mut self, key:usize) {
        if self.root.keys.len() == self.degree * 2 - 1 {
            let mut new_root = Node::new(
                Vec::<usize>::new(),
                Vec::<Node>::new(),
                false
            );
            mem::swap(&mut new_root, &mut self.root);
            self.root.children.insert(0, new_root);
            BTree::split_child(&mut self.root, 0, self.degree);
        }
        BTree::insert_nonfull(&mut self.root, key, self.degree);

    }

    fn insert_nonfull(node: &mut Node, key: usize, degree: usize) {
        let mut index = node.keys.len();
        while index > 0 && key < node.keys[index - 1] {
            index -= 1;
        }
        if node.leaf {
            node.keys.insert(index, key);
        } else {
            if node.children[index].keys.len() == degree * 2 - 1 {
                BTree::split_child(node, index, degree);
                if key > node.keys[index] {
                    index += 1;
                }
            }
            BTree::insert_nonfull(&mut node.children[index], key, degree);
        }
    }

    fn search_from_node(node: &Node, key: usize) -> Option<(&Node, usize)> {
        let mut index = node.keys.len();
        while index > 0 && key < node.keys[index - 1] {
            index -= 1;
        }
        if index > 0 && key == node.keys[index - 1] {
            Some((&node, index))
        } else if node.leaf {
            None
        } else {
            BTree::search_from_node(node.children.get(index)?, key)
        }
    }

    /// Split a node
    /// # Arguments
    /// * `internal_node` - A nonfull internal (parent) node
    /// * `leaf` - A full leaf (child) node
    fn split_child(internal_node: &mut Node, index: usize, degree: usize) {
        let child = &mut internal_node.children[index];
        let mut new_child = Node::new(
            child.keys.split_off(degree),
            Vec::<Node>::new(),
            child.leaf,
        );
        if !child.leaf { //?
            new_child.children = child.children.split_off(degree);
        }

        let median = child.keys.pop().unwrap();
        internal_node.keys.insert(index, median);
        internal_node.children.insert(index + 1, new_child);
    }
}