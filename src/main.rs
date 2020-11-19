use rand::Rng;
use std::thread;
const RADIUS: f64 = 50.0;

struct Experiment {
    experiments: u64,
    successes: u64,
}

fn main() {
    let mut children = vec![];
    for _ in 0..8 {
        children.push(thread::spawn(|| {
            let mut rng = rand::thread_rng();
            let mut experiments = 0;
            let mut successes = 0;
            for _ in 0..10000000 {
                experiments = experiments + 1;
                let x: f64 = rng.gen::<f64>() * RADIUS;
                let y: f64 = rng.gen::<f64>() * RADIUS;
                let radius = (x.powi(2) + y.powi(2)).sqrt();
                if radius <= RADIUS {
                    successes = successes + 1;
                }
            }
            Experiment {
                successes,
                experiments,
            }   
        }));
    }

    let mut all_experiments = Experiment {
        successes: 0,
        experiments: 0,
    }; 
    for child in children {
        let new_experiment = child.join().unwrap();
        all_experiments.experiments += new_experiment.experiments;
        all_experiments.successes += new_experiment.successes;
    }

    let pi : f64 = 
        (all_experiments.successes as f64) / 
        (all_experiments.experiments as f64) * 
        4.0;
    println!("Pi = {}", pi);
}
