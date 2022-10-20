mod search;
mod maze;
mod node;

pub fn run_maze_solver() {
    let m = maze::Maze::new(50, 50, (0, 0), (40, 37), 0.3);
    let found = search::dfs(&m);
    println!("{}", m);
    match found {
        None => println!("No solution found"),
        Some(s) => {
            let path = maze::Solution::new(m, node::Node::to_path(&s));
            println!("{}", path);
        }
    }
}
