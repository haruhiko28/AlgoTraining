use proconio::input;

fn main() {
    input! {
        x1: f64,
        y1: f64,
        x2: f64,
        y2: f64,
    }
    println!("{}", calc_dist(x1, y1, x2, y2));
}

fn calc_dist(x1: f64, y1: f64, x2: f64, y2: f64) -> f64 {
    return ((x2 - x1).powf(2.0) + (y2 - y1).powf(2.0)).sqrt();
}
