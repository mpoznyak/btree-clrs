use std::cell::{Ref, RefCell};
use std::mem;
use std::ops::Deref;
use std::rc::Rc;
use crate::node::Node;

type Link = Option<Rc<RefCell<Node>>>;

#[derive(Debug, PartialEq)]
pub struct BTree {
    root: Link,
    degree: usize,
}

impl BTree {
    pub fn degree(degree: usize) -> Self {
        Self {
            root: Some(Rc::new(RefCell::new(Node::empty()))),
            degree,
        }
    }

    pub fn order(order: usize) -> Self {
        Self {
            root: Some(Rc::new(RefCell::new(Node::empty()))),
            degree: order / 2,
        }
    }

    pub fn search(&self, key: usize) -> Option<(Ref<Node>, usize)> {
        self.root.as_ref().map(|node| {
            BTree::search_from_node(node.borrow(), key)
        }).unwrap()
    }

    pub fn insert(&mut self, key: usize) {
        let mut root = &mut *self.root.as_mut().unwrap().borrow_mut();
        if root.keys.len() == self.degree * 2 - 1 {
            let mut new_root = Node::new(
                Vec::<usize>::new(),
                Vec::<Node>::new(),
                false,
            );
            mem::swap(&mut new_root, &mut root);
            root.children.insert(0, new_root);
            BTree::split_child(&mut root, 0, self.degree);
        }
        BTree::insert_nonfull(&mut root, key, self.degree);
    }

    // pub fn delete(&mut self, key: usize) -> Option<usize> {
    //     return  self.delete_from_node(Rc::new(RefCell::new(&mut self.root)), key);
    // }

    //todo wrap into RC::REFCELL
    fn delete_from_node(&mut self, root: Rc<RefCell<&mut Node>>, key: usize) -> Option<usize> {
        // case 1 leaves deletion
        // case 2 internal node deletion
        // case 3 internal node and the deletion leads to a fewer number of keys than required

        //case 1
        let mut borrowed_root = root.borrow_mut();
        let mut i = 0;
        for root_key in &borrowed_root.keys {
            if key < *root_key {
                break;
            }
            i += 1;
        }
        if borrowed_root.leaf {
            if i < borrowed_root.keys.len() && borrowed_root.keys[i] == key {
                borrowed_root.keys.retain(|item| *item != key);
                return Some(i);
            }
            return None;
        }
        if i < borrowed_root.keys.len() && borrowed_root.keys[i] == key {
            return BTree::delete_internal_node(Rc::clone(&root), key, i);
        } else if borrowed_root.children[i].keys.len() >= self.degree {
            return self.delete_from_node(Rc::new(
                RefCell::new(&mut borrowed_root.children[i])), key);
        } else {
            if i != 0 && i + 2 < borrowed_root.children.len() {
                BTree::delete_sibling(Rc::clone(&root), i, i - 1);
            }
        }
        None
    }

    fn delete_predecessor(&mut self, root: Rc<RefCell<&mut Node>>, degree: usize) -> Option<usize> {
        let borrowed_root = root.borrow();
        if borrowed_root.leaf {
            return root.borrow_mut().keys.pop(); //todo is it correct?
        }
        let last_index = borrowed_root.keys.len() - 1;
        if borrowed_root.children[last_index].keys.len() >= degree {
            BTree::delete_sibling(Rc::clone(&root), last_index + 1, last_index);
        } else {
            self.delete_merge(Rc::clone(&root), last_index, last_index + 1);
        }
        return self.delete_predecessor(Rc::new(RefCell::new(
            &mut root.borrow_mut().children[last_index])), degree);
    }

    fn delete_successor(&mut self, root: Rc<RefCell<&mut Node>>, degree: usize) -> Option<usize> {
        let borrowed_root = root.borrow();
        if borrowed_root.leaf {
            return Some(root.borrow_mut().keys.remove(0));
        }
        if borrowed_root.children[1].keys.len() >= degree {
            BTree::delete_sibling(Rc::clone(&root), 0, 1);
        } else {
            self.delete_merge(Rc::clone(&root), 0, 1);
        }
        return self.delete_successor(Rc::new(
            RefCell::new(
                &mut root.borrow_mut().children[0])), degree);
    }

    fn delete_merge(&mut self, root: Rc<RefCell<&mut Node>>, i: usize, j: usize) {
        let child_node = &mut root.borrow_mut().children[i];
        let mut left_sibling = &mut root.borrow_mut().children[j];
        let mut new: Option<Rc<RefCell<&mut Node>>> = None;
        if j > i {
            let mut right_sibling = &mut root.borrow_mut().children[j];
            child_node.keys.push(root.borrow().keys[i]);
            for k in 0..right_sibling.keys.len() {
                child_node.keys.push(right_sibling.keys[k]); //todo is it correct NOT to  remove?
                if right_sibling.children.len() > 0 {
                    child_node.children.push(right_sibling.children.remove(k)); //todo is it correct to remove?
                }
            }
            if right_sibling.children.len() > 0 {
                child_node.children.push(right_sibling.children.pop().unwrap())
            }
            new = Some(Rc::new(RefCell::new(child_node)));
            let mut borrowed_root = root.borrow_mut();
            borrowed_root.keys.remove(i);
            borrowed_root.children.remove(j);
        } else {
            left_sibling.keys.push(root.borrow().keys[j]);
            for i in 0..child_node.keys.len() {
                left_sibling.keys.push(child_node.keys[i]); //todo is it correct NOT to  remove?
                if left_sibling.children.len() > 0 {
                    left_sibling.children.push(child_node.children.remove(i)) //todo is it correct to remove?
                }
            }
            if left_sibling.children.len() > 0 {
                left_sibling.children.push(child_node.children.pop().unwrap());
            }
            new = Some(Rc::new(RefCell::new(left_sibling)));
            let mut borrowed_root = root.borrow_mut();
            borrowed_root.keys.remove(j);
            borrowed_root.children.remove(i);
        }
        // todo complete resolve
        // if root.is_root && root.borrow().keys.len() == 0 {
        //     mem::swap(&mut self.root, *new.unwrap().borrow_mut());
        // }
    }

    fn delete_sibling(root: Rc<RefCell<&mut Node>>, i: usize, j: usize) {
        let mut child_node = &mut root.borrow_mut().children[i];
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

    fn delete_internal_node(node: Rc<RefCell<&mut Node>>, key: usize, index: usize) -> Option<usize> {
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

    fn search_from_node(node: Ref<Node>, key: usize) -> Option<(Ref<Node>, usize)> {
        let mut index = node.keys.len();
        while index > 0 && key < node.keys[index - 1] {
            index -= 1;
        }
        if index > 0 && key == node.keys[index - 1] {
            Some((node, index))
        } else if node.leaf {
            None
        } else {
            // BTree::search_from_node(node.children.get(index)?, key)
            None
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