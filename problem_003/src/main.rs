// This problem was asked by Google.

// Given the root to a binary tree, implement serialize(root), which serializes the tree into a string, and deserialize(s), which deserializes the string back into the tree.

// For example, given the following Node class

// class Node:
//     def __init__(self, val, left=None, right=None):
//         self.val = val
//         self.left = left
//         self.right = right
// The following test should pass:

// node = Node('root', Node('left', Node('left.left')), Node('right'))
// assert deserialize(serialize(node)).left.left.val == 'left.left'



struct Node {
    val: String,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}

impl Node {
    pub fn new(val: String, left: Option<Box<Node>>, right: Option<Box<Node>>) -> Self {
        Node {
            val,
            left,
            right
        }
    }
}

struct Tree {
   root: Option<Node>
}

impl Tree {
    pub fn new(root: Option<Node>) -> Self {
        Tree {root}
    }

    pub fn iter(&self) -> PreorderIter {
        PreorderIter::new(self.root.as_ref())
    }
}
struct PreorderIter<'a> {
    stack: Vec<&'a Node>
}

impl <'a> PreorderIter<'a> {
    pub fn new(root: Option<&'a Node>) -> Self {
        if let Some(node) = root {
            PreorderIter {
                stack: vec![&node]
            }
        } else {
            PreorderIter {
                stack: vec![]
            }
        }
    }
}


impl <'a> Iterator for PreorderIter<'a> {

    type Item = &'a Node;

    fn next(& mut self) -> Option<Self::Item> {
        if let Some(node) = self.stack.pop() {
            if let Some(right) = &node.right {
                self.stack.push(&right)
            }

            if let Some(left) = &node.left {
                self.stack.push(&left)
            }

            return Some(node)
        }
        return None
    }
    
}


fn main() {
        let a = Node::new(String::from("left.left"), None, None);
        let b = Node::new(String::from("left.right"), None, None);
        let c = Node::new(String::from("left"), Some(Box::from(a)), Some(Box::from(b)));

        let d = Node::new(String::from("right"), None, None);
        let e = Node::new(String::from("root"), Some(Box::from(c)), Some(Box::from(d)));

       let tree = Tree::new(Some(e));
   
       for node in tree.iter() {
            println!("{}", node.val)
       }
}
