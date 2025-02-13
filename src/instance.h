#ifndef JOB_SHOP_INSTANCE_H
#define JOB_SHOP_INSTANCE_H

#include <cstdint>
#include <string>
#include <vector>

class Operation {
public:
  int job;
  int machine;
  int time;
  int seq;

  void display();
};

class Matrix {
public:
  std::vector<std::vector<Operation>> mat;

  Matrix(uint8_t rows, uint8_t cols);

  Operation &at(uint8_t i, uint8_t j);

  void display();
};

class Instance {
public:
  int num_of_jobs;
  int num_of_machines;
  Matrix jobs;

  Instance(char *filename);

  void display();
};
#endif // !JOB_SHOP_INSTANCE_H
