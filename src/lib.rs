use bitvec::prelude::*;
use counter::Counter;
use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::collections::HashSet;
use std::hash::Hash;
use std::iter::Iterator;
struct Node<T> {
    frequency: usize,
    node_type: NodeType<T>,
}
impl<T> PartialEq for Node<T> {
    fn eq(&self, other: &Self) -> bool {
        self.frequency == other.frequency
    }
}
impl<T> Eq for Node<T> {}
impl<T> PartialOrd for Node<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl<T> Ord for Node<T> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.frequency.cmp(&other.frequency).reverse()
    }
}

enum NodeType<T> {
    Branch {
        left: Box<Node<T>>,
        right: Box<Node<T>>,
    },
    Leaf(T),
}
#[derive(Debug)]
enum Direction {
    Left,
    Right,
}

pub struct Tree<T> {
    root: Box<Node<T>>,
    elements: HashSet<T>,
}

impl<T> Tree<T>
where
    T: Hash,
    T: Eq,
    T: Clone,
{
    pub fn new<I>(source: I) -> Option<Tree<T>>
    where
        I: Iterator<Item = T>,
    {
        let counts = source.collect::<Counter<_>>();

        // Cannot create Huffman tree from empty source
        if counts.is_empty() {
            return None;
        }

        let mut node_queue = BinaryHeap::new();
        let mut elements = HashSet::new();

        for item in &counts {
            node_queue.push(Box::new(Node {
                node_type: NodeType::Leaf(item.0.clone()),
                frequency: item.1.clone(),
            }));
            elements.insert(item.0.clone());
        }

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
            elements,
        })
    }

    fn find_path(head: &Box<Node<T>>, path: &mut Vec<Direction>, target: &T) -> bool {
        match &head.node_type {
            NodeType::Leaf(letter) => {
                return *letter == *target;
            }
            NodeType::Branch { left, right } => {
                path.push(Direction::Left);
                if !Tree::find_path(&left, path, target) {
                    path.pop();
                } else {
                    return true;
                }

                path.push(Direction::Right);
                if !Tree::find_path(&right, path, &target) {
                    path.pop();
                } else {
                    return true;
                }
            }
        }

        false
    }

    fn get_code(&self, item: T) -> BitVec {
        let mut path = Vec::new();
        let mut code = BitVec::new();

        match self.root.node_type {
            NodeType::Leaf(_) => {
                path.push(Direction::Left);
            }
            NodeType::Branch { .. } => {
                Tree::find_path(&self.root, &mut path, &item);
            }
        }

        for direction in &path {
            match direction {
                Direction::Left => code.push(false), // 0
                Direction::Right => code.push(true), // 1
            }
        }

        code
    }

    pub fn encode<I>(&self, symbols: I) -> Option<BitVec>
    where
        I: Iterator<Item = T>,
    {
        let mut encoding = BitVec::new();

        for symbol in symbols {
            if self.elements.contains(&symbol) {
                encoding.append(&mut self.get_code(symbol))
            } else {
                return None;
            }
        }

        Some(encoding)
    }

    pub fn decode(&self, bits: &BitVec) -> Vec<T> {
        let mut curr_node = &self.root;
        let mut decoded = Vec::new();

        for bit in bits {
            match &curr_node.node_type {
                NodeType::Branch { left, right } => match *bit {
                    false => {
                        curr_node = &left;
                    }
                    true => {
                        curr_node = &right;
                    }
                },
                NodeType::Leaf(symbol) => {
                    decoded.push(symbol.clone());
                    curr_node = &self.root;
                }
            }
        }

        decoded
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
        let maybe_tree = Tree::new(input.chars());
        match maybe_tree {
            None => {
                if input.is_empty() {
                    return true;
                } else {
                    return false;
                }
            }
            Some(tree) => {
                return input
                    == tree
                        .decode(&tree.encode(input.chars()).unwrap())
                        .iter()
                        .collect::<String>()
            }
        }
    }
}
