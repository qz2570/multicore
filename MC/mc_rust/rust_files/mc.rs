use rand::Rng;
use rayon::prelude::*;
use std::env;
use std::f64::consts::PI;
fn mc_sampling(sampling_num: u64) -> u64 {
    let mut accepted_samples = 0;
    let mut rng = rand::thread_rng();
    for _ in 0..sampling_num {
        let x: f64 = rng.gen();
        let y: f64 = rng.gen();
        if x * x + y * y <= 1.0 {
            accepted_samples += 1;
        }
    }
    accepted_samples
}
fn main() {
    // Get command-line arguments
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        eprintln!("Usage: {} [sampling_num] [thread_num]", args[0]);
        std::process::exit(1);
    }
    let sampling_num: u64 = args[1].parse().expect("Invalid sampling number");
    let thread_num: usize = args[2].parse().expect("Invalid thread number");
    // Divide the total samples by the number of threads
    let samples_per_thread = sampling_num / thread_num as u64;
    // Perform parallel Monte Carlo sampling
    let total_accepted: u64 = (0..thread_num)
        .into_par_iter()
        .map(|_| mc_sampling(samples_per_thread))
        .sum();
    // Calculate π and error
    let pi_estimate = 4.0 * (total_accepted as f64) / (sampling_num as f64);
    let error = (pi_estimate - PI).abs() / PI;
    println!("Estimated PI: {:.15}, Error: {:.15}", pi_estimate, error);
}