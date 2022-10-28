
use super::maze::Location;

pub fn euclidean_distance(loc1: &Location, loc2: &Location) -> f64 {
    let (x_1, y_1) = loc1;
    let (x_2, y_2) = loc2;
    let x_dist = *x_1 as f64 - *x_2 as f64;
    let y_dist = *y_1 as f64 - *y_2 as f64;
    ((x_dist * x_dist) + (y_dist * y_dist)).sqrt()
}