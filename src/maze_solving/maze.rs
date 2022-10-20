use rand::{distributions::Uniform, prelude::*};
use std::fmt;
use std::vec;

/// A location is represented as a tuple of (x, y) coordinates
pub type Location = (i32, i32);

/// The Cell type is an enum with different variantions of cells
#[derive(Debug, PartialEq, Clone)]
pub enum Cell {
    /// An empty cell which can be walked into
    EMPTY = 0,
    /// A cell which is blocked by a wall
    BLOCKED = -1,
    /// The start cell of the maze
    START = 1,
    /// The goal cell of the maze
    GOAL = 2,
}
/// Custom display of cells using 
/// ▉ for empty cells, X for blocked cells, S for the start cell and G for the goal cell
impl fmt::Display for Cell {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Cell::EMPTY => write!(f, "▉"),
            Cell::BLOCKED => write!(f, "X"),
            Cell::START => write!(f, "S"),
            Cell::GOAL => write!(f, "G"),
        }
    }
}

/// A Maze
#[derive(Debug)]
pub struct Maze {
    rows: usize,
    cols: usize,
    pub start_location: Location,
    pub goal_location: Location,
    sparseness: f64,
    pub grid: Vec<Vec<Cell>>,
}

impl Maze {
    pub fn new(
        rows: usize,
        cols: usize,
        start_location: Location,
        goal_location: Location,
        sparseness: f64,
    ) -> Maze {
        let mut m = Maze {
            rows,
            cols,
            start_location,
            goal_location,
            sparseness,
            grid: vec![vec![Cell::EMPTY; cols]; rows],
        };
        m.random_fill();
        m
    }

    fn random_fill(&mut self) {
        let mut rng = thread_rng();
        let u = Uniform::new(0.0, 1.0);
        for r in 0..self.rows {
            for c in 0..self.cols {
                if rng.sample(u) < self.sparseness {
                    self.grid[r][c] = Cell::BLOCKED;
                }
            }
        }

        let (x, y) = self.start_location;
        let (gx, gy) = self.goal_location;
        self.grid[x as usize][y as usize] = Cell::START;
        self.grid[gx as usize][gy as usize] = Cell::GOAL;
    }

    pub fn successor(&self, loc: &Location) -> Vec<Location> {
        let mut locations = vec![];
        let (x, y) = loc.clone();

        if x + 1 < self.rows as i32 && self.grid[(x + 1) as usize][y as usize] != Cell::BLOCKED {
            locations.push((x + 1, y));
        }

        if (x - 1) >= 0 && self.grid[(x - 1) as usize][y as usize] != Cell::BLOCKED {
            locations.push((x - 1, y));
        }

        if y + 1 < self.cols as i32 && self.grid[x as usize][(y + 1) as usize] != Cell::BLOCKED {
            locations.push((x, y + 1));
        }

        if (y - 1) >= 0 && self.grid[x as usize][(y - 1) as usize] != Cell::BLOCKED {
            locations.push((x, y - 1));
        }

        locations
    }
}

impl fmt::Display for Maze {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for r in 0..self.rows {
            for c in 0..self.cols {
                write!(f, "{}", self.grid[r][c]);
            }
            write!(f, "\n");
        }
        write!(f, " ")
    }
}

pub struct Solution {
    maze: Maze,
    path: Vec<Location>,
}

impl Solution {
    pub fn new(maze: Maze, path: Vec<Location>) -> Solution {
        Solution { maze, path }
    }
}

impl fmt::Display for Solution {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for r in 0..self.maze.rows {
            for c in 0..self.maze.cols {
                if self.path.contains(&(r as i32, c as i32)) {
                    write!(f, "\x1b[93m*\x1b[0m");
                } else {
                    write!(f, "{}", self.maze.grid[r][c]);
                }
            }
            write!(f, "\n");
        }
        write!(f, " ")
    }
}
