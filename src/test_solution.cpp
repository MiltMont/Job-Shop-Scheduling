#include "solution.cpp"
#include <cstdlib>

int main(int argc, char *argv[]) {
  char *filename = argv[1];
  int tipo = argc > 2 ? atoi(argv[2]) : 1;

  unsigned int seed = 1;
  srand(seed);

  Instance instance(filename);
  Solution solution(instance);

  solution.display();
}
