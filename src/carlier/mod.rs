//!
//!
//!
//!
//!
//!
//!
//!
//!
//!
//!
//!

use crate::jobs::{Job, JobList, SchrageJobTable};
use crate::schrage::{part_time_schrage, schrage};
use std::cmp;

pub fn carlier(jobs: &mut JobList, upper_bound: &mut u32) {
    let result: SchrageJobTable = schrage(jobs);
    let mut pi: JobList = result.job_list.clone();
    let c_max_from_schrage: u32 = result.c_max();

    if c_max_from_schrage < *upper_bound {
        *upper_bound = c_max_from_schrage;
    }

    let critical_path_end_index: u32 = find_critical_path_end(pi.clone(), c_max_from_schrage);
    let critical_path_start_index: u32 =
        find_critical_path_start(pi.clone(), c_max_from_schrage, critical_path_end_index);
    let critical_job_index: i32 = find_critical_job(
        pi.clone(),
        critical_path_end_index,
        critical_path_start_index,
    );

    if critical_job_index == -1 {
        return;
    }

    let mut rj: u32 = u32::MAX;
    let mut pj: u32 = 0;
    let mut qj: u32 = u32::MAX;

    for i in (critical_job_index as usize + 1)..=critical_path_end_index as usize {
        if pi.jobs[i as usize].delivery_time < rj {
            rj = pi.jobs[i as usize].delivery_time;
        }

        if pi.jobs[i as usize].cooldown_time < qj {
            qj = pi.jobs[i as usize].cooldown_time;
        }

        pj += pi.jobs[i as usize].processing_time;
    }

    let c_job_delivery: u32 = pi.jobs[critical_job_index as usize].delivery_time;
    pi.jobs[critical_job_index as usize].delivery_time = cmp::max(c_job_delivery, rj + pj);

    let mut lower_bound: u32 = part_time_schrage(jobs);
    if lower_bound < *upper_bound {
        carlier(&mut pi, upper_bound);
    }

    pi.jobs[critical_job_index as usize].delivery_time = c_job_delivery;

    let c_job_cooldown: u32 = pi.jobs[critical_job_index as usize].cooldown_time;
    pi.jobs[critical_job_index as usize].cooldown_time = cmp::max(c_job_cooldown, pj + qj);
    lower_bound = part_time_schrage(jobs);

    if lower_bound < *upper_bound {
        carlier(&mut pi, upper_bound)
    }
    pi.jobs[critical_job_index as usize].cooldown_time = c_job_cooldown;
}

fn find_critical_path_end(pi: JobList, c_max: u32) -> u32 {
    let mut b_value: i32 = -1;
    let mut t: u32 = pi.jobs[0].delivery_time;

    for i in 0..pi.jobs.len() {
        let current_job: Job = pi.jobs[i];
        t = cmp::max(t, current_job.delivery_time) + current_job.processing_time;

        if c_max == (current_job.cooldown_time + t) {
            b_value = i as i32;
        }
    }
    b_value as u32
}

fn find_critical_path_start(pi: JobList, c_max: u32, b_value: u32) -> u32 {
    let mut sum: u32;
    let mut a_value: i32 = -1;
    let mut t: u32 = pi.jobs[0].delivery_time;

    for i in 0..pi.jobs.len() {
        let current_job: Job = pi.jobs[i];
        t = cmp::max(t, current_job.delivery_time) + current_job.processing_time;

        if a_value == -1 {
            sum = 0;

            for j in i..=b_value as usize {
                sum += pi.jobs[j].processing_time;
            }
            sum += pi.jobs[b_value as usize].cooldown_time;

            if c_max == (current_job.delivery_time + sum) {
                a_value = i as i32;
            }
        }
    }
    a_value as u32
}

