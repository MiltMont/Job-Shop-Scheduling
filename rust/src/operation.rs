use std::fmt::Debug;

use crate::{matrix::Matrix, solution::Solution};

#[derive(Clone, Default)]
pub struct Operation {
    pub id: usize,
    // J_i: Job to which this operation belongs.
    pub job: usize,
    // M_i: Machin which processes i.
    pub machine: usize,
    // d_i: Processing time of i.
    pub time: usize,
    // Index in the sequence of operations in J_i
    pub seq: usize,

    // This will be initially zero.
    pub seq_m: usize,
    pub r: usize,
    pub q: usize,
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

pub type Operations = Matrix<Operation>;

impl Operation {
    pub fn new(id: usize, job: usize, machine: usize, time: usize, seq: usize) -> Operation {
        Operation {
            id,
            job,
            machine,
            time,
            seq,
            seq_m: 0,
            r: 0,
            q: 0,
        }
    }
    // PM_i in Taillard's notation.
    pub fn get_predecesor_machine(&self, solution: &Solution) -> Option<Operation> {
        if self.seq_m > 0 {
            solution
                .operations
                .at(self.machine, self.seq_m - 1)
                .cloned()
        } else {
            None
        }
    }

    // SM_i in Taillard's notation.
    pub fn get_successor_machine(&self, solution: &Solution) -> Option<Operation> {
        if self.seq_m < solution.num_of_jobs - 1 {
            solution
                .operations
                .at(self.machine, self.seq_m + 1)
                .cloned()
        } else {
            None
        }
    }

    // PJ_i in Taillard's notation.
    pub fn get_predecesor_job(&self, solution: &Solution) -> Option<Operation> {
        if self.seq > 0 {
            solution.scheduled_operations.get(self.id - 1).cloned()
        } else {
            None
        }
    }

    // SJ_i in Taillard's notation.
    pub fn get_successor_job(&self, solution: &Solution) -> Option<Operation> {
        if self.seq < solution.num_of_machines - 1 {
            solution.scheduled_operations.get(self.id + 1).cloned()
        } else {
            None
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
}
