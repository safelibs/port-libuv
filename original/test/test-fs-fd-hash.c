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

TEST_IMPL(fs_fd_hash) {
  static const char path[] = "test-fs-fd-hash";
  static const char payload[] = "filemap";
  uv_fs_t open_req;
  uv_fs_t write_req;
  uv_fs_t read_req;
  uv_fs_t close_req;
  uv_fs_t unlink_req;
  uv_file file;
  uv_buf_t buf;
  char read_back[sizeof(payload)];
  int r;

  r = uv_fs_unlink(NULL, &unlink_req, path, NULL);
  ASSERT(r == 0 || r == UV_ENOENT);
  uv_fs_req_cleanup(&unlink_req);

  buf = uv_buf_init((char*) payload, sizeof(payload) - 1);
  r = uv_fs_open(NULL,
                 &open_req,
                 path,
                 UV_FS_O_WRONLY | UV_FS_O_CREAT | UV_FS_O_TRUNC | UV_FS_O_FILEMAP,
                 S_IWUSR | S_IRUSR,
                 NULL);
  ASSERT_GE(r, 0);
  file = r;
  ASSERT_EQ(file, (uv_file) uv_fs_get_result(&open_req));
  uv_fs_req_cleanup(&open_req);

  r = uv_fs_write(NULL, &write_req, file, &buf, 1, 0, NULL);
  ASSERT_EQ(sizeof(payload) - 1, r);
  ASSERT_EQ((int64_t) (sizeof(payload) - 1), uv_fs_get_result(&write_req));
  uv_fs_req_cleanup(&write_req);

  r = uv_fs_close(NULL, &close_req, file, NULL);
  ASSERT_OK(r);
  ASSERT_OK(uv_fs_get_result(&close_req));
  uv_fs_req_cleanup(&close_req);

  memset(read_back, 0, sizeof(read_back));
  buf = uv_buf_init(read_back, sizeof(read_back) - 1);
  r = uv_fs_open(NULL,
                 &open_req,
                 path,
                 UV_FS_O_RDONLY | UV_FS_O_FILEMAP,
                 0,
                 NULL);
  ASSERT_GE(r, 0);
  file = r;
  uv_fs_req_cleanup(&open_req);

  r = uv_fs_read(NULL, &read_req, file, &buf, 1, 0, NULL);
  ASSERT_EQ(sizeof(payload) - 1, r);
  ASSERT_EQ((int64_t) (sizeof(payload) - 1), uv_fs_get_result(&read_req));
  ASSERT_OK(memcmp(read_back, payload, sizeof(payload) - 1));
  uv_fs_req_cleanup(&read_req);

  r = uv_fs_close(NULL, &close_req, file, NULL);
  ASSERT_OK(r);
  ASSERT_OK(uv_fs_get_result(&close_req));
  uv_fs_req_cleanup(&close_req);

  r = uv_fs_unlink(NULL, &unlink_req, path, NULL);
  ASSERT_OK(r);
  ASSERT_OK(uv_fs_get_result(&unlink_req));
  uv_fs_req_cleanup(&unlink_req);

  return 0;
}

#else

typedef int file_has_no_tests;  /* ISO C forbids an empty translation unit. */

#endif  /* ifndef _WIN32 */
