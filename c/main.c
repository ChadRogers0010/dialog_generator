#include "./vector.h"
#include <stdio.h>
#include <time.h>
void query(int predicates[], Vector *response);

int main(int argc, char *argv[]) {
  Vector response;
  vector_init(&response, 32);

  int predicates[100] = {0};
  for (int i = 0; i < 100; ++i) {
    predicates[i] = 50;
  }
  clock_t start = clock();
  query(predicates, &response);
  clock_t end = clock();
  double elapsed_secs = (double)(end - start) / CLOCKS_PER_SEC * 1000.0;

  printf("response len: %ld\n", response.len);
  printf("Elapsed time: %.3f\n", elapsed_secs);

  return 0;
}
