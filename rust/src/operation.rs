use std::fmt::Debug;

use crate::matrix::Matrix;

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
}
