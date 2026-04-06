#include "uv.h"

TEST_DECLARE(safe_regression_sanity)
TEST_DECLARE(getaddrinfo_long_hostname)
TEST_DECLARE(io_uring_privdrop_regression)
TEST_DECLARE(supplementary_groups_dropped)

TASK_LIST_START
  TEST_ENTRY(safe_regression_sanity)
  TEST_ENTRY(getaddrinfo_long_hostname)
  TEST_ENTRY(io_uring_privdrop_regression)
  TEST_ENTRY(supplementary_groups_dropped)
TASK_LIST_END
