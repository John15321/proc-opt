use crate::schrage::jobs::Job;
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
            let vec_by_processing_time = JobSequence {
                job_sequence: ready_to_run.get_by_processing_time(),
            };
            let reversed: JobSequence = JobSequence {
                job_sequence: vec_by_processing_time
                    .job_sequence
                    .into_iter()
                    .rev()
                    .collect(),
            };
            let cooldown_times: Vec<Job> = reversed.get_by_cooldown_time();

            let max_cooldown_time = cooldown_times.last().unwrap();
            let position = ready_to_run
                .job_sequence
                .iter()
                .position(|&n| &n == max_cooldown_time)
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

#[allow(dead_code)]
pub fn schrage_with_division(jobs: &JobSequence) -> JobSequence {
    // N
    let mut shortest_delivery_jobs = JobSequence {
        job_sequence: jobs.get_by_delivery_time(),
    };
    // G
    let mut ready_to_run = JobSequence {
        job_sequence: Vec::new(),
    };
    let mut current_job: Job = Job {
        delivery_time: 0,
        processing_time: 0,
        cooldown_time: 0,
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
            let next_job = shortest_delivery_jobs.job_sequence.remove(0);

            if next_job.cooldown_time > current_job.cooldown_time {
                current_job.processing_time = t - next_job.delivery_time;
                t = next_job.delivery_time;

                if current_job.processing_time > 0 {
                    ready_to_run
                        .job_sequence
                        .append(&mut vec![current_job.clone()]);
                    ready_to_run.job_sequence = ready_to_run.get_by_delivery_time().clone();
                }
            }
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

    #[test]
    fn test_schrage_ex2() {
        let expected_result = JobSequence {
            job_sequence: vec![
                jobs::Job::new(1, 5, 9), // 1
                jobs::Job::new(3, 6, 8), // 5
                jobs::Job::new(1, 4, 6), // 3
                jobs::Job::new(4, 5, 4), // 2
                jobs::Job::new(7, 3, 3), // 4
                jobs::Job::new(4, 7, 1), // 6
            ],
        };
        let js = JobSequence {
            job_sequence: vec![
                jobs::Job::new(1, 5, 9), // 1
                jobs::Job::new(4, 5, 4), // 2
                jobs::Job::new(1, 4, 6), // 3
                jobs::Job::new(7, 3, 3), // 4
                jobs::Job::new(3, 6, 8), // 5
                jobs::Job::new(4, 7, 1), // 6
            ],
        };
        let result = schrage(&js);
        assert_eq!(result, expected_result);
        assert_eq!(result.c_max(), 32);
    }

    #[test]
    fn test_schrage_ex3() {
        let expected_result = JobSequence {
            job_sequence: vec![
                jobs::Job::new(15, 86, 700),  // 5
                jobs::Job::new(51, 52, 403),  // 7
                jobs::Job::new(144, 73, 536), // 6
                jobs::Job::new(183, 17, 641), // 9
                jobs::Job::new(226, 5, 629),  // 15
                jobs::Job::new(162, 80, 575), // 16
                jobs::Job::new(103, 68, 470), // 2
                jobs::Job::new(394, 34, 400), // 4
                jobs::Job::new(35, 37, 386),  // 13
                jobs::Job::new(39, 38, 340),  // 3
                jobs::Job::new(162, 52, 241), // 1
                jobs::Job::new(556, 23, 79),  // 18
                jobs::Job::new(567, 71, 618), // 14
                jobs::Job::new(588, 45, 632), // 17
                jobs::Job::new(598, 45, 200), // 20
                jobs::Job::new(728, 18, 640), // 10
                jobs::Job::new(715, 8, 93),   // 19
                jobs::Job::new(667, 80, 92),  // 11
                jobs::Job::new(57, 21, 76),   // 12
                jobs::Job::new(233, 68, 23),  // 8
            ],
        };
        let js = JobSequence {
            job_sequence: vec![
                jobs::Job::new(162, 52, 241), // 1
                jobs::Job::new(103, 68, 470), // 2
                jobs::Job::new(39, 38, 340),  // 3
                jobs::Job::new(394, 34, 400), // 4
                jobs::Job::new(15, 86, 700),  // 5
                jobs::Job::new(144, 73, 536), // 6
                jobs::Job::new(51, 52, 403),  // 7
                jobs::Job::new(233, 68, 23),  // 8
                jobs::Job::new(183, 17, 641), // 9
                jobs::Job::new(728, 18, 640), // 10
                jobs::Job::new(667, 80, 92),  // 11
                jobs::Job::new(57, 21, 76),   // 12
                jobs::Job::new(35, 37, 386),  // 13
                jobs::Job::new(567, 71, 618), // 14
                jobs::Job::new(226, 5, 629),  // 15
                jobs::Job::new(162, 80, 575), // 16
                jobs::Job::new(588, 45, 632), // 17
                jobs::Job::new(556, 23, 79),  // 18
                jobs::Job::new(715, 8, 93),   // 19
                jobs::Job::new(598, 45, 200), // 20
            ],
        };
        let result = schrage(&js);
        assert_eq!(result, expected_result);
        assert_eq!(result.c_max(), 1399);
    }

    #[test]
    fn test_schrage_ex4() {
        let expected_result = JobSequence {
            job_sequence: vec![
                jobs::Job::new(2, 20, 88),   // 8
                jobs::Job::new(5, 14, 125),  // 4
                jobs::Job::new(8, 16, 114),  // 5
                jobs::Job::new(9, 28, 94),   // 10
                jobs::Job::new(70, 4, 93),   // 2
                jobs::Job::new(71, 7, 71),   // 6
                jobs::Job::new(52, 1, 56),   // 1
                jobs::Job::new(52, 20, 56),  // 9
                jobs::Job::new(112, 22, 79), // 3
                jobs::Job::new(90, 2, 13),   // 7
            ],
        };
        let js = JobSequence {
            job_sequence: vec![
                jobs::Job::new(52, 1, 56),   // 1
                jobs::Job::new(70, 4, 93),   // 2
                jobs::Job::new(112, 22, 79), // 3
                jobs::Job::new(5, 14, 125),  // 4
                jobs::Job::new(8, 16, 114),  // 5
                jobs::Job::new(71, 7, 71),   // 6
                jobs::Job::new(90, 2, 13),   // 7
                jobs::Job::new(2, 20, 88),   // 8
                jobs::Job::new(52, 20, 56),  // 9
                jobs::Job::new(9, 28, 94),   // 10
            ],
        };
        let result = schrage(&js);
        assert_eq!(result, expected_result);
        assert_eq!(result.c_max(), 213);
    }

    #[test]
    fn test_sorty() {
        let js = JobSequence {
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
        println!("Before sort: {}", js);
        println!(
            "After sort: {}",
            JobSequence {
                job_sequence: js.get_by_cooldown_time()
            }
        );
        assert_eq!(true, true);
    }
}
