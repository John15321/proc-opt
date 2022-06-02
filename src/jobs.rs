//! Implements Job structs used for processing data by scheduling algorithms.

use std::collections::HashMap;
use std::{fmt, vec};

#[derive(Copy, Clone, Debug)]
pub struct Job {
    pub delivery_time: u32,   // r
    pub processing_time: u32, // p
    pub cooldown_time: u32,   // q
}

impl Job {
    pub fn new(delivery_time: u32, processing_time: u32, cooldown_time: u32) -> Job {
        Job {
            delivery_time,
            processing_time,
            cooldown_time,
        }
    }

    #[allow(dead_code)]
    pub fn total_time(&self) -> u32 {
        self.delivery_time + self.processing_time + self.cooldown_time
    }
}

impl fmt::Display for Job {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "({}, {}, {})",
            self.delivery_time, self.processing_time, self.cooldown_time
        )
    }
}

impl PartialEq for Job {
    fn eq(&self, other: &Self) -> bool {
        self.delivery_time == other.delivery_time
            && self.processing_time == other.processing_time
            && self.cooldown_time == other.cooldown_time
    }
}

#[derive(Debug, Clone)]
pub struct JobList {
    pub jobs: Vec<Job>,
}

impl fmt::Display for JobList {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for i in &self.jobs {
            writeln!(f, "{}", i)?;
        }
        Ok(())
    }
}

impl JobList {
    /// Creates a new [`JobList`].
    pub fn new(jobs: Vec<Job>) -> JobList {
        JobList { jobs }
    }

    /// Returns the Job List sorted by delivery time of this [`JobList`].
    #[allow(dead_code)]
    pub fn sorted_by_delivery_time(&self) -> Vec<Job> {
        let mut by_delivery_time = self.jobs.clone();
        by_delivery_time.sort_by_key(|a| a.delivery_time);
        by_delivery_time
    }

    /// Returns the Job List sorted by processing time of this [`JobList`].
    #[allow(dead_code)]
    pub fn sorted_by_processing_time(&self) -> Vec<Job> {
        let mut by_processing_time = self.jobs.clone();
        by_processing_time.sort_by_key(|a| a.processing_time);
        by_processing_time
    }

    /// Returns the Job List sorted by cooldown time of this [`JobList`].
    #[allow(dead_code)]
    pub fn sorted_by_cooldown_time(&self) -> Vec<Job> {
        let mut by_cooldown_time = self.jobs.clone();
        by_cooldown_time.sort_by_key(|a| a.cooldown_time);
        by_cooldown_time
    }
}

impl PartialEq for JobList {
    fn eq(&self, other: &Self) -> bool {
        if self.jobs.len() != other.jobs.len() {
            return false;
        }
        for (i, j) in self.jobs.iter().enumerate() {
            if j != &other.jobs[i] {
                return false;
            }
        }
        true
    }
}

pub struct SchrageJobTable {
    pub job_list: JobList,
}

impl SchrageJobTable {
    pub fn new(job_list: JobList) -> SchrageJobTable {
        SchrageJobTable { job_list }
    }

    /// Returns the c max of this [`SchrageJobTable`].
    ///
    /// # Panics
    ///
    /// Panics if the job list is empty.
    pub fn c_max(&self) -> u32 {
        let mut end_times = vec![0; self.job_list.jobs.len()];
        let mut s = 0;
        let mut sums = vec![0; self.job_list.jobs.len()];

        for (i, job) in self.job_list.jobs.iter().enumerate() {
            if job.delivery_time > s {
                s = job.delivery_time + job.processing_time;
            } else {
                s += job.processing_time;
            }
            end_times[i] = s;
        }

        for (i, job) in self.job_list.jobs.iter().enumerate() {
            sums[i] = job.cooldown_time + end_times[i];
        }
        *sums.iter().max().unwrap()
    }
}

pub struct PartTimeSchrageJobTable {
    pub job_list: JobList,
    pub time_table: HashMap<u32, u32>,
}

impl PartTimeSchrageJobTable {
    pub fn c_max_wip(&self) -> u32 {
        let mut end_times = vec![0; self.job_list.jobs.len()];
        let mut s = 0;
        let mut sums = vec![0; self.job_list.jobs.len()];

        for (i, job) in self.job_list.jobs.iter().enumerate() {
            if job.delivery_time > s {
                s = job.delivery_time + job.processing_time;
            } else {
                s += job.processing_time;
            }
            end_times[i] = s;
        }

        for (i, job) in self.job_list.jobs.iter().enumerate() {
            sums[i] = job.cooldown_time + end_times[i];
        }
        *sums.iter().max().unwrap()
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_c_max_ex1() {
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
        let result = js.c_max();
        assert_eq!(result, 58);
    }

    #[test]
    fn test_c_max_ex2() {
        let js = SchrageJobTable::new(JobList {
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
        let result = js.c_max();
        assert_eq!(result, 53);
    }

    #[test]
    fn test_c_max_ex3() {
        let js = SchrageJobTable::new(JobList {
            jobs: vec![
                Job::new(0, 6, 17),  // 6
                Job::new(11, 7, 24), // 3
                Job::new(13, 6, 26), // 2
                Job::new(20, 4, 21), // 4
                Job::new(10, 5, 7),  // 1
                Job::new(30, 3, 8),  // 5
                Job::new(30, 2, 0),  // 7
            ],
        });
        let result = js.c_max();
        assert_eq!(result, 50);
    }

    #[test]
    fn test_c_max_ex4() {
        let js = SchrageJobTable::new(JobList {
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
        let result = js.c_max();
        assert_eq!(result, 213);
    }

    #[test]
    fn test_c_max_ex5() {
        let js = SchrageJobTable::new(JobList {
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
        let result = js.c_max();
        assert_eq!(result, 1399);
    }
}
