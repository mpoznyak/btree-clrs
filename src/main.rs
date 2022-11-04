use crate::btree::BTree;

mod node;
mod btree;

fn main() {
    let mut btree = BTree::degree(3);
    fill_btree(&mut btree);
    println!("{:?}", btree);
    let ket = btree.search(3);
    println!("{}", ket.unwrap().borrow().keys[0])
}
fn fill_btree(btree: &mut BTree) {
    btree.insert(1);
    btree.insert(2);
    btree.insert(3);
    btree.insert(4);
    btree.insert(5);
    btree.insert(6);
    btree.insert(7);
    btree.insert(8);
    btree.insert(9);
    btree.insert(10);
    btree.insert(11);
    btree.insert(12);
    btree.insert(13);
    btree.insert(14);
    btree.insert(15);
    btree.insert(16);
    btree.insert(17);
    btree.insert(18);
    btree.insert(19);
}

#[cfg(test)]
mod test {
    use rand::Rng;
    use crate::BTree;

    #[test]
    fn test_btree_search() {
        let mut btree = BTree::degree(3);
        crate::fill_btree(&mut btree);
        let binding1 = btree.search(12).unwrap();
        let search_result = binding1.borrow();
        assert_eq!(search_result.keys.len(), 2);
        assert_eq!(search_result.keys[0], 12);
        assert_eq!(search_result.keys[1], 15);
        assert_eq!(search_result.children.len(), 3);
        assert_eq!(search_result.children[0].borrow().keys[0], 10);
        assert_eq!(search_result.children[0].borrow().keys[1], 11);
        assert_eq!(search_result.children[1].borrow().keys[0], 13);
        assert_eq!(search_result.children[1].borrow().keys[1], 14);
        assert_eq!(search_result.children[2].borrow().keys[0], 16);
        assert_eq!(search_result.children[2].borrow().keys[1], 17);
        assert_eq!(search_result.children[2].borrow().keys[2], 18);
        assert_eq!(search_result.children[2].borrow().keys[3], 19);

        let binding1 = btree.search(6).unwrap();
        let search_result1 = binding1.borrow();
        assert_eq!(search_result1.keys.len(), 2);
        assert_eq!(search_result1.children.len(), 3);
        assert_eq!(search_result1.children[0].borrow().keys[0], 1);
        assert_eq!(search_result1.children[0].borrow().keys[1], 2);

        let binding2 = btree.search(1).unwrap();
        let search_result2 = binding2.borrow();
        assert_eq!(search_result2.keys.len(), 2);
        assert_eq!(search_result2.leaf, true);

        let mut btree_with_order = BTree::order(6);
        crate::fill_btree(&mut btree_with_order);
        assert_eq!(btree, btree_with_order);
    }
    #[test]
    fn test_random_btree_filling() {
        use rand::Rng;

        let mut btree = BTree::order(9);
        let mut rng = rand::thread_rng();
        let mut count = 0;
        loop {
            if count > 99 {
                break;
            }
            count += 1;
            btree.insert(rng.gen_range(0..100));
        }
        btree.insert(101);
        let node = btree.search(101).unwrap();
        assert_eq!(node.borrow().leaf, true);
    }

    // #[test]
    // fn test_btree_delete() {
    //     let mut btree = BTree::degree(3);
    //     crate::fill_btree(&mut btree);
    //     btree.delete(19);
    //     println!("{}", 1)
    // }




}

