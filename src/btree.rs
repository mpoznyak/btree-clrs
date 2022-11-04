use std::cell::{Ref, RefCell, RefMut};
use std::mem;
use std::ops::{Deref, DerefMut};
use std::rc::Rc;
use crate::node::Node;

type Link = Rc<RefCell<Node>>;

#[derive(Debug, PartialEq)]
pub struct BTree {
    root: Rc<RefCell<Node>>,
    degree: usize,
}

impl BTree {
    pub fn degree(degree: usize) -> Self {
        Self {
            root: Rc::new(RefCell::new(Node::empty())),
            degree,
        }
    }

    pub fn order(order: usize) -> Self {
        Self {
            root: Rc::new(RefCell::new(Node::empty())),
            degree: order / 2,
        }
    }

    /// Search for the node which contains a key
    pub fn search(&self, key: usize) -> Option<Rc<RefCell<Node>>> {
        let mut next = &self.root;
        let mut index = next.borrow().keys.len();
        while index > 0 && key < next.borrow().keys[index - 1] {
            index -= 1;
        }
        let mut result = None;
        if index > 0 && key == next.borrow().keys[index - 1] {
            result = Some(next.clone());
        } else if next.borrow().leaf {
            return None;
        } else {
            let mut search_index: usize = index;
            let mut cloned = self.root.clone();
            let mut stack = vec![cloned];
            while let Some(node) = stack.pop() {
                let borrowed = node.borrow();
                let child = borrowed.children.get(search_index).unwrap();
                println!("index-{}, {:?}", search_index, child);
                let mut index = child.borrow().keys.len();
                while index > 0 && key < child.borrow().keys[index - 1] {
                    index -= 1;
                }
                if index > 0 && key == child.borrow().keys[index - 1] {
                    result = Some(child.clone());
                    break;
                } else if child.borrow().leaf {
                    return None;
                } else {
                    search_index = index;
                    stack.push(child.clone())
                }
            }
        }
        result
    }

    fn wrap(elem: Node) -> Rc<RefCell<Node>> {
        Rc::new(
            RefCell::new(
                elem
            )
        )
    }

    pub fn insert(&mut self, key: usize) {
        let mut root = &mut *self.root.borrow_mut();
        if root.keys.len() == self.degree * 2 - 1 {
            let mut new_root = Node::new(
                Vec::<usize>::new(),
                Vec::<Rc<RefCell<Node>>>::new(),
                false,
            );
            mem::swap(&mut new_root, &mut root);
            root.children.insert(0, BTree::wrap(new_root));
            BTree::split_child(&mut root, 0, self.degree);
        }
        BTree::insert_nonfull(&mut root, key, self.degree);
    }

    pub fn delete(&mut self, key: usize) {
        let binding = self.root.clone();
        let temp = &mut binding.borrow_mut();
        self.delete_from_node(Rc::new(RefCell::new(temp)), key);
    }

    //todo wrap into RC::REFCELL
    fn delete_from_node(&mut self, root: Rc<RefCell<&mut Node>>, key: usize) {
        // case 1 leaves deletion
        // case 2 internal node deletion
        // case 3 internal node and the deletion leads to a fewer number of keys than required

        //case 1
        let mut cloned_root = Rc::clone(&root);
        let mut i = 0;
        for root_key in &cloned_root.borrow().keys {
            if key < *root_key {
                break;
            }
            i += 1;
        }
        if cloned_root.borrow().leaf {
            if i < cloned_root.borrow().keys.len() && cloned_root.borrow().keys[i] == key {
                root.borrow_mut().keys.retain(|item| *item != key);
                return;
            }
            return;
        }
        if i < cloned_root.borrow().keys.len() && cloned_root.borrow().keys[i] == key {
            self.delete_internal_node(Rc::clone(&root), key, i);
        } else if cloned_root.borrow().children[i].borrow().keys.len() >= self.degree {
            self.delete_from_node(Rc::new(
                RefCell::new(&mut cloned_root.borrow().children[i].borrow_mut())), key);
        } else {
            if i != 0 && i + 2 < cloned_root.borrow().children.len() {
                if cloned_root.borrow().children[i - 1].borrow().keys.len() >= self.degree {
                    BTree::delete_sibling(Rc::clone(&root), i, i - 1);
                } else if cloned_root.borrow().children[i + 1].borrow().keys.len() >= self.degree {
                    BTree::delete_sibling(Rc::clone(&root), i, i + 1);
                } else {
                    self.delete_merge(Rc::clone(&root), i, i + 1)
                }
            } else if i == 0 {
                if cloned_root.borrow().children[i + 1].borrow().keys.len() >= self.degree {
                    BTree::delete_sibling(Rc::clone(&root), i, i + 1);
                } else {
                    self.delete_merge(Rc::clone(&root), i, i + 1);
                }
            } else if i + 1 == cloned_root.borrow().children.len() {
                if cloned_root.borrow().children[i - 1].borrow().keys.len() >= self.degree {
                    BTree::delete_sibling(Rc::clone(&root), i, i - 1);
                } else {
                    self.delete_merge(root, i, i - 1);
                }
            }
            let binding = self.root.clone();
            let binding1 = binding.borrow();
            let temp = &mut binding1.children[i].borrow_mut();
            return self.delete_from_node(Rc::new(RefCell::new(temp)), key);
        }
    }

