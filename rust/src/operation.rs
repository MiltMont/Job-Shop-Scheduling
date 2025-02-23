use std::fmt::Debug;

use crate::{matrix::Matrix, solution::Solution};

#[derive(Clone, Default, PartialEq)]
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

    //
    // These will be initially zero.
    //
    pub seq_m: usize,
    // Release time: Earliest beginning time of the operation.
    pub r: usize,
    // Length of the tail: Longest path from the operation to the end.
    pub q: usize,

    // Location in solution matrix.
    pub location: (usize, usize),
}

impl Debug for Operation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "(J{}[{}], m={}, r={}, q={}, d={})",
            &self.job + 1,
            &self.seq + 1,
            &self.machine,
            &self.r,
            &self.q,
            &self.time,
        )
    }
}

pub type Operations = Matrix<Operation>;

impl Operation {
    pub fn equal_up_to_position(&self, o2: &Operation) -> bool {
        self.machine == o2.machine
            && self.id == o2.id
            && self.job == o2.job
            && self.time == o2.time
            && self.seq == o2.seq
    }
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
            location: (0, 0),
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

    // Returns the location of the predecesor operation in the current
    // machine.
    // PM_i
    pub fn get_predecesor_op_in_machine(&self) -> Option<(usize, usize)> {
        if self.seq_m > 0 {
            Some((self.machine, self.seq_m - 1))
        } else {
            None
        }
    }

    // Returns the location of the successor operation in the current
    // machine.
    // SM_i
    pub fn get_successor_op_in_machine(&self, solution: &Solution) -> Option<(usize, usize)> {
        if self.seq_m < solution.num_of_jobs - 1 {
            Some((self.machine, self.seq_m + 1))
        } else {
            None
        }
    }

    // Return the location of the successor operation in the current job
    // given a solution.
    pub fn get_successor_op_in_job(&self, solution: &Solution) -> Option<(usize, usize)> {
        if self.seq < solution.num_of_machines - 1 {
            Some(
                solution
                    .scheduled_operations
                    .get(self.id + 1)
                    .unwrap()
                    .location,
            )
        } else {
            None
        }
    }

    // Return the location of the predecesor operation in  the current job
    // given a solution.
    pub fn get_predecesor_op_in_job(&self, solution: &Solution) -> Option<(usize, usize)> {
        if self.seq > 0 {
            Some(
                solution
                    .scheduled_operations
                    .get(self.id - 1)
                    .unwrap()
                    .location,
            )
        } else {
            None
        }
    }

    // Return the location
}
