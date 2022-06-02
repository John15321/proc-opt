use crate::jobs::{Job, JobList, SchrageJobTable};
use crate::schrage::{part_time_schrage, schrage};
use std::{cmp, vec};

pub fn carlier(jobs: &JobList, upper_bound: &u32) -> Result<SchrageJobTable, _> {
    let result: SchrageJobTable = schrage(&jobs);
    let pi: JobList = result.job_list.clone();
    let u: u32 = result.c_max();

    if u < upper_bound {
        upper_bound = u;
    }

    let b: u32 = find_b(pi, u);
    let a: u32 = find_a(pi, u, b);
    let c: i32 = find_c(pi, u, b, a);

    if c == -1 {
        return;
    }

    let mut rj: u32 = u32::MAX;
    let mut pj: u32 = 0;
    let mut qj: u32 = u32::MAX;

    for i in (c + 1)..=b {
        if pi.jobs[i].delivery_time < rj {
            rj = pi.jobs[i].delivery_time;
        }

        if pi.jobs[i].cooldown_time < qj {
            qj = pi.jobs[i].cooldown_time;
        }

        pj += pi.jobs[i].processing_time;
    }

    let c_job_delivery: u32 = pi.jobs[c].delivery_time;
    pi.jobs[c].delivery_time = cmp::max(c_job_delivery, rj + pj);

    let lower_bound: u32 = part_time_schrage(&jobs);
    if lower_bound < upper_bound {
        carlier(&pi, &upper_bound);
    }

    pi.jobs[c].delivery_time = c_job_delivery;

    let c_job_cooldown: u32 = pi.jobs[c].cooldown_time;
    pi.jobs[c].cooldown_time = cmp::max(c_job_cooldown, pj + qj);
    lower_bound = part_time_schrage(&jobs);

    if lower_bound < upper_bound {
        carlier(&pi, &upper_bound)
    }
    pi.jobs[c].cooldown_time = c_job_cooldown;
}

fn find_b(pi: JobList, c_max: u32) -> u32 {
    let mut b_value: i32 = -1;
    let mut t: u32 = pi.jobs[0].delivery_time;

    for i in 0..pi.jobs.len() {
        let mut current_job: Job = pi.jobs[i].clone();
        t = cmp::max(t, current_job.delivery_time) + current_job.processing_time;

        if c_max == (current_job.cooldown_time + t) {
            b_value = i;
        }
    }
    b_value
}

fn find_a(pi: JobList, c_max: u32, b_value: u32) -> u32 {
    let mut sum: u32 = 0;
    let mut a_value: i32 = -1;
    let mut t: u32 = pi.jobs[0].delivery_time;

    for i in 0..pi.jobs.len() {
        let mut current_job: Job = pi.jobs[i].clone();
        t = cmp::max(t, current_job.delivery_time) + current_job.processing_time;

        if a == -1 {
            sum = 0;

            for j in i..=b_value {
                sum += pi.jobs[j].processing_time;
            }
            sum += pi.jobs[b_value].cooldown_time;

            if c_max == (current_job.delivery_time + sum) {
                a_value = i;
            }
        }
    }
    a_value
}

fn find_c(pi: JobList, c_max: u32, b_value: u32, a_value: u32) -> i32 {
    let mut c: i32 = -1;
    let mut t: u32 = pi.jobs[0].delivery_time;

    for i in a..=b {
        if pi.jobs[i].cooldown_time < pi.jobs[b].cooldown_time {
            c = i32::from(i);
        }
    }
}
