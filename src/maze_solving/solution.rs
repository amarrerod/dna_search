use super::maze::{Maze, Location};
use std::fmt;

pub struct Solution<'a> {
    maze: &'a Maze,
    path: Vec<Location>,
}

impl<'a> Solution<'a> {
    pub fn new(maze: &Maze, path: Vec<Location>) -> Solution {
        Solution { maze, path }
    }
}

impl<'a> fmt::Display for Solution<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for r in 0..self.maze.rows {
            for c in 0..self.maze.cols {
                if self.path.contains(&(r as i32, c as i32)) {
                   _ =  write!(f, "\x1b[93m*\x1b[0m");
                } else {
                  _ =  write!(f, "{}", self.maze.grid[r][c]);
                }
            }
           _ = write!(f, "\n");
        }
        write!(f, " ")
    }
}
