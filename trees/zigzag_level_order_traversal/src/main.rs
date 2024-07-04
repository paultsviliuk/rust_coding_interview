/*
use a modified breadth-first search (BFS) approach. 
The zigzag order means that we traverse the levels of the tree alternatively from left to right and then from right to left. 
To achieve this, we maintain a queue (or deque) to store nodes along with their levels 
and use a flag to indicate the direction of traversal for each level.


Solution Explanation

To solve the problem of zigzag level order traversal in a binary tree, we use a modified breadth-first search (BFS) approach. The zigzag order means that we traverse the levels of the tree alternatively from left to right and then from right to left. To achieve this, we maintain a queue (or deque) to store nodes along with their levels and use a flag to indicate the direction of traversal for each level.

Here’s the step-by-step approach:

    Initialization:
        Create a result vector res to store the final zigzag level order traversal.
        Create a queue (deque) dq to facilitate BFS, where each element is a tuple containing the level of the node and the node itself.
        Initialize curlevel to keep track of the current level.

    BFS Traversal:
        Start by pushing the root node (if it exists) onto the queue with its level set to 0.
        Traverse the tree level-by-level using a loop until the queue is empty.
        For each node, check its level. If it's greater than curlevel, it indicates that we've moved to the next level, and the current level's nodes should be added to response.
        Depending on the level (even or odd), we reverse the collected nodes' values for that level before adding them to the result.

    Handling the Zigzag:
        Use the curlevel % 2 check to determine if the current level should be reversed.
        For each node, add its children (if any) to the queue with their respective levels incremented by 1.

    Final Step:
        After exiting the loop, if there are any remaining nodes collected in the current level vector, process and add them to the result.
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

fn zigzag_level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
    let mut response = Vec::new();
    let mut cur_level = 0;
    if root.is_none() {
        return response;
    }
    let mut dq = VecDeque::new();
    dq.push_back((0, root.clone()));
    let mut vec = Vec::new();
    while !dq.is_empty() {
        if let Some((level, Some(node))) = dq.pop_front() {
            dq.push_back((level + 1, node.borrow().left.clone()));
            dq.push_back((level + 1, node.borrow().right.clone()));
            if level > cur_level {
                if cur_level % 2 == 1 {
                    vec.reverse();
                }
                response.push(vec.clone());
                vec.clear();
                cur_level = level;
            }
            vec.push(node.borrow().val);
        }
    }

    if !vec.is_empty() {
        if cur_level % 2 == 1 {
            vec.reverse();
        }
        response.push(vec)
    }
    response
}



fn main() {
    let mut root = TreeNode::new(3);
    root.left = Some(Rc::new(RefCell::new(TreeNode::new(6))));
    root.right = Some(Rc::new(RefCell::new(TreeNode::new(9))));
    root.right.clone().unwrap().borrow_mut().left = Some(Rc::new(RefCell::new(TreeNode::new(15))));
    root.right.clone().unwrap().borrow_mut().right = Some(Rc::new(RefCell::new(TreeNode::new(12))));

    let res = zigzag_level_order(Some(Rc::new(RefCell::new(root))));
    println!("{:?}", res);
}
