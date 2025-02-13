#include "solution.hpp"

#include <cstdlib>
#include <time.h>

int main(int argc, char *argv[]) {
  char *filename = argv[1];

  unsigned int seed = (unsigned int)time(nullptr);
  srand(seed);

  Instance instance(filename);
  Solution solution(instance);

  solution.display();
}
