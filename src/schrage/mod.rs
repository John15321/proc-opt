//! $ test^{if}_{latex} + works \lambda$
//!
//! test test test
//!
//!
//!

// TODO: Zrobic JobTimeTable for Schrage z podzialem
// TODO: Dokonczyc Schrage z podzialem
// Small clean up
// Skonczyc PRa i wydac nowa wersje 0.0.2 z Schrage i Schrage z podzialem

use crate::schrage::jobs::{Job, JobList, SchrageJobTable};
use std::vec;

pub mod jobs;

/// $ test^{if}_{latex} + works $
///
/// # Arguments
///
/// * `jobs`:
///
/// returns: JobList
///
/// # Examples
///
/// ```rust
/// use proc_opt::schrage::jobs::JobList;
/// use proc_opt::schrage::jobs::Job;
/// use proc_opt::schrage::jobs::SchrageJobTable;
/// use proc_opt::schrage::schrage;
/// let expected_result = SchrageJobTable::new(JobList {
///     jobs: vec![
///         Job::new(0, 6, 17),  // 6
///         Job::new(10, 5, 7),  // 1
///         Job::new(13, 6, 26), // 2
///         Job::new(11, 7, 24), // 3
///         Job::new(20, 4, 21), // 4
///         Job::new(30, 3, 8),  // 5
///         Job::new(30, 2, 0),  // 7
///     ],
/// });
/// let js = SchrageJobTable::new(JobList {
///     jobs: vec![
///         Job::new(10, 5, 7),  // 1
///         Job::new(13, 6, 26), // 2
///         Job::new(11, 7, 24), // 3
///         Job::new(20, 4, 21), // 4
///         Job::new(30, 3, 8),  // 5
///         Job::new(0, 6, 17),  // 6
///         Job::new(30, 2, 0),  // 7
///     ],
/// });
/// let result = schrage(&js.job_list);
/// assert_eq!(result.job_list, expected_result.job_list);
/// assert_eq!(result.c_max(), 53);
/// ```
pub fn schrage(jobs: &JobList) -> SchrageJobTable {
    // N
    // A list of jobs to be completed
    let mut shortest_delivery_jobs = JobList::new(jobs.sorted_by_delivery_time());
    // G
    // A list of jobs that in a current moment are ready to run
    let mut ready_to_run = JobList::new(Vec::new());
    // Time tracking variable
    let mut t: u32 = 0;
    // The final sequence in which the jobs should be run
    let mut pi: JobList = JobList::new(Vec::new());

    // Iterate over all of the jobs until we ran out of them
    while !shortest_delivery_jobs.jobs.is_empty() || !ready_to_run.jobs.is_empty() {
        // Find all jobs that are available
        while !shortest_delivery_jobs.jobs.is_empty()
            && shortest_delivery_jobs.jobs[0].delivery_time <= t
        {
            ready_to_run
                .jobs
                .append(&mut vec![shortest_delivery_jobs.jobs[0]]);
            shortest_delivery_jobs.jobs.remove(0);
        }
        // If there are jobs that are ready to run schedule them
        if !ready_to_run.jobs.is_empty() {
            let vec_by_processing_time = JobList {
                jobs: ready_to_run.sorted_by_processing_time(),
            };
            let reversed: JobList = JobList {
                jobs: vec_by_processing_time.jobs.into_iter().rev().collect(),
            };
            let cooldown_times: Vec<Job> = reversed.sorted_by_cooldown_time();

            let max_cooldown_time = cooldown_times.last().unwrap();
            let position = ready_to_run
                .jobs
                .iter()
                .position(|&n| &n == max_cooldown_time)
                .unwrap();
            ready_to_run.jobs.remove(position);
            // Add a job to the final sequence
            pi.jobs.push(*max_cooldown_time);
            t += max_cooldown_time.processing_time;
        } else {
            // If there aren't any jobs that can be run,
            // skip to when the nearest job is available
            t = shortest_delivery_jobs.jobs[0].delivery_time;
        }
    }
    SchrageJobTable { job_list: pi }
}




