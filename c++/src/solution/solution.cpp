#include "solution.hpp"
#include <climits>
#include <cstdio>
#include <cstdlib>
#include <iostream>
#include <vector>

Solution::Solution(Instance instance) : machines(0, 0) {
  num_of_jobs = instance.num_of_jobs;
  num_of_machines = instance.num_of_machines;

  machines = Matrix(num_of_machines, num_of_jobs);
  makespan = INT_MAX;

  std::vector<int> machines_free_positions(num_of_machines, 0);
  std::vector<Operation> availables(num_of_jobs);

  for (int i = 0; i < num_of_jobs; i++) {
    availables.at(i) = instance.jobs.at(i, 0);
  }

  int total_availables = num_of_jobs;

  int tmp_random;
  int current_free_position;
  Operation random_op;

  while (total_availables > 0) {
    // 1. Pick a random operation from availables.
    tmp_random = rand() % total_availables;
    random_op = availables.at(tmp_random);

    // 2. Plan this operation
    current_free_position = machines_free_positions.at(random_op.machine);
    machines.at(random_op.machine, current_free_position) = random_op;
    machines_free_positions.at(random_op.machine)++;

    // 3. If there is a following operation, replace it for the actual operation
    if (random_op.seq < num_of_machines - 1) {
      availables.at(tmp_random) =
          instance.jobs.at(random_op.job, random_op.seq + 1);
    } else {
      availables.at(tmp_random) = availables[total_availables - 1];
      total_availables--;
    }
  }
}

void Solution::display() {
  std::cout << "Solution: \n";
  std::cout << "Num of machines: " << num_of_machines << std::endl;
  std::cout << "Num of jobs: " << num_of_jobs << std::endl;
  std::cout << "Machines: \n";

  for (int i = 0; i < num_of_machines; i++) {
    std::cout << "M" << i + 1 << " = [";
    for (int j = 0; j < num_of_jobs; j++) {
      // Displaying only the job.
      std::cout << machines.at(i, j).job << ",";
    }
    std::cout << "]\n\n";
  }
}
