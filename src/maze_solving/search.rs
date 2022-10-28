use std::collections::btree_map::{VacantEntry, OccupiedEntry};
use std::collections::{VecDeque, HashSet, BinaryHeap, HashMap};

use super::maze::{Location, Maze};
use super::node::Node;
use super::solution::Solution;
use super::distances;
pub type Stack<T> = Vec<T>;

pub fn dfs(maze: &Maze) -> Option<Solution> {
    let mut frontier: Stack<Node> = vec![];
    let start_node = Node::new(maze.start_location, None);
    frontier.push(start_node.clone());

    let mut explored: Stack<Location> = vec![];
    explored.push(maze.start_location);

    while !frontier.is_empty() {
        let current_node = frontier.pop().unwrap();
        let current_state = current_node.state;

        if maze.goal_location == current_state {
            return Some(Solution::new(maze, Node::to_path(&current_node)));
        }
        for s in current_node.successors(&maze) {
            match explored.contains(&s) {
                true => continue,
                false => {
                    explored.push(s);
                    frontier.push(Node::new(s, Some(Box::new(current_node.clone()))));
                }
            }
        }
    }
    None
}


pub fn bfs(maze: &Maze) -> Option<Solution> {
    let mut frontier : VecDeque<Node> = VecDeque::new();
    let start_node = Node::new(maze.start_location, None);
    frontier.push_back(start_node.clone());
    
    let mut explored : HashSet<Location> = HashSet::new();
    explored.insert(maze.start_location);

    while !frontier.is_empty() {
        let current_node = frontier.pop_front().unwrap();
        let current_state = current_node.state;
        if maze.goal_location == current_state {
            return Some(Solution::new(maze, Node::to_path(&current_node)));
        }
        for s in current_node.successors(&maze) {
            match explored.contains(&s) {
                true => continue, 
                false => {
                    explored.insert(s);
                    frontier.push_back(Node::new(s, Some(Box::new(current_node.clone()))));
                }
            }
        }
    }
    None
}


pub fn a_star(maze: &Maze) -> Option<Solution> {
    let mut frontier : BinaryHeap<Node> = BinaryHeap::new();
    let mut start_node = Node::new(maze.start_location, None);
    start_node.cost = 0.0;
    start_node.heuristic = distances::euclidean_distance(&start_node.state, &maze.goal_location);
    frontier.push(start_node.clone());

    let mut explored : HashMap<Location, f64> = HashMap::new();
    explored.insert(maze.start_location, 0.0);

    while !frontier.is_empty() {
        let current_node = frontier.pop().unwrap();
        let current_state = current_node.state;
        if maze.goal_location == current_state {
            return Some(Solution::new(maze, Node::to_path(&current_node)));
        }
        for s in current_node.successors(&maze) {
            let new_cost = current_node.cost + 1.0;
            // let entry = explored.entry(s).and_modify(|c| {
            //     if *c > new_cost {
            //         *c = new_cost;
            //     }
            // }).or_insert(new_cost);
            let is_in = explored.get_key_value(&s);
            let mut to_include : bool = false;
            match is_in {
                None => {
                    explored.insert(s, new_cost);
                    to_include = true;
                }
                Some((_, v)) => {
                    if *v > new_cost {
                        explored.entry(s).and_modify(|c| *c = new_cost);
                        to_include = true;
                    }
                }
            }
            if to_include {
                let mut node = Node::new(s, Some(Box::new(current_node.clone())));
                node.cost = new_cost;
                node.heuristic = distances::euclidean_distance(&node.state, &maze.goal_location);
                frontier.push(node);
            }
        }
    }
    None
}