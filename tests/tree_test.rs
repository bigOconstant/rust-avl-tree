extern crate avl_tree;
use avl_tree::Avl_Tree;

#[test]
fn test_depth() {
    let mut a_tree = Avl_Tree::new(100);
    a_tree.insert(99);
    a_tree.insert(98);
    println!("Tree Depth{}", a_tree.get_depth());

    assert_eq!(2, a_tree.get_depth());
    a_tree.delete_left();
    a_tree.delete_right();
}

#[test]
fn test_insert_left() {
    let mut a_tree = Avl_Tree::new(10);
    a_tree.insert(9);
    unsafe {
        assert_eq!((*a_tree.get_root()).get_left().is_null(), false);
    }
}

#[test]
fn test_insert_right() {
    let mut a_tree = Avl_Tree::new(10);
    a_tree.insert(11);
    unsafe {
        assert_eq!((*a_tree.get_root()).get_right().is_null(), false);
    }
}
