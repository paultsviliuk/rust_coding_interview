/* 
A path in a binary tree is defined as:

A sequence of nodes such that each pair of adjacent nodes must have an edge connecting them. 
A node can only be included in a path at most once. Moreover, including the root in the path is not compulsory.

Steps to Implement and Test the max_path_sum Function

   * Define the TreeNode structure.
   * Implement the max_path_sum and max_contrib functions.
   * Write a main function to create a tree and test the max_path_sum function.

   max_contrib function calculates the maximum contribution of each node 
   and updates the max variable if a new maximum path sum is found.

*/

use std::rc::Rc;
use std::cell::RefCell;
use std::cmp;

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

fn max_path_sum(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    let mut max = i32::MIN;
    max_contrib(root.as_ref(), &mut max);
    max
}

fn max_contrib(root: Option<&Rc<RefCell<TreeNode>>>, max: &mut i32) -> i32 {
    if let Some(node) = root {
        // Recursive call to get the maximum contribution from the left subtree
        let left = max_contrib(node.borrow().left.as_ref(), max);
        
        // Recursive call to get the maximum contribution from the right subtree
        let right = max_contrib(node.borrow().right.as_ref(), max);
        
        // Update the maximum path sum including the current node and both children
        // The sum of the current node's value and the greater of the left and right contributions is checked
        *max = cmp::max(
            node.borrow().val + cmp::max(left, 0) + cmp::max(right, 0),
            *max,
        );
        
        // Return the maximum contribution from this node to the path sum
        // We can only include one of the left or right subtrees in the path we return upwards
        node.borrow().val + cmp::max(cmp::max(left, right), 0)
    } else {
        0
    }
}


fn main() {
    let mut root = TreeNode::new(-8);
    root.left = Some(Rc::new(RefCell::new(TreeNode::new(2))));
    root.right = Some(Rc::new(RefCell::new(TreeNode::new(17))));
    root.left.clone().unwrap().borrow_mut().left = Some(Rc::new(RefCell::new(TreeNode::new(1))));
    root.left.clone().unwrap().borrow_mut().right = Some(Rc::new(RefCell::new(TreeNode::new(4))));
    root.right.clone().unwrap().borrow_mut().left = Some(Rc::new(RefCell::new(TreeNode::new(19))));
    root.right.clone().unwrap().borrow_mut().right = Some(Rc::new(RefCell::new(TreeNode::new(5))));

    let res = max_path_sum(Some(Rc::new(RefCell::new(root))));
    println!("Maximum Path Sum: {}", res);
}
