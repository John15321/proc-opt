use std::cmp::Ordering;

use super::jobs::{Job, JobSequence};

pub fn schrage(jobs: &Vec<Job>) -> JobSequence {
    let shortest_delivery_jobs = jobs.clone().sort_by(|a, b| {
        if a.delivery_time < b.delivery_time {
            Ordering::Less
        } else if a.delivery_time == b.delivery_time {
            Ordering::Equal
        } else {
            Ordering::Greater
        }
    });
    println!("min delivery_time: {:?}", shortest_delivery_jobs);

    return JobSequence {
        job_sequence: jobs.clone(),
    };
}
