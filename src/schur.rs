use std::collections::HashMap;
use std::process;

use clap::Parser;

mod algorithm;

use lib::COLOR_LETTERS;
use lib::ColorVec;

#[derive(Parser, Debug)]
pub struct Args {
    /// Number of colors to use
    #[arg(long, required = true)]
    colors: u8,

    /// Target number you want a coloring for
    #[arg(long, required = true)]
    target: usize,

    /// Algorithm
    #[arg(long, required = true)]
    algorithm: String,

    /// Attempts
    #[arg(long, default_value = "100")]
    attempts: usize,
}

type SchurColoringAlgorithm = fn(args: &Args) -> Option<Vec<ColorVec>>;

fn main() {
    let args = Args::parse();

    if args.colors as usize > COLOR_LETTERS.len() {
        panic!("Max colors is {}", COLOR_LETTERS.len());
    }

    let mut algorithms = HashMap::new();
    algorithms.insert(
        "random",
        algorithm::random::random_assignment as SchurColoringAlgorithm,
    );
    algorithms.insert("random_ban", algorithm::random::random_with_bannings);
    algorithms.insert("random_dfs", algorithm::random::random_with_backtrack);
    algorithms.insert("search_dfs", algorithm::search::depth_first);
    algorithms.insert("search_bfs", algorithm::search::breadth_first);
    algorithms.insert("sim", algorithm::simulation::annealing);

    let algorithm = match algorithms.get(&args.algorithm.as_str()) {
        Some(method) => method,
        None => {
            println!("Unsupported algorithm: {}", args.algorithm);
            println!(
                "Alogithms: {}",
                algorithms.keys().copied().collect::<Vec<_>>().join(", ")
            );
            process::exit(1);
        }
    };

    let result = algorithm(&args);
    match result {
        Some(solutions) => {
            for s in solutions {
                println!("Result: {:?}", s);
                println!("Short form: {:?}", lib::short(&s));
                match lib::check_coloring(&s) {
                    Ok(check_ok) => {
                        println!("{check_ok}");
                    }
                    Err(invalid) => {
                        println!(
                            "Coloring failed at index {}: {}",
                            invalid.index, invalid.message
                        );
                    }
                }
            }
        }
        None => {
            println!("No coloring found.");
        }
    }

    process::exit(0);
}
