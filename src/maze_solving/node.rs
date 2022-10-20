use super::maze::Location;
use std::cmp::Ordering;
use std::fmt;

pub type Stack<T> = Vec<T>;

#[derive(Debug, PartialEq, Clone)]
pub struct Node {
    pub state: Location,
    pub parent: Option<Box<Node>>,
    pub cost: f64,
    pub heuristic: f64,
}

impl Node {
    pub fn new(state: Location, parent: Option<Box<Node>>) -> Node {
        Node {
            state,
            parent: parent,
            cost: 0.0,
            heuristic: 0.0,
        }
    }

    pub fn to_path(node: &Node) -> Vec<Location> {
        let mut n: Node = node.clone();
        let mut path: Vec<Location> = vec![];
        while n.parent != None {
            n = *n.parent.unwrap();
            path.push(n.state);
        }
        path.reverse();
        path
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.heuristic.partial_cmp(&other.heuristic)
    }

    fn lt(&self, other: &Self) -> bool {
        (self.cost + self.heuristic) < (other.cost + other.heuristic)
    }
}
