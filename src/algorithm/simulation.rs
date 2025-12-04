use std::f64::consts;

use rand::Rng;

use lib::*;

use crate::Args;
use crate::algorithm::random;

// Freeze the column from the bottom and keeping the correct ones.
pub fn annealing(args: &Args) -> Option<Vec<ColorVec>> {
    let mut result = Vec::new();

    let mut rng = rand::rng();
    let mut candidate: ColorVec = random::random_fill(args.colors, args.target);

    // Note: we never need to change the first 2 colors and they must be different.
    candidate[0] = 0;
    candidate[1] = 1;

    let mut sentinel = 0;
    while sentinel <= args.attempts {
        // println!("---------- {candidate:?} -------------");
        match lib::check_coloring(&candidate) {
            Ok(_) => {
                // println!("Solution is valid!");
                result.push(candidate.clone());
                // Maybe find another
                candidate = random::random_fill(args.colors, args.target);
            }
            Err(e) => {
                // println!("Failed at index {}", e.index);
                // anneal! e.index is the first incorrect one
                // of course we can't really know anything about the rest, because we don't know
                // what the color for e.index "should" be. We do know that some higher ones are wrong for sure.
                // Maybe it's good to check everything rather than up to the first one?
                // We use an sigmoid function where e.index has a 50% chance of being changed.
                // logistics function f(x) = L / ( 1 + e^-k(x-x_0) ) where
                // L is the max. (we use 1)
                // k is the steepness which we guess at? Should be fairly steep to avoid reassigning correct numbers.
                // x_0 is the midpoint which is e.index
                candidate[e.index] = rand::random_range(0..args.colors); // Always change the one that failed
                for i in 2..candidate.len() {
                    let odds: f64 = logistic(i, e.index);
                    // print!("f({}, {}) = {:.4}", i, e.index, odds);
                    // pick a random float in the range [0,1). If it is under the odds then we reassign the color.
                    if rng.random::<f64>() < odds {
                        candidate[i] = rand::random_range(0..args.colors);
                        // print!("\tChanged index {} to color {}", i, candidate[i]);
                    }
                    // println!();
                }
            }
        }
        sentinel += 1;
    }

    if result.len() > 0 {
        return Some(result);
    }

    return None;
}

fn logistic(n: usize, mid: usize) -> f64 {
    const L: f64 = 1.0;
    const K: f64 = 2.0; // This number feels good. No evidence.
    let x0 = mid as f64;
    let x = n as f64;

    return L / (1.0 + consts::E.powf(-K * (x - x0)));
}
