use super::Node;

pub struct PostorderIter<'a> {
    stack: Vec<&'a Node>,
    visited: Vec<&'a Node>,
}

impl<'a> PostorderIter<'a> {
    pub fn new(node: &'a Node) -> Self {
        PostorderIter {
            stack: vec![&node],
            visited: vec![],
        }
    }
}

impl<'a> Iterator for PostorderIter<'a> {
    type Item = &'a Node;

    fn next(&mut self) -> Option<Self::Item> {
        // Loop to drill down to leaf
        while self.stack.len() > 0 {
            if let Some(node) = self.stack.pop() {
                //If visiting a node for second time, return current node
                if self.visited.contains(&node) {
                    return Some(node);
                }

                // If at a leaf, return current node
                if node.left == None && node.right == None {
                    return Some(node);
                }

                // If none of the two above, continue down the tree, start by adding back current node
                self.visited.push(&node);
                self.stack.push(&node);

                if let Some(right) = &node.right {
                    self.stack.push(&right)
                }

                if let Some(left) = &node.left {
                    self.stack.push(&left)
                }
            }
        }

        return None;
    }
}
