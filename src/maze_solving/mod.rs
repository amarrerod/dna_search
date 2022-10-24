mod search;
mod maze;
mod node;

pub fn run_maze_solver() {
    let m = maze::Maze::new(50, 50, (0, 0), (40, 37), 0.3);
    let solution = search::dfs(&m);
    match solution {
        None => println!("No solution found"),
        Some(path) => {
            println!("{}", path);
        }
    }
}
