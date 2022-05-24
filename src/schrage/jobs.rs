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
    pub fn c_max(&self) -> u32 {
        let mut end_times = vec![0; self.job_sequence.len()];
        let mut s = 0;
        let mut sums = vec![0; self.job_sequence.len()];

        for (i, job) in self.job_sequence.iter().enumerate() {
            println!("Number: {}, Job: {}", i, job);
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
}
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
        println!("RESULT: {}", result);
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
        println!("RESULT: {}", result);
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
        println!("RESULT: {}", result);
        assert_eq!(result, 50);
    }
}
