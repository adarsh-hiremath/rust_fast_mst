use crate::graph::Graph;
use rayon::prelude::*;

use crate::{graph::EuclideanGraph, kruskals};

#[allow(dead_code, unused)]
const TRIALS: u32 = 10;
#[allow(dead_code, unused)]
const TARGET_ERROR: f64 = 0.2;

#[allow(dead_code, unused)]
fn estimate_with_filter<G: Graph, F: Fn(usize) -> f64 + Copy + Send + Sync>(
    vertices: usize,
    filter: F,
    create: impl Fn(usize, F) -> G + Send + Sync,
) -> f64 {
    (0..TRIALS)
        .into_iter()
        .into_par_iter()
        .map(|_| kruskals::mst(&create(vertices, filter)))
        .sum::<f64>()
        / (TRIALS as f64)
}

#[allow(dead_code, unused)]
fn estimate_filter(vertices: usize) -> f64 {
    let ground_truth =
        estimate_with_filter(vertices, |_| 1.0, EuclideanGraph::<4>::create_with_filter);
    // println!("ground_truth {vertices:2} -> {ground_truth}");

    let mut level = 1_f64;
    let mut error = 0_f64;

    while error < TARGET_ERROR {
        level /= 1.2;
        let result =
            estimate_with_filter(vertices, |_| level, EuclideanGraph::<4>::create_with_filter);
        error = (ground_truth - result).abs() / ground_truth;
        // println!("error {error} from {level}");
    }

    level
}