    fn delete_predecessor(&mut self, root: Rc<RefCell<&mut Node>>) -> Option<usize> {
        let borrowed_root = root.borrow();
        if borrowed_root.leaf {
            return root.borrow_mut().keys.pop(); //todo is it correct?
        }
        let last_index = borrowed_root.keys.len() - 1;
        if borrowed_root.children[last_index].borrow().keys.len() >= self.degree {
            BTree::delete_sibling(Rc::clone(&root), last_index + 1, last_index);
        } else {
            self.delete_merge(Rc::clone(&root), last_index, last_index + 1);
        }
        return self.delete_predecessor(Rc::new(RefCell::new(
            &mut root.borrow().children[last_index].borrow_mut())));
    }

    fn delete_successor(&mut self, root: Rc<RefCell<&mut Node>>) -> Option<usize> {
        let borrowed_root = root.borrow();
        if borrowed_root.leaf {
            return Some(root.borrow_mut().keys.remove(0));
        }
        if borrowed_root.children[1].borrow().keys.len() >= self.degree {
            BTree::delete_sibling(Rc::clone(&root), 0, 1);
        } else {
            self.delete_merge(Rc::clone(&root), 0, 1);
        }
        return self.delete_successor(Rc::new(
            RefCell::new(
                &mut root.borrow_mut().children[0].borrow_mut())));
    }

    fn delete_merge(&mut self, root: Rc<RefCell<&mut Node>>, i: usize, j: usize) {
        let child_node = &mut root.borrow_mut().children[i].clone();
        let mut left_sibling = &mut root.borrow_mut().children[j].clone();
        let mut new: &mut Rc<RefCell<Node>>;
        let mut removal_index: usize;
        if j > i {
            let mut right_sibling = &mut root.borrow_mut().children[j];
            child_node.borrow_mut().keys.push(root.borrow().keys[i]);
            for k in 0..right_sibling.borrow().keys.len() {
                child_node.borrow_mut().keys.push(right_sibling.borrow().keys[k]); //todo is it correct NOT to  remove?
                if right_sibling.borrow().children.len() > 0 {
                    child_node.borrow_mut().children.push(right_sibling.borrow_mut().children.remove(k)); //todo is it correct to remove?
                }
            }
            if right_sibling.borrow().children.len() > 0 {
                child_node.borrow_mut().children.push(right_sibling.borrow_mut().children.pop().unwrap())
            }
            new = child_node;
            let mut borrowed_root = root.borrow_mut();
            borrowed_root.keys.remove(i);
            borrowed_root.children.remove(j);
            removal_index = i;
        } else {
            let mut child_borrowed = child_node.borrow_mut();
            left_sibling.borrow_mut().keys.push(root.borrow().keys[j]);
            for i in 0..child_borrowed.keys.len() {
                left_sibling.borrow_mut().keys.push(child_borrowed.keys[i]); //todo is it correct NOT to  remove?
                if left_sibling.borrow_mut().children.len() > 0 {
                    left_sibling.borrow_mut().children.push(child_borrowed.children.remove(i)) //todo is it correct to remove?
                }
            }
            if left_sibling.borrow().children.len() > 0 {
                left_sibling.borrow_mut().children.push(child_borrowed.children.pop().unwrap());
            }
            new = left_sibling;
            let mut borrowed_root = root.borrow_mut();
            borrowed_root.keys.remove(j);
            borrowed_root.children.remove(i);
            removal_index = j;
        }
        // todo complete resolve root == self.root
        if root.borrow().keys.len() == 0 {
            let a = self.root.clone();
            mem::swap(a.borrow_mut().deref_mut(), &mut new.deref_mut().borrow_mut());
            self.root.borrow_mut().children.remove(removal_index);
        }
    }

