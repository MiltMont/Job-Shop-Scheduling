use std::fmt::Debug;
use std::{cmp, collections::HashSet};

use crate::{
    instance::Instance,
    operation::{Operation, Operations},
};

use rand::prelude::*;

/// Represents a feasible solution.
pub struct Solution {
    pub operations: Operations,
    // TODO: Do we need this field?
    pub scheduled_operations: Vec<Operation>,
    pub makespan: usize,
    pub num_of_jobs: usize,
    pub num_of_machines: usize,
}

impl Debug for Solution {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Generated solution: ")?;
        for (i, row) in self.operations.mat.iter().enumerate() {
            write!(f, "M{} [", i + 1)?;
            for s in row.iter() {
                write!(f, "{:?}, ", s)?;
            }
            writeln!(f, "]")?;
        }
        writeln!(f, "makespan: {}", self.makespan)?;
        writeln!(f, "{:?}", self.scheduled_operations)?;
        Ok(())
    }
}

/// Obtains a feasible solution from an instance.
impl From<&Instance> for Solution {
    fn from(instance: &Instance) -> Self {
        let mut operations = Operations::new(instance.num_of_machines, instance.num_of_jobs);
        let mut scheduled_operations =
            vec![Operation::default(); instance.num_of_jobs * instance.num_of_machines];
        let makespan = 0;

        let mut machines_free_positions = vec![0; instance.num_of_machines];
        let mut availables: Vec<Operation> = Vec::new();

        // Availables is the first row of the operations matrix.
        instance
            .jobs
            .mat
            .iter()
            .for_each(|job| availables.push(job[0].clone()));

        let mut rng = rand::rng();
        let mut total_availables = instance.num_of_jobs;

        while total_availables > 0 {
            // Pick a random operation
            let tmp: Vec<usize> = (0..total_availables).collect();
            let tmp_random = tmp.choose(&mut rng).unwrap();
            let mut random_op = availables.get(tmp_random.to_owned()).unwrap().to_owned();

            // Plan this operation
            let current_free = machines_free_positions[random_op.machine];
            random_op.seq_m = current_free;

            // Set location in solution
            random_op.location = (random_op.machine, current_free);
            operations.set_at(
                // Schedule::new(random_op, current_free, None, None),
                random_op.clone(),
                random_op.machine,
                current_free,
            );
            scheduled_operations[random_op.id] = operations
                .at(random_op.machine, current_free)
                .unwrap()
                .clone();

            machines_free_positions[random_op.machine] += 1;

            // If there is a following operation, replace it for the actual operation.
            if random_op.seq < instance.num_of_machines - 1 {
                availables[tmp_random.to_owned()] = instance
                    .jobs
                    .at(random_op.job, random_op.seq + 1)
                    .unwrap()
                    .clone();
            } else {
                availables[tmp_random.to_owned()] = availables[total_availables - 1].clone();
                total_availables -= 1;
            }
        }

        let mut solution = Solution {
            operations,
            makespan,
            scheduled_operations,
            num_of_machines: instance.num_of_machines,
            num_of_jobs: instance.num_of_jobs,
        };

        solution
            .compute_release_dates(instance)
            .compute_length_tails(instance);
        solution
    }
}

