use crate::schrage::jobs::{Job, JobList, SchrageJobTable};
use std::{cmp, vec};

pub mod jobs;

/// Schrage algorithm.
///
/// # Arguments
///
/// * `jobs`: A vector of jobs.
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

/// Part time Schrage algorithm.
///
/// # Panics
///
/// Panics if empty job list.
///
/// # Example
///
/// ```
/// use proc_opt::schrage::jobs::JobList;
/// use proc_opt::schrage::jobs::Job;
/// use proc_opt::schrage::jobs::SchrageJobTable;
/// use proc_opt::schrage::part_time_schrage;
/// let js = JobList::new(vec![
///     Job::new(0, 27, 78),
///     Job::new(140, 7, 67),
///     Job::new(14, 36, 54),
///     Job::new(133, 76, 5),
/// ]);
/// let result = part_time_schrage(&js);
/// assert_eq!(result, 221)
/// ```
pub fn part_time_schrage(jobs: &JobList) -> u32 {
    // N
    let mut shortest_delivery_jobs = JobList::new(jobs.sorted_by_delivery_time());
    // G
    let mut ready_to_run = JobList::new(Vec::new());
    let mut current_job = Job::new(0, 0, 0);
    let mut t: u32 = 0;
    let mut c_max: u32 = 0;
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
            current_job = ready_to_run.jobs.remove(position);

            // Add a job to the final sequence
            pi.jobs.push(*max_cooldown_time);
            t += max_cooldown_time.processing_time;
            c_max = cmp::max(t + max_cooldown_time.cooldown_time, c_max)
        } else {
            t = shortest_delivery_jobs.jobs[0].delivery_time;
        }
    }
    c_max
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

    #[test]
    fn test_part_time_schrage1() {
        let js = JobList::new(vec![
            Job::new(0, 27, 78),
            Job::new(140, 7, 67),
            Job::new(14, 36, 54),
            Job::new(133, 76, 5),
        ]);
        let result = part_time_schrage(&js);
        assert_eq!(result, 221)
    }

    #[test]
    fn test_part_time_schrage2() {
        let js = JobList::new(vec![
            Job::new(8, 68, 984),
            Job::new(747, 60, 1241),
            Job::new(811, 78, 56),
            Job::new(1760, 58, 1558),
            Job::new(860, 16, 319),
            Job::new(1549, 28, 927),
            Job::new(1010, 96, 749),
            Job::new(738, 37, 844),
            Job::new(599, 20, 1170),
            Job::new(446, 53, 1509),
            Job::new(1363, 36, 19),
            Job::new(1277, 14, 685),
            Job::new(1574, 98, 1472),
            Job::new(1886, 3, 1571),
            Job::new(591, 21, 1587),
            Job::new(714, 25, 1490),
            Job::new(1881, 43, 1647),
            Job::new(983, 62, 514),
            Job::new(858, 8, 1215),
            Job::new(634, 7, 587),
            Job::new(784, 14, 1897),
            Job::new(1893, 22, 1878),
            Job::new(308, 89, 1039),
            Job::new(1892, 91, 1815),
            Job::new(1024, 75, 1602),
            Job::new(1467, 59, 378),
            Job::new(1830, 3, 1173),
            Job::new(167, 25, 702),
            Job::new(357, 3, 416),
            Job::new(1739, 68, 71),
            Job::new(1810, 58, 1220),
            Job::new(453, 62, 393),
            Job::new(462, 60, 22),
            Job::new(332, 25, 1512),
            Job::new(845, 96, 1176),
            Job::new(522, 80, 513),
            Job::new(1110, 61, 1854),
            Job::new(484, 32, 570),
            Job::new(545, 91, 274),
            Job::new(64, 67, 74),
            Job::new(90, 9, 1423),
            Job::new(1013, 67, 1567),
            Job::new(1509, 86, 878),
            Job::new(238, 12, 285),
            Job::new(1226, 23, 1767),
            Job::new(83, 35, 22),
            Job::new(626, 97, 63),
            Job::new(6, 24, 707),
            Job::new(507, 31, 1294),
            Job::new(638, 98, 1528),
        ]);
        let result = part_time_schrage(&js);
        assert_eq!(result, 3820);
    }

    #[test]
    fn test_part_time_schrage3() {
        let js = JobList::new(vec![
            Job::new(162, 52, 241),
            Job::new(103, 68, 470),
            Job::new(39, 38, 340),
            Job::new(394, 34, 400),
            Job::new(15, 86, 700),
            Job::new(144, 73, 536),
            Job::new(51, 52, 403),
            Job::new(233, 68, 23),
            Job::new(183, 17, 641),
            Job::new(728, 18, 640),
            Job::new(667, 80, 92),
            Job::new(57, 21, 76),
            Job::new(35, 37, 386),
            Job::new(567, 71, 618),
            Job::new(226, 5, 629),
            Job::new(162, 80, 575),
            Job::new(588, 45, 632),
            Job::new(556, 23, 79),
            Job::new(715, 8, 93),
            Job::new(598, 45, 200),
        ]);
        let result = part_time_schrage(&js);
        assert_eq!(result, 1386);
    }

    #[test]
    fn test_part_time_schrage4() {
        let js = JobList::new(vec![
            Job::new(219, 5, 276),
            Job::new(84, 13, 103),
            Job::new(336, 35, 146),
            Job::new(271, 62, 264),
            Job::new(120, 33, 303),
            Job::new(299, 14, 328),
            Job::new(106, 46, 91),
            Job::new(181, 93, 97),
            Job::new(263, 13, 168),
            Job::new(79, 60, 235),
        ]);
        let result = part_time_schrage(&js);
        assert_eq!(result, 641);
    }
}
