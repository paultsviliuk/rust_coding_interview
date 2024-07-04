/*
Let's use a depth-first search (DFS) to solve this problem.
The intuition here is to traverse the tree level by level recursively, starting from the rightmost node for each recursive call.

The time complexity will be O(N)O(N), where N is the number of nodes because we have to visit all the nodes.
*/

use std::rc::Rc;
use std::cell::RefCell;
use std::collections::VecDeque;

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

fn right_side_view(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
    let mut response = Vec::new();
    let mut cur_level = 0;

    if root.is_none() {
        return response;
    }

    let mut dq = VecDeque::new();
    dq.push_back((0, root.clone()));
    response.push(root.as_ref().unwrap().borrow().val);

    while !dq.is_empty() {
        if let Some((level, Some(node))) = dq.pop_front() {
            dq.push_back((level + 1, node.borrow().right.clone()));
            dq.push_back((level + 1, node.borrow().left.clone()));
            if level > cur_level {
                response.push(node.borrow().val);
                cur_level = level;
            }
        }
    }
    response
}

fn main() {

    let mut root = TreeNode::new(1);
    root.left = Some(Rc::new(RefCell::new(TreeNode::new(2))));
    root.right = Some(Rc::new(RefCell::new(TreeNode::new(3))));
    root.left.clone().unwrap().borrow_mut().left = Some(Rc::new(RefCell::new(TreeNode::new(4))));
    root.left.clone().unwrap().borrow_mut().right = Some(Rc::new(RefCell::new(TreeNode::new(5))));
    root.right.clone().unwrap().borrow_mut().left = Some(Rc::new(RefCell::new(TreeNode::new(6))));
    root.right.clone().unwrap().borrow_mut().right = Some(Rc::new(RefCell::new(TreeNode::new(7))));
    root.right.clone().unwrap().borrow_mut().right.clone().unwrap().borrow_mut().left = Some(Rc::new(RefCell::new(TreeNode::new(8))));

    let resp = right_side_view(Some(Rc::new(RefCell::new(root))));
     println!("{:?}",resp);

}