/// .
///
/// # Panics
///
/// Panics if .
pub fn schrage_with_division(jobs: &JobList) -> JobList {
    // N
    let mut shortest_delivery_jobs = JobList {
        jobs: jobs.sorted_by_delivery_time(),
    };
    // G
    let mut ready_to_run = JobList { jobs: Vec::new() };
    let mut current_job: Job = Job {
        delivery_time: 0,
        processing_time: 0,
        cooldown_time: 0,
    };
    let mut t: u32 = 0;
    let mut pi: JobList = JobList { jobs: Vec::new() };

    while !shortest_delivery_jobs.jobs.is_empty() || !ready_to_run.jobs.is_empty() {
        while !shortest_delivery_jobs.jobs.is_empty()
            && shortest_delivery_jobs.jobs[0].delivery_time <= t
        {
            ready_to_run
                .jobs
                .append(&mut vec![shortest_delivery_jobs.jobs[0]]);
            let next_job = shortest_delivery_jobs.jobs.remove(0);

            if next_job.cooldown_time > current_job.cooldown_time {
                current_job.processing_time = t - next_job.delivery_time;
                t = next_job.delivery_time;

                if current_job.processing_time > 0 {
                    ready_to_run.jobs.append(&mut vec![current_job]);
                    ready_to_run.jobs = ready_to_run.sorted_by_delivery_time().clone();
                }
            }
        }

        if !ready_to_run.jobs.is_empty() {
            let cooldown_times = ready_to_run.sorted_by_cooldown_time();
            let max_cooldown_time = cooldown_times.last().unwrap();
            let position = ready_to_run
                .jobs
                .iter()
                .position(|&n| n.cooldown_time == max_cooldown_time.cooldown_time)
                .unwrap();
            ready_to_run.jobs.remove(position);

            // Add a job to the final sequence
            pi.jobs.push(*max_cooldown_time);
            t += max_cooldown_time.processing_time;
        } else {
            t = shortest_delivery_jobs.jobs[0].delivery_time;
        }
    }
    pi
}

#[cfg(test)]
mod tests {
    use crate::schrage::jobs::{Job, JobList};

    use super::*;

    #[test]
    fn test_schrage_ex1() {
        let expected_result = SchrageJobTable::new(JobList {
            jobs: vec![
                Job::new(0, 6, 17),  // 6
                Job::new(10, 5, 7),  // 1
                Job::new(13, 6, 26), // 2
                Job::new(11, 7, 24), // 3
                Job::new(20, 4, 21), // 4
                Job::new(30, 3, 8),  // 5
                Job::new(30, 2, 0),  // 7
            ],
        });
        let js = SchrageJobTable::new(JobList {
            jobs: vec![
                Job::new(10, 5, 7),  // 1
                Job::new(13, 6, 26), // 2
                Job::new(11, 7, 24), // 3
                Job::new(20, 4, 21), // 4
                Job::new(30, 3, 8),  // 5
                Job::new(0, 6, 17),  // 6
                Job::new(30, 2, 0),  // 7
            ],
        });
        let result = schrage(&js.job_list);
        assert_eq!(result.job_list, expected_result.job_list);
        assert_eq!(result.c_max(), 53);
    }

    #[test]
    fn test_schrage_ex2() {
        let expected_result = SchrageJobTable::new(JobList {
            jobs: vec![
                Job::new(1, 5, 9), // 1
                Job::new(3, 6, 8), // 5
                Job::new(1, 4, 6), // 3
                Job::new(4, 5, 4), // 2
                Job::new(7, 3, 3), // 4
                Job::new(4, 7, 1), // 6
            ],
        });
        let js = SchrageJobTable::new(JobList {
            jobs: vec![
                Job::new(1, 5, 9), // 1
                Job::new(4, 5, 4), // 2
                Job::new(1, 4, 6), // 3
                Job::new(7, 3, 3), // 4
                Job::new(3, 6, 8), // 5
                Job::new(4, 7, 1), // 6
            ],
        });
        let result = schrage(&js.job_list);
        assert_eq!(result.job_list, expected_result.job_list);
        assert_eq!(result.c_max(), 32);
    }

