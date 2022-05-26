use std::{cmp::Ordering, fmt, vec};

#[derive(Copy, Clone, Debug)]
pub struct Job {
    pub delivery_time: u32,   // r
    pub processing_time: u32, // p
    pub cooldown_time: u32,   // q
}

impl Job {
    #[allow(dead_code)]
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
pub struct JobSequence {
    pub job_sequence: Vec<Job>,
}

impl fmt::Display for JobSequence {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for i in &self.job_sequence {
            writeln!(f, "{}", i)?;
        }
        Ok(())
    }
}

impl JobSequence {
    #[allow(dead_code)]
    pub fn c_max(&self) -> u32 {
        let mut end_times = vec![0; self.job_sequence.len()];
        let mut s = 0;
        let mut sums = vec![0; self.job_sequence.len()];

        for (i, job) in self.job_sequence.iter().enumerate() {
            if job.delivery_time > s {
                s = job.delivery_time + job.processing_time;
            } else {
                s = s + job.processing_time;
            }
            end_times[i] = s;
        }

        for (i, job) in self.job_sequence.iter().enumerate() {
            sums[i] = job.cooldown_time + end_times[i];
        }
        *sums.iter().max().unwrap()
    }

    #[allow(dead_code)]
    pub fn get_by_delivery_time(&self) -> Vec<Job> {
        let mut by_delivery_time = self.job_sequence.clone();
        by_delivery_time.sort_by(|a, b| {
            if a.delivery_time < b.delivery_time {
                Ordering::Less
            } else if a.delivery_time == b.delivery_time {
                Ordering::Equal
            } else {
                Ordering::Greater
            }
        });
        by_delivery_time
    }

    #[allow(dead_code)]
    pub fn get_by_processing_time(&self) -> Vec<Job> {
        let mut by_processing_time = self.job_sequence.clone();
        by_processing_time.sort_by(|a, b| {
            if a.processing_time < b.processing_time {
                Ordering::Less
            } else if a.processing_time == b.processing_time {
                Ordering::Equal
            } else {
                Ordering::Greater
            }
        });
        by_processing_time
    }

    #[allow(dead_code)]
    pub fn get_by_cooldown_time(&self) -> Vec<Job> {
        let mut by_cooldown_time = self.job_sequence.clone();
        by_cooldown_time.sort_by(|a, b| {
            if a.cooldown_time < b.cooldown_time {
                Ordering::Less
            } else if a.cooldown_time == b.cooldown_time {
                Ordering::Equal
            } else {
                Ordering::Greater
            }
        });
        by_cooldown_time
    }
}

impl PartialEq for JobSequence {
    fn eq(&self, other: &Self) -> bool {
        if self.job_sequence.len() != other.job_sequence.len() {
            return false;
        }
        for (i, j) in self.job_sequence.iter().enumerate() {
            if j != &other.job_sequence[i] {
                return false;
            }
        }
        return true;
    }
}

// #[derive(Debug, Clone)]
// pub struct JobTimeTable {

// }

#[cfg(test)]
mod tests {
    use crate::schrage::jobs;

    use super::*;

    #[test]
    fn test_c_max_ex1() {
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
        let result = js.c_max();
        assert_eq!(result, 58);
    }

    #[test]
    fn test_c_max_ex2() {
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
        let result = js.c_max();
        assert_eq!(result, 53);
    }

    #[test]
    fn test_c_max_ex3() {
        let js = JobSequence {
            job_sequence: vec![
                jobs::Job::new(0, 6, 17),  // 6
                jobs::Job::new(11, 7, 24), // 3
                jobs::Job::new(13, 6, 26), // 2
                jobs::Job::new(20, 4, 21), // 4
                jobs::Job::new(10, 5, 7),  // 1
                jobs::Job::new(30, 3, 8),  // 5
                jobs::Job::new(30, 2, 0),  // 7
            ],
        };
        let result = js.c_max();
        assert_eq!(result, 50);
    }

    #[test]
    fn test_c_max_ex4() {
        let js = JobSequence {
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
        let result = js.c_max();
        assert_eq!(result, 213);
    }

    #[test]
    fn test_c_max_ex5() {
        let js = JobSequence {
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
        let result = js.c_max();
        assert_eq!(result, 1399);
    }
}
