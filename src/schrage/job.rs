
#[derive(Copy, Clone, Debug)]
pub struct Job {
    delivery_time: u32,   // r
    processing_time: u32, // p
    cooldown_time: u32,   // q
}

impl Job {
    pub fn new(delivery_time: u32, processing_time: u32, cooldown_time: u32) -> Job {
        Job {
            delivery_time: delivery_time,
            processing_time: processing_time,
            cooldown_time: cooldown_time,
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
