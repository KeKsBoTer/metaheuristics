//! Find an approximate solution to your optimisation problem using Hill Climbing
//!
//! One of the simplest metaheuristics algorithms to understand is Hill Climbing. Just like climbing
//! a hill in real life, the aim is to get to the top. Here, we only walk forward if the next step
//! we take is higher than our current position on the hill. In other words, at each iteration, we
//! only accept a candidate solution as our best so far, if it ranks higher than our current best
//! solution.
//!
//! **Note: As Hill Climbing restricts our movement to only ever going up, we guarantee that we
//! will sometimes get stuck at a local maximum.**
//!
//! For more info on Hill Climbing, please see the [Hill
//! Climbing](https://wikipedia.org/wiki/Hill_climbing) Wikipedia article.
//!
//!# Examples
//!
//!```
//!let solution = metaheuristics::hill_climbing::solve(&mut problem, runtime);
//!```

pub mod random_restarts;

use super::Metaheuristics;
use time::{Duration, PreciseTime};

/// Returns an approximate solution to your optimisation problem using Hill Climbing
///
///# Parameters
///
/// `problem` is the type that implements the `Metaheuristics` trait.
///
/// `runtime` is a `time::Duration` specifying how long to spend searching for a solution.
///
///# Examples
///
///```
///let solution = metaheuristics::hill_climbing::solve(&mut problem, runtime);
///```
pub fn solve<T>(problem: &mut Metaheuristics<T>, runtime: Duration) -> T {
    let mut best_candidate = problem.generate_candidate();
    let start_time         = PreciseTime::now();

    while start_time.to(PreciseTime::now()) < runtime {
        let next_candidate = problem.tweak_candidate(&best_candidate);

        if problem.rank_candidate(&next_candidate) > problem.rank_candidate(&best_candidate) {
            best_candidate = next_candidate;
        }
    }

    best_candidate
}
