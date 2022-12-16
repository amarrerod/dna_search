mod dna_search;
mod maze_solving;
use rand::Rng;

mod sorting;
fn main() {
    dna_search::run_dna_search();
    maze_solving::run_maze_solver();

    let mut rng = rand::thread_rng();
    let mut numbers: Vec<u64> = (0..10).map(|_| rng.gen_range(0..100)).collect();
    println!("Numbers before sorting: {:?}", numbers);
    sorting::quicksort(&mut numbers, 0, 9);
    println!("Numbers after sorting: {:?}", numbers);
}