    fn delete_sibling(root: Rc<RefCell<&mut Node>>, i: usize, j: usize) {
        let mut child_node = &mut root.borrow_mut().children[i];
        if i < j {
            let right_sibling = &mut root.borrow_mut().children[j];
            child_node.borrow_mut().keys.push(root.borrow().keys[i]);
            let first_r_key = right_sibling.borrow().keys[0];
            root.borrow_mut().keys[i] = first_r_key;
            if right_sibling.borrow().children.len() > 0 {
                let first_r_sib = right_sibling.borrow_mut().children.remove(0);
                child_node.borrow_mut().children.push(first_r_sib);
            }
        } else {
            let mut left_sibling = &mut root.borrow_mut().children[j];
            child_node.borrow_mut().keys.insert(0, root.borrow_mut().keys.remove(i - 1));
            root.borrow_mut().keys[i - 1] = left_sibling.borrow_mut().keys.pop().unwrap();
            if left_sibling.borrow().children.len() > 0 {
                child_node.borrow_mut().children.insert(0, left_sibling.borrow_mut().children.pop().unwrap())
            }
        }
    }

    fn delete_internal_node(&mut self, node: Rc<RefCell<&mut Node>>, key: usize, index: usize) {
        if node.borrow().leaf {
            if node.borrow().keys[index] == key {
                node.borrow_mut().keys.remove(index);
            }
            return;
        }
        if node.borrow().children[index].borrow().keys.len() >= self.degree {
            let binding = self.root.clone();
            let binding1 = binding.borrow();
            let temp = &mut binding1.children[index].borrow_mut();
            let pred_key = self.delete_predecessor(Rc::new(RefCell::new(temp)));
            node.clone().borrow_mut().keys[index] = pred_key.unwrap();
            return;
        } else if node.borrow().children[index + 1].borrow().keys.len() >= self.degree {
            let binding = self.root.clone();
            let binding1 = binding.borrow();
            let temp = &mut binding1.children[index + 1].borrow_mut();
            node.borrow_mut().keys[index] = self
                                        .delete_successor(Rc::new(RefCell::new(temp)))
                                        .unwrap();
            return;
        } else {
            let binding = self.root.clone();
            self.delete_merge(Rc::new(RefCell::new(&mut binding.borrow_mut())), index, index + 1);
            self.delete_internal_node(node.clone(), key, index - 1); //todo self.degree - 1 ?
        }
    }

    fn insert_nonfull(node: &mut Node, key: usize, degree: usize) {
        let mut index = node.keys.len();
        while index > 0 && key < node.keys[index - 1] {
            index -= 1;
        }
        if node.leaf {
            node.keys.insert(index, key);
        } else {
            if node.children[index].borrow().keys.len() == degree * 2 - 1 {
                BTree::split_child(node, index, degree);
                if key > node.keys[index] {
                    index += 1;
                }
            }
            BTree::insert_nonfull(&mut node.children[index].borrow_mut(), key, degree);
        }
    }

    /// Split a node
    /// # Arguments
    /// * `internal_node` - A nonfull internal (parent) node
    /// * `leaf` - A full leaf (child) node
    fn split_child(internal_node: &mut Node, index: usize, degree: usize) {
        let binding = Rc::clone(&internal_node.children[index]);
        let child = &mut binding.borrow_mut();
        let mut new_child = Node::new(
            child.keys.split_off(degree),
            Vec::<Rc<RefCell<Node>>>::new(),
            child.leaf,
        );
        if !child.leaf { //?
            new_child.children = child.children.split_off(degree);
        }

        let median = child.keys.pop().unwrap();
        internal_node.keys.insert(index, median);
        internal_node.children.insert(index + 1, BTree::wrap(new_child));
    }
}