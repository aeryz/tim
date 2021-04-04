#ifndef TIM_TEST_H_
#define TIM_TEST_H_

#include <stdarg.h>
#include <stdlib.h>
#include <string.h>

#define TIM_TEST(X) tim_res_t X()

typedef struct {
  /** Contains the name of the file which the error is occured */
  const char *file;
  /** Contains the allocated message if any, otherwise it is set to NULL */
  char *msg;
  /** The line number which the error is occured */
  size_t line;
  /** Indicator of if the test is successfull or not */
  unsigned success;
} tim_res_t;

/** Returns 'success' */
#define TIM_TEST_SUCCESS() TIM_TEST_SUCCESS_MSG(NULL);

/** Returns 'error' */
#define TIM_TEST_ERROR() TIM_TEST_ERROR_MSG(NULL);

/** Asserts if an expression is non-zero. */
#define TIM_ASSERT(X) TIM_ASSERT_MSG(X, NULL);

/** Asserts if two expressions are equal. */
#define TIM_ASSERT_EQ(LHS, RHS) TIM_ASSERT_EQ_MSG(LHS, RHS, NULL);

/** Asserts if two expressions are equal by using an equality function. */
#define TIM_ASSERT_EQ_FN(LHS, RHS, EQ_FN)                                      \
  TIM_ASSERT_EQ_FN_MSG(LHS, RHS, EQ_FN, NULL);

/** Asserts if two strings are equal by using 'strcmp'. */
#define TIM_ASSERT_EQ_STR(LHS, RHS) TIM_ASSERT_EQ_STR_MSG(LHS, RHS, NULL);

/** Asserts if two arrays are equal. */
#define TIM_ASSERT_EQ_ARR(LHS, RHS, LEN)                                       \
  TIM_ASSERT_EQ_ARR_MSG(LHS, RHS, LEN, NULL);

/** Asserts if two arrays are equal by using an equality function to compare
 * each element. */
#define TIM_ASSERT_EQ_ARR_FN(LHS, RHS, LEN, EQ_FN)                             \
  TIM_ASSERT_EQ_ARR_FN_MSG(LHS, RHS, LEN, EQ_FN, NULL);

#define TIM_ASSERT_MSG(X, MSG)                                                 \
  if (!(X)) {                                                                  \
    return _tim_error(__FILE__, __LINE__, MSG);                                \
  }

#define TIM_ASSERT_EQ_MSG(LHS, RHS, MSG)                                       \
  if (LHS != RHS) {                                                            \
    return _tim_error(__FILE__, __LINE__, MSG);                                \
  }

#define TIM_ASSERT_EQ_STR_MSG(LHS, RHS, MSG)                                   \
  if (0 != strcmp(LHS, RHS)) {                                                 \
    return _tim_error(__FILE__, __LINE__, MSG);                                \
  }

#define TIM_ASSERT_EQ_ARR_MSG(LHS, RHS, LEN, MSG)                              \
  for (int i = 0; i < LEN; ++i) {                                              \
    if (LHS[i] != RHS[i])                                                      \
      return _tim_error(__FILE__, __LINE__, MSG);                              \
  }

#define TIM_ASSERT_EQ_FN_MSG(LHS, RHS, EQ_FN, MSG)                             \
  if (0 == EQ_FN(LHS, RHS)) {                                                  \
    return _tim_error(__FILE__, __LINE__, NULL);                               \
  }

#define TIM_ASSERT_EQ_ARR_FN_MSG(LHS, RHS, LEN, EQ_FN, MSG)                    \
  for (int i = 0; i < LEN; ++i) {                                              \
    if (0 == EQ_FN(LHS[i], RHS[i])) {                                          \
      return _tim_error(__FILE__, __LINE__, MSG);                              \
    }                                                                          \
  }

#define TIM_TEST_SUCCESS_MSG(MSG) return _tim_success(MSG);

#define TIM_TEST_ERROR_MSG(MSG) return _tim_error(__FILE__, __LINE__, MSG);

tim_res_t _tim_error(const char *file, size_t line, const char *msg);

tim_res_t _tim_success(const char *msg);

void tim_free(char *ptr);

#endif
