use std::vec;

use super::jobs::JobSequence;

#[allow(dead_code)]
pub fn schrage(jobs: &JobSequence) -> JobSequence {
    // N
    let mut shortest_delivery_jobs = JobSequence {
        job_sequence: jobs.get_by_delivery_time(),
    };
    // G
    let mut ready_to_run = JobSequence {
        job_sequence: Vec::new(),
    };
    let mut t: u32 = 0;
    let mut pi: JobSequence = JobSequence {
        job_sequence: Vec::new(),
    };

    while shortest_delivery_jobs.job_sequence.len() != 0 || ready_to_run.job_sequence.len() != 0 {
        while shortest_delivery_jobs.job_sequence.len() != 0
            && shortest_delivery_jobs.job_sequence[0].delivery_time <= t
        {
            ready_to_run
                .job_sequence
                .append(&mut vec![shortest_delivery_jobs.job_sequence[0].clone()]);
            shortest_delivery_jobs.job_sequence.remove(0);
        }
        if ready_to_run.job_sequence.len() != 0 {
            let cooldown_times = ready_to_run.get_by_cooldown_time();
            let max_cooldown_time = cooldown_times.last().unwrap();
            let position = ready_to_run
                .job_sequence
                .iter()
                .position(|&n| n.cooldown_time == max_cooldown_time.cooldown_time)
                .unwrap();
            ready_to_run.job_sequence.remove(position);

            // Add a job to the final sequence
            pi.job_sequence.push(max_cooldown_time.clone());
            t = t + max_cooldown_time.processing_time;
        } else {
            t = shortest_delivery_jobs.job_sequence[0].delivery_time;
        }
    }
    return pi;
}

#[cfg(test)]
mod tests {
    use crate::schrage::jobs;

    use super::*;

    #[test]
    fn test_schrage_ex1() {
        let expected_result = JobSequence {
            job_sequence: vec![
                jobs::Job::new(0, 6, 17),  // 6
                jobs::Job::new(10, 5, 7),  // 1
                jobs::Job::new(13, 6, 26), // 2
                jobs::Job::new(11, 7, 24), // 3
                jobs::Job::new(20, 4, 21), // 4
                jobs::Job::new(30, 3, 8),  // 5
                jobs::Job::new(30, 2, 0),  // 7
            ],
        };
        let js = JobSequence {
            job_sequence: vec![
                jobs::Job::new(10, 5, 7),  // 1
                jobs::Job::new(13, 6, 26), // 2
                jobs::Job::new(11, 7, 24), // 3
                jobs::Job::new(20, 4, 21), // 4
                jobs::Job::new(30, 3, 8),  // 5
                jobs::Job::new(0, 6, 17),  // 6
                jobs::Job::new(30, 2, 0),  // 7
            ],
        };
        let result = schrage(&js);
        assert_eq!(result, expected_result);
        assert_eq!(result.c_max(), 53);
    }
}
