#include "instance.hpp"
#include <cstdint>
#include <cstdio>
#include <cstdlib>
#include <iostream>
#include <vector>

Matrix::Matrix(uint8_t r, uint8_t c) {
  mat = std::vector<std::vector<Operation>>(r, std::vector<Operation>(c));
};

Operation &Matrix::at(uint8_t i, uint8_t j) { return mat.at(i).at(j); }

void Matrix::display() {
  int i = 1;
  for (std::vector<Operation> row : mat) {
    std::cout << "M" << i << ": ";
    for (Operation e : row) {
      e.display();
    }
    i++;
    std::cout << "\n\n";
  }
}

Instance::Instance(char *filename) : jobs(0, 0) {
  FILE *in = fopen(filename, "r");

  fscanf(in, "%d", &num_of_jobs);
  fscanf(in, "%d", &num_of_machines);

  jobs = Matrix(num_of_jobs, num_of_machines);

  for (int i = 0; i < num_of_jobs; i++) {
    for (int j = 0; j < num_of_machines; j++) {
      jobs.at(i, j).job = i;
      fscanf(in, "%d", &jobs.at(i, j).machine);
      jobs.at(i, j).seq = j;
      fscanf(in, "%d", &jobs.at(i, j).time);
    }
  }
};

void Instance::display() {
  std::cout << "Instance {\n";
  std::cout << "\tNumber of jobs: " << num_of_jobs << "\n";
  std::cout << "\tNumber of machines: " << num_of_machines << "\n";
  std::cout << "\tJobs: \n";
  jobs.display();
  std::cout << "}\n";
}

void Operation::display() {
  printf(" (J%d[%d], M%d, %d)", job + 1, seq + 1, machine + 1, time);
};
