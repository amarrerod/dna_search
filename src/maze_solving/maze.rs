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
/// ▉ for empty cells, X for blocked cells, S for the start cell and  G for the goal cell
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
    pub rows: usize,
    pub cols: usize,
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

}

impl fmt::Display for Maze {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for r in 0..self.rows {
            for c in 0..self.cols {
                _ = write!(f, "{}", self.grid[r][c]);
            }
            _ = write!(f, "\n");
        }
        write!(f, " ")
    }
}

