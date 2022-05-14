mod schrage;

use schrage::{jobs, schrage::schrage};

fn main() {
    let js = jobs::JobSequence {
        job_sequence: vec![jobs::Job::new(0, 10, 5), jobs::Job::new(1, 10, 123)],
    };
    // println!("{}", Job::new(0, 10, 5));
    println!("{}", js);
    let out = schrage(&vec![jobs::Job::new(0, 10, 5), jobs::Job::new(1, 10, 123)]);
}
// (0, 10, 5)
// (30, 10, 5)
// (0, 10, 5)
// (0, 10, 5)
// (0, 10, 5)
