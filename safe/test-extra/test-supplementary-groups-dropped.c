#ifndef _WIN32

#include <fcntl.h>
#include <stddef.h>
#include <string.h>
#include <unistd.h>

#include "task.h"

struct uv_test_privdrop_call {
  int kind;
  int value0;
  int value1;
};

void uv_test_privdrop_set_record_fd(int fd);
void uv_test_privdrop_reset(void);

static int exit_cb_called;
static int close_cb_called;


static void close_cb(uv_handle_t* handle) {
  close_cb_called++;
}


static void exit_cb(uv_process_t* process,
                    int64_t exit_status,
                    int term_signal) {
  ASSERT_OK(exit_status);
  ASSERT_OK(term_signal);
  exit_cb_called++;
  uv_close((uv_handle_t*) process, close_cb);
}


TEST_IMPL(supplementary_groups_dropped) {
  uv_process_t process;
  uv_process_options_t options;
  struct uv_test_privdrop_call calls[8];
  char* args[2];
  int pipefd[2];
  ssize_t nread;
  int count;

  memset(&process, 0, sizeof(process));
  memset(&options, 0, sizeof(options));

  ASSERT_OK(pipe(pipefd));

  args[0] = "/bin/true";
  args[1] = NULL;

  options.file = args[0];
  options.args = args;
  options.flags = UV_PROCESS_SETUID | UV_PROCESS_SETGID;
  options.uid = 1234;
  options.gid = 4321;
  options.exit_cb = exit_cb;

  exit_cb_called = 0;
  close_cb_called = 0;
  uv_test_privdrop_set_record_fd(pipefd[1]);

  ASSERT_OK(uv_spawn(uv_default_loop(), &process, &options));

  uv_test_privdrop_reset();
  ASSERT_OK(close(pipefd[1]));
  ASSERT_OK(uv_run(uv_default_loop(), UV_RUN_DEFAULT));

  ASSERT_EQ(1, exit_cb_called);
  ASSERT_EQ(1, close_cb_called);

  nread = read(pipefd[0], calls, sizeof(calls));
  ASSERT_EQ((int) (3 * sizeof(calls[0])), (int) nread);
  ASSERT_OK(close(pipefd[0]));

  count = (int) (nread / (ssize_t) sizeof(calls[0]));
  ASSERT_EQ(3, count);
  ASSERT_EQ('g', calls[0].kind);
  ASSERT_OK(calls[0].value0);
  ASSERT_EQ(-1, calls[0].value1);
  ASSERT_EQ('G', calls[1].kind);
  ASSERT_EQ(4321, calls[1].value0);
  ASSERT_EQ('U', calls[2].kind);
  ASSERT_EQ(1234, calls[2].value0);

  MAKE_VALGRIND_HAPPY(uv_default_loop());
  return 0;
}

#endif
