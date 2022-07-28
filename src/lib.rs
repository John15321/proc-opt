//! This crate provides job/process scheduling optimization algorithms.
//!
//! # A job or a process
//!
//! A job or a process is described using three values:
//!
//! * `r` - ready time
//! * `p` - process time
//! * `q` - cooldown/quit time
//!
#![doc=include_str!("../img/job.svg")]
//!
//! `r` - Time it takes for this process to become available form time `0`. For example after turning on a machine it may take some time for some of its features to become available due to a certain constraints. A trivial example is that of cooking, a grill or any other device that allows thermal treatment for given food needs to achieve a certain temperature otherwise it will not be effective.
//!
//! `p` - Time taken by the process itself. It is the time taken by the modeled mechanism itself to perform a given function. Following the cooking example it can be modeled as the actual time that a given food has to be cooked for on the grill.
//!
//! `q` - Time that it takes for the machine to clean-up/cooldown/quit after executing a given process. Again, following the cooking example it can be described as the time needed to clean-up after serving the cooked food.
//!
//! `C` - The final time when the machine finishes dealing with any part of the process/job.
//!
//!
//! # Scheduling problems
//!
//! There exist two types of job scheduling problems. The first one being *single-machine* problem and the second being *multi-machine* problem.
//! Each of them agrees with these assumptions:
//! 1. First being that jobs ona a single machine cannot have their processing time shared simultaneously between processes.
//! 2. The readying time and quitting time can overlap.
//! 3. The whole of the machines process end when the last job finishes.
//!
//!
//! An example set of jobs and their natural (from the 1 to $j$'th) arrangement:
//!
//! <div align="center">
//!
//! |$j$  |1    |2    |3    |4    |5    |6    |7    |
//! |:---:|:---:|:---:|:---:|:---:|:---:|:---:|:---:|
//! |$r_j$|10   |13   |11   |20   |30   |0    |30   |
//! |$p_j$|5    |6    |7    |4    |3    |6    |2    |
//! |$q_j$|7    |26   |24   |21   |8    |17   |0    |
//!
//! </div>
//!
#![doc=include_str!("../img/schrage_natural.svg")]
//!
//! $C_{max}$ - being the maximum time out of all of the finishing times for all the jobs. It is therefore also the time at which the whole machine finishes work.
//!
//! Here we can see the actual optimal arrangement for the same jobs, where the $C_{max}$ is minimized down to $50$:
//!
#![doc=include_str!("../img/optimal.svg")]
//!
//! # Single-machine problem
//!
//! Meta-heuristic algorithms (giving fast but approximate answer):
//! * Schrage algorithm - [`schrage`]
//! * Part Time Schrage (TBD)
//!
//! Heuristic algorithms (giving slower but optimal answer):
//! * Carlier (TBD)
//!
//! # Multi-machine problem
//! TBD
//!

// cargo doc dark theme background color: #353535
#![forbid(unsafe_code)]

pub mod jobs;
pub mod schrage;
