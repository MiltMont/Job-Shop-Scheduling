#include "instance.cpp"
#include <cstdio>
#include <cstdlib>

int main(int argc, char *argv[]) {
  char *filename = argv[1];
  int tipo = argc > 2 ? atoi(argv[2]) : 1;

  Instance instance(filename);
  instance.display();
}
