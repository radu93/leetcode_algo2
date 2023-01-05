struct Solution;

// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }

    pub fn is_equal(&self, other: &Self) -> bool {
        if self.val != other.val {
            return false;
        }

        let mut left_equals = true;

        if let Some(left) = &self.left {
            match &other.left {
                Some(other_left) => {
                    left_equals = TreeNode::is_equal(
                        RefCell::borrow(Rc::borrow(left)).borrow(),
                        RefCell::borrow(Rc::borrow(other_left)).borrow(),
                    );
                }
                None => {
                    return false;
                }
            }
        } else {
            if let Some(other_left) = &other.left {
                left_equals = false;
            }
        }

        let mut right_equals = true;

        if let Some(right) = &self.right {
            match &other.right {
                Some(other_right) => {
                    right_equals = TreeNode::is_equal(
                        RefCell::borrow(Rc::borrow(right)).borrow(),
                        RefCell::borrow(Rc::borrow(other_right)).borrow(),
                    );
                }
                None => {
                    return false;
                }
            }
        } else {
            if let Some(other_right) = &other.right {
                right_equals = false;
            }
        }

        return left_equals && right_equals;
    }

    pub fn dfs_find(&self, f: &dyn Fn(&TreeNode) -> bool) -> bool {
        if f(self) {
            return true;
        }
        if let Some(left_node) = &self.left {
            let tn: &TreeNode = &RefCell::borrow(Rc::borrow(left_node));
            if tn.dfs_find(f) {
                return true;
            }
        }
        if let Some(right_node) = &self.right {
            let tn: &TreeNode = &RefCell::borrow(Rc::borrow(right_node));
            if tn.dfs_find(f) {
                return true;
            }
        }
        false
    }
}

use std::borrow::Borrow;
use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    pub fn is_subtree(
        root: Option<Rc<RefCell<TreeNode>>>,
        sub_root: Option<Rc<RefCell<TreeNode>>>,
    ) -> bool {
        if let Some(sub_root_node) = sub_root {
            if let Some(root_node) = root {
                (*root_node).borrow().dfs_find(&|node| {
                    node.is_equal(&(*sub_root_node).borrow())
                })
            } else {
                false
            }
        } else {
            false
        }
    }
}

fn main() {
    let tree = Some(Rc::new(RefCell::new(TreeNode {
        val: 3,
        left: Some(Rc::new(RefCell::new(TreeNode {
            val: 4,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 1,
                left: None,
                right: None,
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 2,
                left: None,
                right: None,
            }))),
        }))),
        right: Some(Rc::new(RefCell::new(TreeNode {
            val: 5,
            left: None,
            right: None,
        }))),
    })));

    let subtree = Some(Rc::new(RefCell::new(TreeNode {
        val: 4,
        left: Some(Rc::new(RefCell::new(TreeNode {
            val: 1,
            left: None,
            right: None,
        }))),
        right: Some(Rc::new(RefCell::new(TreeNode {
            val: 2,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 1,
                left: None,
                right: None,
            }))),
            right: None,
        }))),
    })));

    println!("{}", Solution::is_subtree(tree, subtree));
}