    #[test]
    fn test_schrage_ex3() {
        let expected_result = SchrageJobTable::new(JobList {
            jobs: vec![
                Job::new(15, 86, 700),  // 5
                Job::new(51, 52, 403),  // 7
                Job::new(144, 73, 536), // 6
                Job::new(183, 17, 641), // 9
                Job::new(226, 5, 629),  // 15
                Job::new(162, 80, 575), // 16
                Job::new(103, 68, 470), // 2
                Job::new(394, 34, 400), // 4
                Job::new(35, 37, 386),  // 13
                Job::new(39, 38, 340),  // 3
                Job::new(162, 52, 241), // 1
                Job::new(556, 23, 79),  // 18
                Job::new(567, 71, 618), // 14
                Job::new(588, 45, 632), // 17
                Job::new(598, 45, 200), // 20
                Job::new(728, 18, 640), // 10
                Job::new(715, 8, 93),   // 19
                Job::new(667, 80, 92),  // 11
                Job::new(57, 21, 76),   // 12
                Job::new(233, 68, 23),  // 8
            ],
        });
        let js = SchrageJobTable::new(JobList {
            jobs: vec![
                Job::new(162, 52, 241), // 1
                Job::new(103, 68, 470), // 2
                Job::new(39, 38, 340),  // 3
                Job::new(394, 34, 400), // 4
                Job::new(15, 86, 700),  // 5
                Job::new(144, 73, 536), // 6
                Job::new(51, 52, 403),  // 7
                Job::new(233, 68, 23),  // 8
                Job::new(183, 17, 641), // 9
                Job::new(728, 18, 640), // 10
                Job::new(667, 80, 92),  // 11
                Job::new(57, 21, 76),   // 12
                Job::new(35, 37, 386),  // 13
                Job::new(567, 71, 618), // 14
                Job::new(226, 5, 629),  // 15
                Job::new(162, 80, 575), // 16
                Job::new(588, 45, 632), // 17
                Job::new(556, 23, 79),  // 18
                Job::new(715, 8, 93),   // 19
                Job::new(598, 45, 200), // 20
            ],
        });
        let result = schrage(&js.job_list);
        assert_eq!(result.job_list, expected_result.job_list);
        assert_eq!(result.c_max(), 1399);
    }

    #[test]
    fn test_schrage_ex4() {
        let expected_result = SchrageJobTable::new(JobList {
            jobs: vec![
                Job::new(2, 20, 88),   // 8
                Job::new(5, 14, 125),  // 4
                Job::new(8, 16, 114),  // 5
                Job::new(9, 28, 94),   // 10
                Job::new(70, 4, 93),   // 2
                Job::new(71, 7, 71),   // 6
                Job::new(52, 1, 56),   // 1
                Job::new(52, 20, 56),  // 9
                Job::new(112, 22, 79), // 3
                Job::new(90, 2, 13),   // 7
            ],
        });
        let js = SchrageJobTable::new(JobList {
            jobs: vec![
                Job::new(52, 1, 56),   // 1
                Job::new(70, 4, 93),   // 2
                Job::new(112, 22, 79), // 3
                Job::new(5, 14, 125),  // 4
                Job::new(8, 16, 114),  // 5
                Job::new(71, 7, 71),   // 6
                Job::new(90, 2, 13),   // 7
                Job::new(2, 20, 88),   // 8
                Job::new(52, 20, 56),  // 9
                Job::new(9, 28, 94),   // 10
            ],
        });
        let result = schrage(&js.job_list);
        assert_eq!(result.job_list, expected_result.job_list);
        assert_eq!(result.c_max(), 213);
    }

    #[test]
    fn test_sort() {
        let js = JobList {
            jobs: vec![
                Job::new(0, 6, 17),  // 6
                Job::new(10, 5, 7),  // 1
                Job::new(13, 6, 26), // 2
                Job::new(11, 7, 24), // 3
                Job::new(20, 4, 21), // 4
                Job::new(30, 3, 8),  // 5
                Job::new(30, 2, 0),  // 7
            ],
        };
        println!("Before sort: {}", js);
        println!(
            "After sort: {}",
            JobList {
                jobs: js.sorted_by_cooldown_time()
            }
        );
        assert_eq!(true, true);
    }
}
