/*
A binary tree, T, is flip equivalent to another binary tree, S, 
if we can make T equal to S after some number of flip operations.


Recursive Approach

To determine if two trees are flip equivalent, we can use a recursive approach. Here's the strategy:

Base Cases:
   * If both trees are None, they are equivalent.
   * If one tree is None and the other is not, they are not equivalent.
   * If the values of the root nodes of the two trees are different, they are not equivalent.
Recursive Case:
   * We recursively check two possible ways to match the children of the two trees:
        1.Compare the left subtree of root1 with the left subtree of root2 and the right subtree of root1 with the right subtree of root2.
        2.Compare the left subtree of root1 with the right subtree of root2 and the right subtree of root1 with the left subtree of root2.
   * The trees are flip equivalent if either of these two comparisons is true.


*/

use std::rc::Rc;
use std::cell::RefCell;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct TreeNode {
  pub val: i32,
  pub left: Option<Rc<RefCell<TreeNode>>>,
  pub right: Option<Rc<RefCell<TreeNode>>>
}

impl TreeNode {
  pub fn new(val: i32) ->Self { 
    TreeNode {
      val,
      left: None,
      right: None
    }
  }
}

fn flip_equiv(root1: Option<Rc<RefCell<TreeNode>>>, root2: Option<Rc<RefCell<TreeNode>>>) -> bool {
    match (root1, root2) {
        // Case 1: Both nodes are None
        (None, None) => true,
        
        // Case 2: One of the nodes is None
        (None, Some(_)) | (Some(_), None) => false,
        
        // Case 3: Both nodes are Some
        (Some(node1), Some(node2)) => {
            let (node1, node2) = (node1.borrow(), node2.borrow());
            
            // Check if the current nodes' values are the same
            node1.val == node2.val &&
            (
                // Option 1: Compare left with left and right with right
                flip_equiv(node1.left.clone(), node2.left.clone()) && flip_equiv(node1.right.clone(), node2.right.clone()) ||
                // Option 2: Compare left with right and right with left
                flip_equiv(node1.left.clone(), node2.right.clone()) && flip_equiv(node1.right.clone(), node2.left.clone())
            )
        },
    }
}

fn main() {
    let mut root = TreeNode::new(0);
    let mut root1 = TreeNode::new(0);
    let res = flip_equiv(Some(Rc::new(RefCell::new(root))),Some(Rc::new(RefCell::new(root1))));
    println!("{:}",res);

    let mut root = TreeNode::new(1);
    let mut root1 = TreeNode::new(1);
    root.left = Some(Rc::new(RefCell::new(TreeNode::new(2))));
    root.right = Some(Rc::new(RefCell::new(TreeNode::new(3))));
    root.left.clone().unwrap().borrow_mut().left = Some(Rc::new(RefCell::new(TreeNode::new(4))));
    root.left.clone().unwrap().borrow_mut().right = Some(Rc::new(RefCell::new(TreeNode::new(5))));
    root.right.clone().unwrap().borrow_mut().left = Some(Rc::new(RefCell::new(TreeNode::new(6))));
    root.right.clone().unwrap().borrow_mut().right = Some(Rc::new(RefCell::new(TreeNode::new(9))));
    root.left.clone().unwrap().borrow_mut().right.clone().unwrap().borrow_mut().left = Some(Rc::new(RefCell::new(TreeNode::new(7))));
    root.left.clone().unwrap().borrow_mut().right.clone().unwrap().borrow_mut().right = Some(Rc::new(RefCell::new(TreeNode::new(8))));



    root1.left = Some(Rc::new(RefCell::new(TreeNode::new(3))));
    root1.right = Some(Rc::new(RefCell::new(TreeNode::new(2))));
    root1.left.clone().unwrap().borrow_mut().left = Some(Rc::new(RefCell::new(TreeNode::new(9))));
    root1.left.clone().unwrap().borrow_mut().right = Some(Rc::new(RefCell::new(TreeNode::new(6))));
    root1.right.clone().unwrap().borrow_mut().left = Some(Rc::new(RefCell::new(TreeNode::new(4))));
    root1.right.clone().unwrap().borrow_mut().right = Some(Rc::new(RefCell::new(TreeNode::new(5))));
    root1.right.clone().unwrap().borrow_mut().right.clone().unwrap().borrow_mut().left = Some(Rc::new(RefCell::new(TreeNode::new(8))));
    root1.right.clone().unwrap().borrow_mut().right.clone().unwrap().borrow_mut().right = Some(Rc::new(RefCell::new(TreeNode::new(7))));

    let res = flip_equiv(Some(Rc::new(RefCell::new(root))),Some(Rc::new(RefCell::new(root1))));
    println!("{:}",res);

}