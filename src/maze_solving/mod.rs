mod agent;
mod maze;


pub fn run_maze_solver() {
    let m = maze::Maze::new(10, 10, (0, 0), (8, 7), 0.3);
    println!("{}", m);
}