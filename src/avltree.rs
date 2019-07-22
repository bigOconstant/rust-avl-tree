use rand::{self, Rng};
use std::mem;
use std::ptr;
use crate::node;

pub struct AvlTree {
    root: *mut node::Node,
}

impl AvlTree {
    pub fn new(d: i32) ->AvlTree {
       AvlTree {
            root: Box::into_raw(Box::new(node::Node::new(d))),
        }
    }

    pub fn get_root(&self) -> *mut node::Node {
        return self.root;
    }

    pub fn insert(&mut self, n: i32) {
        unsafe {
            if !self.root.is_null() {
                (*self.root).insert(n);
            } else {
                panic!("tree not initialised. Try callingAvlTree::new(i32)");
            }
        }
    }

    pub fn is_balanced(&self) -> bool {
        unsafe {
            if !self.root.is_null() {
                return (*self.root).is_balanced();
            } else {
                panic!("tree not initialised. Try callingAvlTree::new(i32)");
                //return false;
            }
        }
    }
    pub fn get_depth(&self) -> i32 {
        unsafe {
            if !self.root.is_null() {
                return (*self.root).get_depth();
            } else {
                return 0;
            }
        }
    }

    pub fn print_tree(&self) {
        unsafe {
            if !self.root.is_null() {
                (*self.root).print_tree();
            }
        }
    }

    pub fn delete_left(&mut self) {
        unsafe {
            if !self.root.is_null() {
                (*self.root).delete_left();
            }
        }
    }

    pub fn delete_right(&mut self) {
        unsafe {
            if !self.root.is_null() {
                (*self.root).delete_right();
            }
        }
    }
}