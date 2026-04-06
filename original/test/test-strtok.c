/* Copyright libuv project contributors. All rights reserved.
*
* Permission is hereby granted, free of charge, to any person obtaining a copy
* of this software and associated documentation files (the "Software"), to
* deal in the Software without restriction, including without limitation the
* rights to use, copy, modify, merge, publish, distribute, sublicense, and/or
* sell copies of the Software, and to permit persons to whom the Software is
* furnished to do so, subject to the following conditions:
*
* The above copyright notice and this permission notice shall be included in
* all copies or substantial portions of the Software.
*
* THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
* IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
* FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
* AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
* LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING
* FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS
* IN THE SOFTWARE.
*/

#include "uv.h"
#include "task.h"

#include <string.h>

extern char executable_path[];

#if defined(_AIX) || defined(__MVS__) || defined(__PASE__)
static int exit_status;
static int term_signal;
#endif


int strtok_helper(void) {
  char actual[4096];
  char expected[4096];
  size_t actual_len;
  size_t expected_len;

  actual_len = sizeof(actual);
  expected_len = sizeof(expected);

  ASSERT_OK(uv_os_getenv("UV_STRTOK_EXPECTED", expected, &expected_len));
  ASSERT_OK(uv_exepath(actual, &actual_len));
  ASSERT_EQ(actual_len, strlen(actual));
  ASSERT_OK(strcmp(actual, expected));

  return 0;
}


#if defined(_AIX) || defined(__MVS__) || defined(__PASE__)
static void exit_cb(uv_process_t* process, int64_t status, int signal) {
  exit_status = (int) status;
  term_signal = signal;
  uv_close((uv_handle_t*) process, NULL);
}
#endif


TEST_IMPL(strtok) {
#if !defined(_AIX) && !defined(__MVS__) && !defined(__PASE__)
  RETURN_SKIP("uv__strtok is not reachable through a public API on this platform");
#else
  char expected[4096];
  char dirname[4096];
  char path_env[8192];
  char old_path[8192];
  char* slash;
  char* args[3];
  uv_process_t process;
  uv_process_options_t options;
  size_t expected_len;
  size_t old_path_len;
  int old_path_set;
  int r;

  expected_len = sizeof(expected);
  ASSERT_OK(uv_exepath(expected, &expected_len));
  ASSERT_EQ(expected_len, strlen(expected));

  snprintf(dirname, sizeof(dirname), "%s", expected);
  slash = strrchr(dirname, '/');
  ASSERT_NOT_NULL(slash);
  *slash++ = '\0';

  old_path_len = sizeof(old_path);
  old_path_set = (uv_os_getenv("PATH", old_path, &old_path_len) == 0);

  snprintf(path_env,
           sizeof(path_env),
           ":/definitely/not/here:%s::/still/not/here",
           dirname);

  ASSERT_OK(uv_os_setenv("PATH", path_env));
  ASSERT_OK(uv_os_setenv("UV_STRTOK_EXPECTED", expected));

  memset(&process, 0, sizeof(process));
  memset(&options, 0, sizeof(options));

  args[0] = slash;
  args[1] = "strtok_helper";
  args[2] = NULL;

  options.file = slash;
  options.args = args;
  options.exit_cb = exit_cb;

  exit_status = -1;
  term_signal = -1;

  r = uv_spawn(uv_default_loop(), &process, &options);

  if (old_path_set)
    ASSERT_OK(uv_os_setenv("PATH", old_path));
  else
    ASSERT_OK(uv_os_unsetenv("PATH"));
  ASSERT_OK(uv_os_unsetenv("UV_STRTOK_EXPECTED"));

  ASSERT_OK(r);
  ASSERT_OK(uv_run(uv_default_loop(), UV_RUN_DEFAULT));
  ASSERT_OK(exit_status);
  ASSERT_OK(term_signal);

  MAKE_VALGRIND_HAPPY(uv_default_loop());
  return 0;
#endif
}
