use std::{cmp, fmt::Debug};

use crate::{operation::Operation, solution::Solution};

pub struct Neighbor {
    pub a: Operation,
    pub b: Operation,
    pub eval: usize,
}

impl Debug for Neighbor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "N[ (J{:?},m={}), (J{:?},m={}), {:} ]",
            self.a.job, self.a.machine, self.b.job, self.b.machine, self.eval
        )
        // f.debug_struct("Neighbor")
        //     .field("a", &self.a)
        //     .field("b", &self.b)
        //     .field("eval", &self.eval)
        //     .finish()
    }
}

impl Neighbor {
    pub fn new(a: &Operation, b: &Operation, solution: &Solution) -> Neighbor {
        // Computing eval from a and b
        let eval = Neighbor::estimate_evaluation(a, b, solution);
        Neighbor {
            a: a.clone(),
            b: b.clone(),
            eval,
        }
    }

    pub fn estimate_evaluation(a: &Operation, b: &Operation, solution: &Solution) -> usize {
        let r_b = cmp::max(
            if let Some(op) = a.get_predecesor_machine(solution) {
                op.r + op.time
            } else {
                0
            },
            if let Some(op) = b.get_predecesor_job(solution) {
                op.r + op.time
            } else {
                0
            },
        );

        let r_a = cmp::max(
            r_b + b.time,
            if let Some(op) = a.get_predecesor_job(solution) {
                op.r + op.time
            } else {
                0
            },
        );

        let q_a = cmp::max(
            if let Some(op) = b.get_successor_machine(solution) {
                op.q
            } else {
                0
            },
            if let Some(op) = a.get_successor_job(solution) {
                op.q
            } else {
                0
            },
        ) + a.time;

        let q_b = cmp::max(
            q_a,
            if let Some(op) = b.get_successor_job(solution) {
                op.q
            } else {
                0
            },
        ) + b.time;

        cmp::max(r_b + q_b, r_a + q_a)
    }
}

pub type Neighborhood = Vec<Neighbor>;

fn are_critical(solution: &Solution, op1: &Operation, op2: &Operation) -> bool {
    op1.r + op1.q == solution.makespan && op2.r + op2.q == solution.makespan
}

impl From<&Solution> for Neighborhood {
    fn from(solution: &Solution) -> Self {
        let mut res = Vec::new();
        solution.operations.mat.iter().for_each(|job| {
            job.windows(2).for_each(|v| {
                if are_critical(solution, &v[0], &v[1]) {
                    res.push(Neighbor::new(&v[0], &v[1], solution))
                }
            })
        });
        res
    }
}
