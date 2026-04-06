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

#include <stdio.h>
#include <string.h>

static int public_api_cookie1;
static int public_api_cookie2;

static uv_once_t public_api_once_guard = UV_ONCE_INIT;
static int public_api_once_cb_called;

static uv_udp_t public_api_udp_server;
static uv_udp_t public_api_udp_client;
static uv_udp_send_t public_api_udp_send_req;
static int public_api_udp_send_cb_called;
static int public_api_udp_close_cb_called;

static int public_api_req_cb_called;


static void public_api_once_cb(void) {
  public_api_once_cb_called++;
}


static void public_api_once_thread(void* arg) {
  ASSERT_NULL(arg);
  uv_once(&public_api_once_guard, public_api_once_cb);
}


static void public_api_fs_stat_cb(uv_fs_t* req) {
  public_api_req_cb_called++;

  ASSERT_EQ(UV_FS, uv_req_get_type((uv_req_t*) req));
  ASSERT_PTR_EQ(&public_api_cookie1, uv_req_get_data((uv_req_t*) req));
  ASSERT_EQ(UV_FS_STAT, uv_fs_get_type(req));
  ASSERT_GE(uv_fs_get_result(req), 0);

  uv_fs_req_cleanup(req);
}


static void public_api_udp_close_cb(uv_handle_t* handle) {
  ASSERT_PTR_NE(handle, NULL);
  public_api_udp_close_cb_called++;
}


static void public_api_udp_send_cb(uv_udp_send_t* req, int status) {
  ASSERT_OK(status);
  ASSERT_PTR_EQ(&public_api_udp_send_req, req);
  ASSERT_UINT64_EQ(0,
                   (uint64_t) uv_udp_get_send_queue_size(&public_api_udp_client));
  ASSERT_UINT64_EQ(0,
                   (uint64_t) uv_udp_get_send_queue_count(&public_api_udp_client));

  public_api_udp_send_cb_called++;

  uv_close((uv_handle_t*) &public_api_udp_client, public_api_udp_close_cb);
  uv_close((uv_handle_t*) &public_api_udp_server, public_api_udp_close_cb);
}


TEST_IMPL(version) {
  char expected[32];
  int r;

  ASSERT_EQ(UV_VERSION_HEX, uv_version());

#if UV_VERSION_IS_RELEASE
  r = snprintf(expected,
               sizeof(expected),
               "%d.%d.%d",
               UV_VERSION_MAJOR,
               UV_VERSION_MINOR,
               UV_VERSION_PATCH);
#else
  r = snprintf(expected,
               sizeof(expected),
               "%d.%d.%d-%s",
               UV_VERSION_MAJOR,
               UV_VERSION_MINOR,
               UV_VERSION_PATCH,
               UV_VERSION_SUFFIX);
#endif
  ASSERT_GT(r, 0);
  ASSERT_STR_EQ(expected, uv_version_string());

  return 0;
}


TEST_IMPL(err_name_r) {
  char buf[64];
  char* name;

  name = uv_err_name_r(UV_EINVAL, buf, sizeof(buf));
  ASSERT_PTR_EQ(buf, name);
  ASSERT_STR_EQ("EINVAL", name);
  ASSERT_STR_EQ("EINVAL", uv_err_name(UV_EINVAL));

  name = uv_err_name_r(1337, buf, sizeof(buf));
  ASSERT_PTR_EQ(buf, name);
  ASSERT_NOT_NULL(strstr(name, "1337"));

  return 0;
}


TEST_IMPL(loop_new_delete) {
  uv_loop_t* loop;

  loop = uv_loop_new();
  ASSERT_NOT_NULL(loop);
  ASSERT_OK(uv_run(loop, UV_RUN_DEFAULT));
  uv_loop_delete(loop);

  return 0;
}


TEST_IMPL(req_accessors) {
  uv_loop_t loop;
  uv_fs_t req;
  int r;

  public_api_req_cb_called = 0;

  ASSERT_OK(uv_loop_init(&loop));

  memset(&req, 0, sizeof(req));
  uv_req_set_data((uv_req_t*) &req, &public_api_cookie1);
  ASSERT_PTR_EQ(&public_api_cookie1, uv_req_get_data((uv_req_t*) &req));

  r = uv_fs_stat(&loop, &req, ".", public_api_fs_stat_cb);
  ASSERT_OK(r);
  ASSERT_EQ(0, public_api_req_cb_called);

  ASSERT_OK(uv_run(&loop, UV_RUN_DEFAULT));
  ASSERT_EQ(1, public_api_req_cb_called);

  uv_req_set_data((uv_req_t*) &req, &public_api_cookie2);
  ASSERT_PTR_EQ(&public_api_cookie2, uv_req_get_data((uv_req_t*) &req));

  ASSERT_OK(uv_loop_close(&loop));
  return 0;
}


TEST_IMPL(thread_once) {
  uv_thread_t threads[4];
  size_t i;
  int r;

  public_api_once_cb_called = 0;

  uv_once(&public_api_once_guard, public_api_once_cb);

  for (i = 0; i < ARRAY_SIZE(threads); i++) {
    r = uv_thread_create(threads + i, public_api_once_thread, NULL);
    ASSERT_OK(r);
  }

  for (i = 0; i < ARRAY_SIZE(threads); i++) {
    r = uv_thread_join(threads + i);
    ASSERT_OK(r);
  }

  uv_once(&public_api_once_guard, public_api_once_cb);
  ASSERT_EQ(1, public_api_once_cb_called);

  return 0;
}


TEST_IMPL(udp_send_queue_accessors) {
  struct sockaddr_in addr;
  uv_buf_t buf;
  char payload[] = "PING";
  int r;

  public_api_udp_send_cb_called = 0;
  public_api_udp_close_cb_called = 0;

  ASSERT_OK(uv_ip4_addr("127.0.0.1", TEST_PORT_3, &addr));

  r = uv_udp_init(uv_default_loop(), &public_api_udp_server);
  ASSERT_OK(r);

  r = uv_udp_bind(&public_api_udp_server, (const struct sockaddr*) &addr, 0);
  ASSERT_OK(r);

  r = uv_udp_init(uv_default_loop(), &public_api_udp_client);
  ASSERT_OK(r);

  ASSERT_UINT64_EQ(0,
                   (uint64_t) uv_udp_get_send_queue_size(&public_api_udp_client));
  ASSERT_UINT64_EQ(0,
                   (uint64_t) uv_udp_get_send_queue_count(&public_api_udp_client));

  buf = uv_buf_init(payload, 4);
  r = uv_udp_send(&public_api_udp_send_req,
                  &public_api_udp_client,
                  &buf,
                  1,
                  (const struct sockaddr*) &addr,
                  public_api_udp_send_cb);
  ASSERT_OK(r);

  ASSERT_UINT64_EQ(4,
                   (uint64_t) uv_udp_get_send_queue_size(&public_api_udp_client));
  ASSERT_UINT64_EQ(1,
                   (uint64_t) uv_udp_get_send_queue_count(&public_api_udp_client));

  ASSERT_OK(uv_run(uv_default_loop(), UV_RUN_DEFAULT));

  ASSERT_EQ(1, public_api_udp_send_cb_called);
  ASSERT_EQ(2, public_api_udp_close_cb_called);

  MAKE_VALGRIND_HAPPY(uv_default_loop());
  return 0;
}
