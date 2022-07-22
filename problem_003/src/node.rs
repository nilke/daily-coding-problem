mod postorder_iterator;
mod preorder_iterator;

use self::preorder_iterator::PreorderIter;
use self::postorder_iterator::PostorderIter;

pub struct Node {
    pub val: String,
    pub left: Option<Box<Node>>,
    pub right: Option<Box<Node>>,
}


impl Node {
    pub fn new(val: String, left: Option<Box<Node>>, right: Option<Box<Node>>) -> Self {
        Node { val, left, right }
    }
    pub fn preorder_iter(&self) -> PreorderIter {
        PreorderIter::new(self)
    }
    pub fn postorder_iter(&self) -> PostorderIter {
        PostorderIter::new(self)
    }
}
impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.val == other.val && self.left == other.left && self.right == other.right
    }
}