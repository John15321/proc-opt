#[derive(Debug, Clone)]
pub struct JobSequence {
    pub job_sequence: Vec<Job>,
}

impl fmt::Display for JobSequence {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut final_output = String::new();
        for i in &self.job_sequence {
            final_output.push_str(i.to_string().as_str());
            final_output.push_str("\n");
        }
        write!(f, "{}", final_output)
    }
}

impl JobSequence {
    pub fn c_max() -> u32 {
        let c_max: u32 = 0;

        c_max
    }
}
