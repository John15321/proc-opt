// use heuristic_opt::schrage::{Job, JobSequence};
//use heuristic_opt::
// use heuristic_opt::schrage;


pub use crate::schrage::{Job, JobSequence};

fn main() {
    let js = JobSequence {
        job_sequence: vec![Job::new(0, 10, 5), Job::new(1, 10, 123)],
    };
    // println!("{}", Job::new(0, 10, 5));
    println!("{}", js);
    let out = schrage(js);
}
// (0, 10, 5)
// (30, 10, 5)
// (0, 10, 5)
// (0, 10, 5)
// (0, 10, 5)
