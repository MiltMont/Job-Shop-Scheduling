use std::io::Read;

use text_io::read;

use crate::operation::{Operation, Operations};

#[derive(Debug)]
pub struct Instance {
    pub num_of_jobs: usize,
    pub num_of_machines: usize,
    pub jobs: Operations,
}

impl From<&str> for Instance {
    fn from(filename: &str) -> Self {
        let mut file = std::fs::File::open(filename)
            .unwrap()
            .bytes()
            .map(|c| c.unwrap());

        let num_of_jobs: usize = read!("{}", file);
        let num_of_machines: usize = read!("{}", file);

        let mut jobs: Operations = Operations::new(num_of_jobs, num_of_machines);

        for i in 0..num_of_jobs {
            for j in 0..num_of_machines {
                let machine: usize = read!("{}", file);
                let time: usize = read!("{}", file);
                jobs.set_at(Operation::new(i, machine, time, j), i, j);
            }
        }

        Instance {
            num_of_jobs,
            num_of_machines,
            jobs,
        }
    }
}
