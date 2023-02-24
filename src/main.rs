use std::time::Instant; // used for timing
use clap::Parser; // used for parsing command line arguments
use rayon::prelude::*; // used for parallelized iterator

// importing the fast_zero_dim_mst function from fast_zero_dim.rs, the CompleteGraph & EuclideanGraph structs from graph.rs
use crate::{
    fast_zero_dim::fast_zero_dim_mst,
    graph::{CompleteGraph, EuclideanGraph},
};

pub mod fast_zero_dim; // declare other files as public modules (as dependencies) so that rust knows what to build
pub mod filter; // declare other files as public modules (as dependencies) so that rust knows what to build
pub mod graph; // declare other files as public modules (as dependencies) so that rust knows what to build
pub mod kruskals; // declare other files as public modules (as dependencies) so that rust knows what to build

// filter function for both 0 and d-dimensional case
fn filter<const D: usize>(n: usize) -> f64 {
    let exponent = -1.0 / (D.max(2) as f64);
    2.0 * (n as f64).powf(exponent)
}

#[allow(unused)] // unoptimized zero-dimensional case 
fn run_zero_dimensional_case(n: usize) -> f64 {
    let graph = CompleteGraph::create_with_filter(n, filter::<0>);
    kruskals::mst(&graph)
}

// unoptimized d-dimensional case
fn run_d_dimensional_case<const D: usize>(n: usize) -> f64 {
    let graph = EuclideanGraph::<D>::create_with_filter(n, filter::<D>);
    kruskals::mst(&graph)
}

#[derive(Parser)] // parse the Arguements struct below as command line arguments
pub struct Arguments {
    _debug: usize,
    num_points: usize,
    num_trials: usize,
    dimensions: usize,
}

// code entry point
fn main() {
    let args = Arguments::parse();

    if !matches!(args.dimensions, 0 | 2 | 3 | 4) {
        panic!("unsupported dimension!");
    }

    let start = Instant::now();
    let average = (0..args.num_trials)
        .into_par_iter() // this (in combination with rayon::prelude crate paralellizes accross the trials)
        .map(|_| match args.dimensions {
            0 => fast_zero_dim_mst(args.num_points),
            2 => run_d_dimensional_case::<2>(args.num_points),
            3 => run_d_dimensional_case::<3>(args.num_points),
            4 => run_d_dimensional_case::<4>(args.num_points),
            _ => unreachable!(),
        })
        .sum::<f64>() / args.num_trials as f64;

    if args._debug == 1 {
        println!("{:?}", start.elapsed());
    }

    // the average to 4 decimal precision points + numpoints numtrials dimension gets printed here
    println!(
        "{:.4} {} {} {}",
        average, args.num_points, args.num_trials, args.dimensions
    );
}
