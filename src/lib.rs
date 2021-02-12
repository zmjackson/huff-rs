use counter::Counter;
use std::collections::BinaryHeap;
use std::collections::HashSet;

#[derive(PartialEq, Eq, PartialOrd, Ord)]
enum Node {
    Branch {
        frequency: i32,
        left: Option<Box<Node>>,
        right: Option<Box<Node>>,
    },
    Leaf(char),
}

struct Tree {
    root: Option<Box<Node>>,
    letters: HashSet<char>,
}

impl Tree {
    pub fn new(source: &str) -> Tree {
        let counts = source.chars().collect::<Counter<_>>();

        let mut node_queue: BinaryHeap<Box<Node>> = BinaryHeap::new();
        let mut letters: HashSet<char> = HashSet::new();

        for item in &counts {
            node_queue.push(Box::new(Node::Leaf(item.0.clone())));
            letters.insert(*item.0);
        }

        Tree {
            root: node_queue.peek(),
            letters: letters,
        }
    }

    pub fn encode(&self, text: &str) -> String {
        String::from("")
    }

    pub fn decode(&self, text: &str) -> String {
        String::from("")
    }
}
