use std::fmt::Debug;

use crate::{matrix::Matrix, solution::Solution};

#[derive(Clone, Default)]
pub struct Operation {
    pub id: usize,
    pub job: usize,
    pub machine: usize,
    pub time: usize,
    pub seq: usize,
}

impl Debug for Operation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "(J{}[{}], M{}, {})",
            &self.job + 1,
            &self.seq + 1,
            &self.machine + 1,
            &self.time
        )
    }
}

pub type Operations = Matrix<Schedule>;

impl Operation {
    pub fn new(id: usize, job: usize, machine: usize, time: usize, seq: usize) -> Operation {
        Operation {
            id,
            job,
            machine,
            time,
            seq,
        }
    }
}

#[derive(Debug, Default, Clone)]
pub struct Schedule {
    // I only need a reference to an operation
    pub operation: Box<Operation>,
    pub seq_m: usize,
    // Release date.
    pub r: Option<usize>,
    // Length tail
    pub q: Option<usize>,
}

pub type Schedules = Vec<Schedule>;

impl Schedule {
    pub fn new(
        operation: &Operation,
        seq_m: usize,
        r: Option<usize>,
        q: Option<usize>,
    ) -> Schedule {
        Schedule {
            operation: Box::new(operation.clone()),
            seq_m,
            r,
            q,
        }
    }

    pub fn get_predecesor_machine(&self, solution: &Solution) -> Option<Schedule> {
        if self.seq_m > 0 {
            solution
                .operations
                .at(self.operation.machine, self.seq_m - 1)
                .cloned()
        } else {
            None
        }
    }

    pub fn get_successor_machine(&self, solution: &Solution) -> Option<Schedule> {
        todo!()
    }

    pub fn get_predecesor_job(&self, solution: &Solution) -> Option<Schedule> {
        todo!()
    }

    pub fn get_successor_job(&self, solution: &Solution) -> Option<Schedule> {
        todo!()
    }
}
