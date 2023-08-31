#![allow(unused)]
use std::rc::Rc;
use std::cell::RefCell;

/// ## 116. Populating Next Right Pointers in Each Node
/// https://leetcode.com/problems/populating-next-right-pointers-in-each-node/
///
/// You are given a perfect binary tree where all leaves are on the same level, and every parent has 
/// two children. The binary tree has the following definition:
///
/// ```rust
/// type NodeRef = Rc<RefCell<Node>>;
/// 
/// struct Node {
///    val: i32,
///    left: Option<NodeRef>,
///    right: Option<NodeRef>,
///    next: Option<NodeRef>
/// }
/// ```
/// Populate each next pointer to point to its next right node. If there is no next right node, the 
/// next pointer should be set to NULL.
///
/// Initially, all next pointers are set to NULL.
///
/// Example 1:
/// -----------
/// ```doc
/// Input: root = [1,2,3,4,5,6,7]
/// Output: [1,#,2,3,#,4,5,6,7,#]
///
/// *Explanation*: Given the above perfect binary tree (Figure A), your function should populate each 
/// next pointer to point to its next right node, just like in Figure B. The serialized output is in 
/// level order as connected by the next pointers, with '#' signifying the end of each level.
///
/// ```
/// Example 2:
/// -----------
/// ```doc
/// Input: root = []
/// Output: []
/// ```
///
/// Constraints:
/// -------------
/// * The number of nodes in the tree is in the range [0, 212 - 1].
/// * -1000 <= Node.val <= 1000
///
/// Follow-up:
/// ------------
/// * You may only use constant extra space.
/// * The recursive approach is fine. You may assume implicit stack space does not count as extra space for this problem.
///
struct Solution;

#[derive(Debug, Clone)]
struct Node {
    val: i32,
    left: Option<NodeRef>,
    right: Option<NodeRef>,
    next: Option<NodeRef>
 }

type NodeRef = Rc<RefCell<Node>>;

impl Solution {
    fn connect(root: Option<NodeRef>) -> Option<NodeRef> {
        unimplemented!()
    }
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test1() {
        
    }

    #[test]
    fn test2() {

    }
}