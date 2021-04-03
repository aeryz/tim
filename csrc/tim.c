#include "tim.h"

tim_res_t _tim_error(const char *file, size_t line, const char *msg) {
  tim_res_t res;
  res.file = file;

  res.line = line;
  if (NULL != msg) {
    res.msg = (char *)malloc(strlen(msg) * sizeof(char) + sizeof(char));
    strcpy(res.msg, msg);
  } else {
    res.msg = NULL;
  }
  res.success = 0;
  return res;
}

tim_res_t _tim_success(const char *msg) {
  tim_res_t res;
  res.file = NULL;
  if (NULL != msg) {
    res.msg = (char *)malloc(strlen(msg) * sizeof(char) + sizeof(char));
    strcpy(res.msg, msg);
  } else {
    res.msg = NULL;
  }
  res.line = 0;
  res.success = 1;
  return res;
}

void tim_free(unsigned int n_args, char **ptrs) {
  for (int i = 0; i < n_args; ++i) {
    char *ptr = ptrs[i];
    if (NULL != ptr) {
      free(ptr);
    }
  }
}
