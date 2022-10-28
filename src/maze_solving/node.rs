use super::maze::{Location, Maze, Cell};
use std::cmp::Ordering;
 

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
    /// Uses the parent value to calculate the path of locations to arrive to this node.
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

    /// Calculates the list of neighbours to self
    pub fn successors(&self, maze: &Maze) -> Vec<Location> {
        let mut locations = vec![];
        let (x, y) = self.state.clone();

        if x + 1 < maze.rows as i32 && maze.grid[(x + 1) as usize][y as usize] != Cell::BLOCKED {
            locations.push((x + 1, y));
        }

        if (x - 1) >= 0 && maze.grid[(x - 1) as usize][y as usize] != Cell::BLOCKED {
            locations.push((x - 1, y));
        }

        if y + 1 < maze.cols as i32 && maze.grid[x as usize][(y + 1) as usize] != Cell::BLOCKED {
            locations.push((x, y + 1));
        }

        if (y - 1) >= 0 && maze.grid[x as usize][(y - 1) as usize] != Cell::BLOCKED {
            locations.push((x, y - 1));
        }

        locations
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

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.cost > other.cost {
            Ordering::Less
        } else if other.cost > self.cost {
            Ordering::Greater
        } else {
            Ordering::Equal
        }
    }
}


impl Eq for Node {}
