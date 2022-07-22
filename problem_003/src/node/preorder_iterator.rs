use super::Node;

pub struct PreorderIter<'a> {
    stack: Vec<&'a Node>,
}

impl<'b> PreorderIter<'b> {
    pub fn new(node: &'b Node) -> Self {
        PreorderIter { stack: vec![&node] }
    }
}

impl<'c> Iterator for PreorderIter<'c> {
    type Item = &'c Node;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(node) = self.stack.pop() {
            if let Some(right) = &node.right {
                self.stack.push(&right)
            }
            if let Some(left) = &node.left {
                self.stack.push(&left)
            }
            return Some(node);
        }
        return None;
    }
}
