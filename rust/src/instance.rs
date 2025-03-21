use std::{fmt::Debug, fs::File, io::Read, path::PathBuf};

use text_io::read;

use crate::{matrix::Matrix, operation::Operation};

pub struct Instance {
    pub num_of_jobs: usize,
    pub num_of_machines: usize,
    pub jobs: Matrix<Operation>,
}

impl Debug for Instance {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Jobs: {}, Machines: {}\n{:?}",
            &self.num_of_jobs, &self.num_of_machines, &self.jobs
        )
    }
}

impl From<PathBuf> for Instance {
    fn from(value: PathBuf) -> Self {
        let mut file = File::open(value).unwrap().bytes().map(|c| c.unwrap());

        let num_of_jobs: usize = read!("{}", file);
        let num_of_machines: usize = read!("{}", file);

        let mut jobs = Matrix::new(num_of_jobs, num_of_machines);

        let mut k = 0;
        for i in 0..num_of_jobs {
            for j in 0..num_of_machines {
                k += 1;
                let machine: usize = read!("{}", file);
                let time: usize = read!("{}", file);
                jobs.set_at(Operation::new(k - 1, i, machine, time, j), i, j);
            }
        }

        Instance {
            num_of_jobs,
            num_of_machines,
            jobs,
        }
    }
}
