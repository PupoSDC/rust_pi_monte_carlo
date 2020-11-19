use rand::Rng;

const RADIUS: f64 = 50.0;

fn main() {
    let mut rng = rand::thread_rng();
    let mut experiments = 0;
    let mut successes = 0;
    
    for _ in 0..100000 {
        experiments = experiments + 1;
        let x: f64 = rng.gen::<f64>() * RADIUS;
        let y: f64 = rng.gen::<f64>() * RADIUS;
        let radius = (x.powi(2) + y.powi(2)).sqrt();
        if radius <= RADIUS {
            successes = successes + 1;
        }
    }
    let pi : f64 = (successes as f64) / (experiments as f64) * 4.0;
    println!("Hello, world! {}", pi);
}
