#ifndef PERMUTATION_MACHINES_SOLUTION_H
#define PERMUTATION_MACHINES_SOLUTION_H

#include "instance.cpp"

class Solution {
public:
  int num_of_machines;
  int num_of_jobs;

  Matrix machines;

  int makespan;

  Solution(Instance Instance);

  void display();
};

#endif // !PERMUTATION_MACHINES_
