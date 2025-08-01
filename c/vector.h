#include <stddef.h>

// Growable list of size_t
typedef struct {
  void *ptr;
  size_t cap;
  size_t len;
} Vector;

// create buffer and set capacity
Vector *vector_init(Vector *vector, size_t cap);
//
// double the size of the vector buffer
void vector_increase(Vector *vector);

// append a value to the buffer
void vector_push(Vector *vector, void *value);

// return the last value in the list and decrement length
// if length is 0 return 0
void *vector_pop(Vector *vector);

// set capacity to 0
size_t vector_clear(Vector *vector);

// free the buffer. Set capacity  and length to 0
void vector_delete(Vector *vector);

// return a heap allocated vector struct
Vector *vector_heap(size_t cap);

// Free the heap buffer, then free the vector
void vector_heap_delete(Vector *vector);
