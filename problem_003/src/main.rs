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

mod node;
use node::Node;

fn serialize(nodes: &Node) -> String {
    let mut serialized_tree: String = String::from("");
    for node in nodes.postorder_iter() {
        serialized_tree.push_str(&*node.val);
        serialized_tree.push_str("|");
    }
    return serialized_tree;
}

fn deserialize(serialized_tree: &String) -> Node {
    let mut nodes: Vec<Node> = vec![];

    let serialized_nodes = serialized_tree.split("|");
    let mut serialized_noded_vec: Vec<&str> = serialized_nodes.collect();
    serialized_noded_vec.pop(); //Remove redundant "|"

    fn find_child(child_val: String, nodes: &mut Vec<Node>) -> Option<Box<Node>> {
        let child_index = nodes.iter().position(|x| x.val == child_val);
        match child_index {
            Some(index) => {
                let node = nodes.remove(index);
                Some(Box::from(node))
            }
            None => None,
        }
    }

    for serialized_node in serialized_noded_vec {
        let left_child_val = if serialized_node == "root" {
            String::from("left")
        } else {
            String::from(serialized_node.clone()) + ".left"
        };
        let right_child_val = if serialized_node == "root" {
            String::from("right")
        } else {
            String::from(serialized_node.clone()) + ".right"
        };
        let left_child = find_child(left_child_val, &mut nodes);
        let right_child = find_child(right_child_val, &mut nodes);

        nodes.push(Node::new(
            String::from(serialized_node),
            left_child,
            right_child,
        ));
    }
    let tree = nodes.pop().unwrap();
    return tree;
}

fn main() {
    let a = Node::new(String::from("left.left"), None, None);
    let b = Node::new(String::from("left.right"), None, None);
    let c = Node::new(String::from("left"), Some(Box::from(a)), Some(Box::from(b)));
    let d = Node::new(String::from("right"), None, None);

    let i = Node::new(String::from("root"), Some(Box::from(c)), Some(Box::from(d)));

    let rhs = deserialize(&serialize(&i)).left.unwrap().left.unwrap().val;
    let lhs = i.left.unwrap().left.unwrap().val;
    
    assert_eq!(lhs, rhs)
}
