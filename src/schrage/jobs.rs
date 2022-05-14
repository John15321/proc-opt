use std::fmt;

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
    pub fn c_max() -> u32 {
        let c_max: u32 = 0;

        c_max
    }
}
