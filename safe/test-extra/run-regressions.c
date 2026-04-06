#include <stdio.h>
#include <string.h>

#include "runner.h"
#include "task.h"
#include "uv.h"

#include "regression-list.h"

static int maybe_run_test(char** argv);


int run_test_safe_regression_sanity(void) {
  ASSERT_UINT64_EQ(uv_version(), UV_VERSION_HEX);
  return 0;
}


int main(int argc, char** argv) {
  platform_init(argc, argv);
  argv = uv_setup_args(argc, argv);

  switch (argc) {
  case 1:
    return run_tests(0);
  case 2:
    return maybe_run_test(argv);
  case 3:
    return run_test_part(argv[1], argv[2]);
  default:
    fprintf(stderr, "Too many arguments.\n");
    fflush(stderr);
    return 1;
  }
}


static int maybe_run_test(char** argv) {
  if (strcmp(argv[1], "--list") == 0) {
    print_tests(stdout);
    return 0;
  }

  return run_test(argv[1], 0, 1);
}
