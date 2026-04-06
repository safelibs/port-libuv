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

#if defined(_WIN32) && !defined(USING_UV_SHARED)

#include "uv.h"
#include "task.h"

#define FILE_COUNT 300
#define PATH_SIZE 64
#define PAYLOAD_SIZE 64

static void unlink_if_exists(const char* path) {
  uv_fs_t req;
  int r;

  r = uv_fs_unlink(NULL, &req, path, NULL);
  ASSERT(r == 0 || r == UV_ENOENT);
  uv_fs_req_cleanup(&req);
}

static uv_file open_filemap(const char* path) {
  uv_fs_t req;
  int r;

  r = uv_fs_open(NULL,
                 &req,
                 path,
                 UV_FS_O_RDWR | UV_FS_O_CREAT | UV_FS_O_TRUNC | UV_FS_O_FILEMAP,
                 S_IWUSR | S_IRUSR,
                 NULL);
  ASSERT_GE(r, 0);
  ASSERT_EQ((int64_t) r, uv_fs_get_result(&req));
  uv_fs_req_cleanup(&req);

  return r;
}

static void close_file(uv_file file) {
  uv_fs_t req;
  int r;

  r = uv_fs_close(NULL, &req, file, NULL);
  ASSERT_OK(r);
  ASSERT_OK(uv_fs_get_result(&req));
  uv_fs_req_cleanup(&req);
}

static void write_current(uv_file file, const char* payload) {
  uv_fs_t req;
  uv_buf_t buf;
  size_t len;
  int r;

  len = strlen(payload);
  buf = uv_buf_init((char*) payload, len);
  r = uv_fs_write(NULL, &req, file, &buf, 1, -1, NULL);
  ASSERT_EQ((int) len, r);
  ASSERT_EQ((int64_t) len, uv_fs_get_result(&req));
  uv_fs_req_cleanup(&req);
}

static void read_at_start(uv_file file, char* actual, size_t actual_size) {
  uv_fs_t req;
  uv_buf_t buf;
  int r;

  memset(actual, 0, actual_size);
  buf = uv_buf_init(actual, actual_size - 1);
  r = uv_fs_read(NULL, &req, file, &buf, 1, 0, NULL);
  ASSERT_GE(r, 0);
  ASSERT_EQ((int64_t) r, uv_fs_get_result(&req));
  actual[r] = '\0';
  uv_fs_req_cleanup(&req);
}

TEST_IMPL(fs_fd_hash) {
  uv_file files[FILE_COUNT];
  char paths[FILE_COUNT][PATH_SIZE];
  char expected[FILE_COUNT][PAYLOAD_SIZE];
  char actual[PAYLOAD_SIZE];
  int i;

  /*
   * Keep a large number of filemap-backed descriptors live at once to stress
   * insertion and collision handling, then update some in-place and remove /
   * recreate others to exercise replacement and cleanup through public APIs.
   */
  for (i = 0; i < FILE_COUNT; i++) {
    snprintf(paths[i], sizeof(paths[i]), "test-fs-fd-hash-%03d", i);
    unlink_if_exists(paths[i]);

    files[i] = open_filemap(paths[i]);
    snprintf(expected[i], sizeof(expected[i]), "file-%03d", i);
    write_current(files[i], expected[i]);
  }

  for (i = 0; i < FILE_COUNT; i++) {
    read_at_start(files[i], actual, sizeof(actual));
    ASSERT_OK(strcmp(expected[i], actual));
  }

  for (i = 0; i < FILE_COUNT; i += 17) {
    write_current(files[i], "+update");
    ASSERT_NOT_NULL(strcat(expected[i], "+update"));
    read_at_start(files[i], actual, sizeof(actual));
    ASSERT_OK(strcmp(expected[i], actual));
  }

  for (i = 0; i < FILE_COUNT; i += 3) {
    close_file(files[i]);
    unlink_if_exists(paths[i]);

    files[i] = open_filemap(paths[i]);
    snprintf(expected[i], sizeof(expected[i]), "reopen-%03d", i);
    write_current(files[i], expected[i]);
    read_at_start(files[i], actual, sizeof(actual));
    ASSERT_OK(strcmp(expected[i], actual));
  }

  for (i = 0; i < FILE_COUNT; i++) {
    read_at_start(files[i], actual, sizeof(actual));
    ASSERT_OK(strcmp(expected[i], actual));
    close_file(files[i]);
    unlink_if_exists(paths[i]);
  }

  return 0;
}

#else

typedef int file_has_no_tests;  /* ISO C forbids an empty translation unit. */

#endif  /* ifndef _WIN32 */
