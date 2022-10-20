use super::maze::{Location, Maze};
use super::node::Node;
use super::node::Stack;

pub fn dfs(maze: &Maze) -> Option<Node> {
    let mut frontier: Stack<Node> = vec![];
    let start_node = Node::new(maze.start_location, None);
    frontier.push(start_node.clone());

    let mut explored: Stack<Location> = vec![];
    explored.push(maze.start_location);

    while !frontier.is_empty() {
        let current_node = frontier.pop().unwrap();
        let current_state = current_node.state;

        if maze.goal_location == current_state {
            return Some(current_node);
        }
        for s in maze.successor(&current_state) {
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
