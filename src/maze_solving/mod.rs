mod search;
mod maze;
mod node;
mod solution;
mod distances;

pub fn run_maze_solver() {
    let m = maze::Maze::new(50, 50, (0, 0), (40, 37), 0.3);
    let mut solution = search::dfs(&m);
    match solution {
        None => println!("No solution found with DFS"),
        Some(path) => {
            println!("{}", path);
        }
    }

    solution = search::bfs(&m);
    match solution {
        None => println!("No solution found with BFS"),
        Some(path) => {
            println!("{}", path);
        }
    }

    solution = search::a_star(&m);
    match solution {
        None => println!("No solution found with A*"),
        Some(path) => {
            println!("{}", path);
        }
    }
}