fn find_critical_job(pi: JobList, b_value: u32, a_value: u32) -> i32 {
    let mut c_value: i32 = -1;

    for i in a_value..=b_value {
        if pi.jobs[i as usize].cooldown_time < pi.jobs[b_value as usize].cooldown_time {
            c_value = i as i32;
        }
    }
    c_value
}

#[cfg(test)]
mod tests {
    use crate::jobs::{Job, JobList};

    use super::*;

    #[test]
    fn test_carlier_ex1() {
        let mut js = JobList::new(vec![
            Job::new(592, 82, 321),
            Job::new(547, 18, 687),
            Job::new(284, 11, 219),
            Job::new(568, 46, 507),
            Job::new(189, 76, 604),
            Job::new(465, 52, 577),
            Job::new(234, 53, 732),
            Job::new(391, 49, 718),
            Job::new(205, 10, 64),
            Job::new(157, 29, 176),
            Job::new(505, 40, 637),
            Job::new(211, 21, 326),
            Job::new(518, 57, 645),
            Job::new(625, 15, 53),
            Job::new(500, 51, 66),
            Job::new(114, 1, 506),
            Job::new(454, 91, 167),
            Job::new(174, 75, 319),
            Job::new(340, 56, 480),
            Job::new(184, 61, 69),
        ]);
        let mut result: u32 = u32::MAX;
        carlier(&mut js, &mut result);
        assert_eq!(result, 1267)
    }

    #[test]
    fn test_carlier_ex2() {
        let mut js = JobList::new(vec![
            Job::new(0, 27, 78),
            Job::new(140, 7, 67),
            Job::new(14, 36, 54),
            Job::new(133, 76, 5),
        ]);
        let mut result: u32 = u32::MAX;
        carlier(&mut js, &mut result);
        assert_eq!(result, 228)
    }

    #[test]
    fn test_carlier_ex3() {
        let mut js = JobList::new(vec![
            Job::new(8, 37, 1828),
            Job::new(1533, 28, 881),
            Job::new(1818, 98, 450),
            Job::new(1100, 74, 1189),
            Job::new(272, 81, 1204),
            Job::new(985, 57, 781),
            Job::new(1325, 30, 417),
            Job::new(989, 48, 266),
            Job::new(1622, 93, 1322),
            Job::new(1322, 37, 1486),
            Job::new(1166, 74, 732),
            Job::new(374, 77, 148),
            Job::new(954, 42, 1643),
            Job::new(644, 55, 2),
            Job::new(160, 67, 608),
            Job::new(1233, 92, 1781),
            Job::new(499, 7, 383),
            Job::new(1407, 25, 991),
            Job::new(1632, 41, 1472),
            Job::new(678, 97, 1337),
            Job::new(696, 10, 1587),
            Job::new(1531, 35, 92),
            Job::new(952, 99, 131),
            Job::new(490, 90, 215),
            Job::new(1459, 62, 1036),
            Job::new(242, 18, 1327),
            Job::new(660, 33, 645),
            Job::new(1586, 10, 921),
            Job::new(961, 73, 1628),
            Job::new(1256, 69, 288),
            Job::new(1179, 52, 250),
            Job::new(201, 71, 1420),
            Job::new(232, 19, 606),
            Job::new(40, 87, 221),
            Job::new(1088, 18, 155),
            Job::new(1061, 78, 1786),
            Job::new(455, 63, 801),
            Job::new(1466, 77, 746),
            Job::new(73, 49, 1039),
            Job::new(1723, 53, 1396),
            Job::new(261, 34, 1366),
            Job::new(741, 40, 1381),
            Job::new(789, 47, 1242),
            Job::new(1806, 25, 1196),
            Job::new(269, 41, 136),
            Job::new(316, 35, 1019),
            Job::new(870, 51, 251),
            Job::new(854, 67, 1693),
            Job::new(824, 23, 499),
            Job::new(1305, 47, 1746),
        ]);
        let mut result: u32 = u32::MAX;
        carlier(&mut js, &mut result);
        assert_eq!(result, 3191)
    }
}
