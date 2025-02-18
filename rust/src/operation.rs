use std::fmt::Debug;

use crate::matrix::Matrix;

#[derive(Clone, Default)]
pub struct Operation {
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

pub type Operations = Matrix<Operation>;

impl Operation {
    pub fn new(job: usize, machine: usize, time: usize, seq: usize) -> Operation {
        Operation {
            job,
            machine,
            time,
            seq,
        }
    }
}
