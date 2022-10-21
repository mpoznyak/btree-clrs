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
            degree: order / 2,
        }
    }

    pub fn search(&self, key: usize) -> Option<(&Node, usize)> {
        BTree::search_from_node(&self.root, key)
    }

    pub fn insert(&mut self, key: usize) {
        if self.root.keys.len() == self.degree * 2 - 1 {
            let mut new_root = Node::new(
                Vec::<usize>::new(),
                Vec::<Node>::new(),
                false,
            );
            mem::swap(&mut new_root, &mut self.root);
            self.root.children.insert(0, new_root);
            BTree::split_child(&mut self.root, 0, self.degree);
        }
        BTree::insert_nonfull(&mut self.root, key, self.degree);
    }

    pub fn delete(mut self, key: usize) -> Option<usize> {
        return BTree::delete_from_node(&mut self.root, key, self.degree);
    }

    fn delete_from_node(root: &mut Node, key: usize, degree: usize) -> Option<usize> {
        // case 1 leaves deletion
        // case 2 internal node deletion
        // case 3 internal node and the deletion leads to a fewer number of keys than required

        //case 1
        let mut i = 0;
        for root_key in &root.keys {
            if key < *root_key {
                break;
            }
            i += 1;
        }
        if root.leaf {
            if i < root.keys.len() && root.keys[i] == key {
                root.keys.retain(|item| *item != key);
                return Some(i);
            }
            return None;
        }
        if i < root.keys.len() && root.keys[i] == key {
            return BTree::delete_internal_node(root, key, i);
        } else if root.children[i].keys.len() >= degree {
            return BTree::delete_from_node(&mut root.children[i], key, degree);
        } else {

        }
        None
    }

    fn delete_internal_node(node: &mut Node, key: usize, index: usize) -> Option<usize> {
        None
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