use crate::{
    instance::Instance,
    operation::{Operation, Operations},
};

use rand::prelude::*;

#[derive(Debug)]
pub struct Solution {
    pub machines: Operations,
    pub makespan: usize,
}

impl From<Instance> for Solution {
    fn from(instance: Instance) -> Self {
        let mut machines = Operations::new(instance.num_of_machines, instance.num_of_jobs);
        let makespan = usize::MAX;

        let mut machines_free_positions = vec![0; instance.num_of_machines];
        let mut availables: Vec<Operation> = vec![Operation::default(); instance.num_of_jobs];

        // Availables is the first row of the operations matrix.
        instance.jobs.mat.iter().enumerate().for_each(|(i, j)| {
            availables[i] = j[0].clone();
        });

        let mut rng = rand::rng();
        let mut total_availables = instance.num_of_jobs;

        while total_availables > 0 {
            // Pick a random operation
            let tmp: Vec<usize> = (0..total_availables).collect();
            let tmp_random = tmp.choose(&mut rng).unwrap();
            let random_op = availables.get(tmp_random.to_owned()).unwrap();

            // Plan this operation
            let current_free = machines_free_positions[random_op.machine];
            machines.set_at(random_op.to_owned(), random_op.machine, current_free);
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

        Solution { machines, makespan }
    }
}
