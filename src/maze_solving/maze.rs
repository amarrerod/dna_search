use std::vec;
use rand::{prelude::*, distributions::Uniform};
use std::fmt;

#[derive(Debug, PartialEq, Clone)]
pub enum Cell {
    EMPTY = 0,
    BLOCKED = -1,
    START = 1,
    GOAL = 2,
    PATH = 10,
}

impl fmt::Display for Cell {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Cell::EMPTY =>  write!(f, "▉"),
            Cell::BLOCKED =>  write!(f, "X"),
            Cell::START =>  write!(f, "S"),
            Cell::GOAL =>  write!(f, "G"),
            Cell::PATH =>  write!(f, "*"),
        }
    }
}

#[derive(Debug)]
pub struct Maze {
    rows: usize,
    cols: usize,
    start_location: (usize, usize),
    goal_location: (usize, usize),
    sparseness: f64,
    pub grid: Vec<Vec<Cell>>,
}

impl Maze {
    pub fn new(
        rows: usize,
        cols: usize,
        start_location: (usize, usize),
        goal_location: (usize, usize),
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
        self.grid[x][y] = Cell::START;
        self.grid[gx][gy] = Cell::GOAL;
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