impl Solution {
    pub fn compute_release_dates(&mut self, instance: &Instance) -> &mut Self {
        let mut availables = Vec::new();
        let mut visited = HashSet::new();

        // We find the index of the initial operations for each job.
        for i in 0..self.num_of_jobs {
            let op_from_current_job = instance.jobs.at(i, 0).unwrap();

            if op_from_current_job
                .equal_up_to_position(self.operations.at(op_from_current_job.machine, 0).unwrap())
            {
                availables.push((op_from_current_job.machine, 0));
            }
        }

        while let Some((i, j)) = availables.pop() {
            visited.insert((i, j));
            // Compute r for operation at (i,j)
            let pred_mach = self
                .operations
                .at(i, j)
                .unwrap()
                .get_predecesor_machine(self);

            let (r_from_pred_m, t_from_pred_m) = if let Some(op) = pred_mach {
                (op.r, op.time)
            } else {
                (0, 0)
            };

            let pred_job = self.operations.at(i, j).unwrap().get_predecesor_job(self);
            let (r_from_pred_job, t_from_pred_job) = if let Some(op) = pred_job {
                (op.r, op.time)
            } else {
                (0, 0)
            };

            // Update current operation r value
            self.operations.mat[i][j].r = cmp::max(
                r_from_pred_m + t_from_pred_m,
                r_from_pred_job + t_from_pred_job,
            );
            // TODO: Do we need this array?
            self.scheduled_operations[self.operations.at(i, j).unwrap().id].r =
                self.operations.mat[i][j].r;

            // Update makespan
            self.makespan = cmp::max(
                self.makespan,
                self.operations.mat[i][j].r + self.operations.mat[i][j].time,
            );

            let successor_job = self.operations.at(i, j).unwrap().get_successor_job(self);
            if let Some(op) = successor_job {
                if op.get_predecesor_machine(self).is_none()
                    || op
                        .get_predecesor_machine(self)
                        .is_some_and(|o| visited.contains(&o.location))
                {
                    availables.push(op.location);
                }
            }

            let successor_machine = self
                .operations
                .at(i, j)
                .unwrap()
                .get_successor_machine(self);

            if let Some(op) = successor_machine {
                if op.get_predecesor_job(self).is_none()
                    || op
                        .get_predecesor_job(self)
                        .is_some_and(|o| visited.contains(&o.location))
                {
                    availables.push(op.location);
                }
            }
        }
        self
    }

    pub fn compute_length_tails(&mut self, instance: &Instance) -> &mut Self {
        let mut availables: Vec<(usize, usize)> = Vec::new();
        let mut visited: HashSet<(usize, usize)> = HashSet::new();

        for i in 0..self.num_of_jobs {
            let op_from_current_job = instance.jobs.mat[i].last().unwrap();

            if op_from_current_job.equal_up_to_position(
                self.operations.mat[op_from_current_job.machine]
                    .last()
                    .unwrap(),
            ) {
                availables.push((op_from_current_job.machine, self.num_of_jobs - 1));
            }
        }

        while let Some((i, j)) = availables.pop() {
            visited.insert((i, j));
            //let successor_machine = self
            //    .operations
            //    .at(i, j)
            //    .unwrap()
            //    .get_successor_machine(&self);
            //let successor_job = self.operations.at(i, j).unwrap().get_successor_job(&self);

            let q_successor_machine = if let Some(op) = self
                .operations
                .at(i, j)
                .unwrap()
                .get_successor_machine(self)
            {
                op.q
            } else {
                0
            };

            let q_successor_job =
                if let Some(op) = self.operations.at(i, j).unwrap().get_successor_job(self) {
                    op.q
                } else {
                    0
                };

            self.operations.mat[i][j].q =
                cmp::max(q_successor_machine, q_successor_job) + self.operations.mat[i][j].time;
            self.scheduled_operations[self.operations.at(i, j).unwrap().id].q =
                self.operations.mat[i][j].q;

            if let Some(op) = self.operations.at(i, j).unwrap().get_predecesor_job(self) {
                if op.get_successor_machine(self).is_none()
                    || op
                        .get_successor_machine(self)
                        .is_some_and(|op| visited.contains(&op.location))
                {
                    availables.push(op.location);
                }
            }

            if let Some(op) = self
                .operations
                .at(i, j)
                .unwrap()
                .get_predecesor_machine(self)
            {
                if op.get_successor_job(self).is_none()
                    || op
                        .get_successor_job(self)
                        .is_some_and(|op| visited.contains(&op.location))
                {
                    availables.push(op.location);
                }
            }
        }
        self
    }
}
