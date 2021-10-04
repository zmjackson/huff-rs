use counter::Counter;
use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::collections::HashSet;
struct Node {
    frequency: usize,
    node_type: NodeType,
}
impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.frequency == other.frequency
    }
}
impl Eq for Node {}
impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        self.frequency.cmp(&other.frequency).reverse()
    }
}

enum NodeType {
    Branch { left: Box<Node>, right: Box<Node> },
    Leaf(char),
}
#[derive(Debug)]
enum Direction {
    Left,
    Right,
}

pub struct Tree {
    root: Box<Node>,
    letters: HashSet<char>,
}

impl Tree {
    pub fn new(source: &str) -> Option<Tree> {
        // Cannot create Huffman tree from empty source
        if source.is_empty() {
            return None;
        }

        let counts = source.chars().collect::<Counter<_>>();

        let mut node_queue = BinaryHeap::new();
        let mut letters = HashSet::new();

        for item in &counts {
            node_queue.push(Box::new(Node {
                node_type: NodeType::Leaf(item.0.clone()),
                frequency: item.1.clone(),
            }));
            letters.insert(item.0.clone());
        }

        // while let (Some(left), Some(right)) = (node_queue.pop(), node_queue.pop()) {}
        while node_queue.len() > 1 {
            let left = node_queue.pop().unwrap();
            let right = node_queue.pop().unwrap();
            node_queue.push(Box::new(Node {
                frequency: left.frequency + right.frequency,
                node_type: NodeType::Branch { left, right },
            }))
        }

        Some(Tree {
            root: node_queue.pop().unwrap(),
            letters,
        })
    }

    fn find_path(head: &Box<Node>, path: &mut Vec<Direction>, target: char) -> bool {
        match &head.node_type {
            NodeType::Leaf(letter) => {
                return *letter == target;
            }
            NodeType::Branch { left, right } => {
                path.push(Direction::Left);
                if !Tree::find_path(&left, path, target) {
                    path.pop();
                } else {
                    return true;
                }

                path.push(Direction::Right);
                if !Tree::find_path(&right, path, target) {
                    path.pop();
                } else {
                    return true;
                }
            }
        }

        false
    }

    fn get_code(&self, letter: char) -> String {
        let mut code = String::new();
        let mut path = Vec::new();

        match self.root.node_type {
            NodeType::Leaf(_) => {
                path.push(Direction::Left);
            }
            NodeType::Branch { .. } => {
                Tree::find_path(&self.root, &mut path, letter);
            }
        }

        for direction in &path {
            match direction {
                Direction::Left => code.push('0'),
                Direction::Right => code.push('1'),
            }
        }

        code
    }

    pub fn encode(&self, text: &str) -> Option<String> {
        let mut encoding = String::new();

        for letter in text.chars() {
            if self.letters.contains(&letter) {
                encoding.push_str(&self.get_code(letter))
            } else {
                return None;
            }
        }

        Some(encoding)
    }

    pub fn decode(&self, text: &str) -> Option<String> {
        let mut curr_node = &self.root;
        let mut decoded_text = String::new();

        for direction in text.chars() {
            if let NodeType::Branch { left, right } = &curr_node.node_type {
                match direction {
                    '0' => {
                        curr_node = &left;
                    }
                    '1' => {
                        curr_node = &right;
                    }
                    _ => return None,
                }
            } else if let NodeType::Leaf(symbol) = curr_node.node_type {
                decoded_text.push(symbol);
                curr_node = &self.root;
            }
        }

        Some(decoded_text)
    }
}

#[cfg(test)]
extern crate quickcheck;
#[cfg(test)]
#[macro_use(quickcheck)]
extern crate quickcheck_macros;

#[cfg(test)]
mod tests {
    use crate::Tree;

    #[quickcheck]
    fn encode_decode(input: String) -> bool {
        let maybe_tree = Tree::new(&input);
        match maybe_tree {
            None => {
                if input.is_empty() {
                    return true;
                } else {
                    return false;
                }
            }
            Some(tree) => return input == tree.decode(&tree.encode(&input).unwrap()).unwrap(),
        }
    }
}
