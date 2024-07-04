/*
The task is to invert give binary tree T.

Let's solve this problem using depth-first search.

Explanation of the DFS algorith:
1. Base case: current node None -> we return None.
2. Recursive case:
    * recursively invert left subtree
    * recursively invert right subtree
    * swap the inverted left and right subtrees
3. Return the root node after inverting its subtrees.

Example:
     1
   /   \
  2     3
 / \   / \
4   5 6   7

Start at the root node (1). Swap its left (2) and right (3) children.
Recursively invert the left subtree (which was initially the right subtree).

   * Invert subtree rooted at 3: Swap its left (6) and right (7) children.
   * Recursively invert the subtrees rooted at 6 and 7, which are leaves and return as is.

Recursively invert the right subtree (which was initially the left subtree).

   * Invert subtree rooted at 2: Swap its left (4) and right (5) children.
   * Recursively invert the subtrees rooted at 4 and 5, which are leaves and return as is.

Result:
     1
   /   \
  3     2
 / \   / \
7   6 5   4
*/

use std::rc::Rc;
use std::cell::RefCell;
use std::collections::VecDeque;

#[derive(Debug, PartialEq, Eq, Clone)]
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

fn invert_tree(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
    match &root {
        Some(node) => {
            let left = invert_tree(node.borrow().left.clone());
            let right = invert_tree(node.borrow().right.clone());
            node.borrow_mut().left = right;
            node.borrow_mut().right = left;
        }, 
        None => {}
    };
    root
}

fn main() {
    let mut root = TreeNode::new(1);
    root.left = Some(Rc::new(RefCell::new(TreeNode::new(2))));
    root.right = Some(Rc::new(RefCell::new(TreeNode::new(3))));
    root.left.clone().unwrap().borrow_mut().left = Some(Rc::new(RefCell::new(TreeNode::new(4))));
    root.left.clone().unwrap().borrow_mut().right = Some(Rc::new(RefCell::new(TreeNode::new(5))));
    root.right.clone().unwrap().borrow_mut().left = Some(Rc::new(RefCell::new(TreeNode::new(6))));
    root.right.clone().unwrap().borrow_mut().right = Some(Rc::new(RefCell::new(TreeNode::new(7))));

    let res = invert_tree(Some(Rc::new(RefCell::new(root))));
    println!("{:?}", res);
}
