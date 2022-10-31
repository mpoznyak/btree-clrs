use std::cell::RefCell;
use std::mem;
use std::rc::Rc;
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
            if i != 0 && i + 2 < root.children.len() {
                let ref_node = Rc::new(RefCell::new(root));
                BTree::delete_sibling(ref_node, i, i - 1);
            }
        }
        None
    }

    fn delete_predecessor(root: Rc<RefCell<&mut Node>>, degree: usize) -> Option<usize> {
        let borrowed_root = root.borrow();
        if borrowed_root.leaf {
            return root.borrow_mut().keys.pop(); //todo is it correct?
        }
        let last_index = borrowed_root.keys.len() - 1;
        if borrowed_root.children[last_index].keys.len() >= degree {
            BTree::delete_sibling(Rc::clone(&root), last_index + 1, last_index);
        } else {
            BTree::delete_merge(Rc::clone(&root), last_index, last_index + 1);
        }
        return BTree::delete_predecessor(Rc::new(RefCell::new(
            &mut root.borrow_mut().children[last_index])), degree);
    }

    fn delete_successor(root: Rc<RefCell<&mut Node>>, degree: usize) -> Option<usize> {
        let borrowed_root = root.borrow();
        if borrowed_root.leaf {
            return Some(root.borrow_mut().keys.remove(0));
        }
        if borrowed_root.children[1].keys.len() >= degree {
            BTree::delete_sibling(Rc::clone(&root), 0, 1);
        } else {
            BTree::delete_merge(Rc::clone(&root), 0, 1);
        }
        return BTree::delete_successor(Rc::new(
            RefCell::new(
                &mut root.borrow_mut().children[0])), degree);
    }

    fn delete_merge(root: Rc<RefCell<&mut Node>>, i: usize, j: usize) -> Option<usize> {
        unimplemented!()
    }

    fn delete_sibling(root: Rc<RefCell<&mut Node>>, i: usize, j: usize) {
        let mut child_node = &mut root.borrow_mut().children[i]; //todo borrow or borrow_mut?
        if i < j {
            let right_sibling = &mut root.borrow_mut().children[j];
            child_node.keys.push(root.borrow().keys[i]);
            let first_r_key = right_sibling.keys[0];
            root.borrow_mut().keys[i] = first_r_key;
            if right_sibling.children.len() > 0 {
                let first_r_sib = right_sibling.children.remove(0);
                child_node.children.push(first_r_sib);
            }
        } else {
            let mut left_sibling = &mut root.borrow_mut().children[j];
            child_node.keys.insert(0, root.borrow_mut().keys.remove(i - 1));
            root.borrow_mut().keys[i - 1] = left_sibling.keys.pop().unwrap();
            if left_sibling.children.len() > 0 {
                child_node.children.insert(0, left_sibling.children.pop().unwrap())
            }
        }
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