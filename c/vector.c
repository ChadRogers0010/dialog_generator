#include "./vector.h"
#include <stdlib.h>
#include <string.h>

Vector *vector_init(Vector *vector, size_t cap) {
  vector->ptr = malloc(cap * sizeof(size_t *));
  vector->cap = cap;
  vector->len = 0;

  return vector;
}

void vector_increase(Vector *vector) {
  const size_t inc_size = 2;

  size_t new_size = vector->cap * inc_size * sizeof(size_t);
  size_t *new_buffer = malloc(new_size);
  memcpy(new_buffer, vector->ptr, vector->cap * sizeof(size_t *));
  free(vector->ptr);

  vector->cap *= inc_size;
  vector->ptr = new_buffer;
}

void vector_push(Vector *vector, void *value) {
  if (vector->cap == vector->len) {
    vector_increase(vector);
  }
  ((void **)vector->ptr)[vector->len] = value;
  vector->len += 1;
}

void *vector_pop(Vector *vector) {
  if (vector->len == 0) {
    return 0;
  }
  void *val = ((void **)vector->ptr)[vector->len - 1];
  vector->len -= 1;
  return val;
}

size_t vector_clear(Vector *vector) {
  size_t len = vector->len;
  vector->len = 0;
  return len;
}

void vector_delete(Vector *vector) {
  free(vector->ptr);
  vector->ptr = NULL;
  vector->cap = 0;
  vector->len = 0;
}

Vector *vector_heap(size_t cap) {
  Vector *heap_vec = (Vector *)malloc(sizeof(Vector));
  vector_init(heap_vec, cap);
  return heap_vec;
}

void vector_heap_delete(Vector *vector) {
  vector_delete(vector);
  free(vector);
}
