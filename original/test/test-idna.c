/* Copyright The libuv project and contributors. All rights reserved.
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

#include "task.h"

#include <string.h>

extern char executable_path[];


static int resolve_sync(const char* hostname,
                        const struct addrinfo* hints,
                        uv_getaddrinfo_t* req) {
  return uv_getaddrinfo(uv_default_loop(), req, NULL, hostname, NULL, hints);
}


static int resolve_numeric_sync(const char* hostname,
                                int family,
                                uv_getaddrinfo_t* req) {
  struct addrinfo hints;

  memset(&hints, 0, sizeof(hints));
  hints.ai_family = family;
  hints.ai_flags = AI_NUMERICHOST;

  return resolve_sync(hostname, &hints, req);
}


static void assert_ipv4_result(const uv_getaddrinfo_t* req,
                               const char* expected_ip) {
  struct sockaddr_in* addr;
  char ip[sizeof("255.255.255.255")];

  ASSERT_NOT_NULL(req->addrinfo);
  ASSERT_EQ(AF_INET, req->addrinfo->ai_family);

  addr = (struct sockaddr_in*) req->addrinfo->ai_addr;
  ASSERT_OK(uv_ip4_name(addr, ip, sizeof(ip)));
  ASSERT_OK(strcmp(expected_ip, ip));
}


static void assert_numeric_host(const char* hostname, const char* expected_ip) {
  uv_getaddrinfo_t req;

  ASSERT_OK(resolve_numeric_sync(hostname, AF_INET, &req));
  assert_ipv4_result(&req, expected_ip);
  uv_freeaddrinfo(req.addrinfo);
}


static void assert_idna_host(const char* hostname, const char* expected_ip) {
  uv_getaddrinfo_t req;
  struct addrinfo hints;

  memset(&hints, 0, sizeof(hints));
  hints.ai_family = AF_INET;

  ASSERT_OK(resolve_sync(hostname, &hints, &req));
  assert_ipv4_result(&req, expected_ip);
  uv_freeaddrinfo(req.addrinfo);
}


#ifdef UV_TEST_HAS_IDNA_INTERCEPT
static int idna_exit_status;
static int idna_term_signal;


static void idna_exit_cb(uv_process_t* process, int64_t status, int signal) {
  idna_exit_status = (int) status;
  idna_term_signal = signal;
  uv_close((uv_handle_t*) process, NULL);
}


static char* dup_env_var(const char* name) {
  const char* value;
  char* copy;
  size_t len;

  value = getenv(name);
  if (value == NULL)
    return NULL;

  len = strlen(value) + 1;
  copy = malloc(len);
  ASSERT_NOT_NULL(copy);
  memcpy(copy, value, len);
  return copy;
}


static void restore_env_var(const char* name, char* value) {
  if (value != NULL) {
    ASSERT_OK(uv_os_setenv(name, value));
    free(value);
  } else {
    ASSERT_OK(uv_os_unsetenv(name));
  }
}


static void run_idna_toascii_helper(void) {
  char* args[3];
  uv_process_t process;
  uv_process_options_t options;
  char* old_preload;
  char* preload_value;
  size_t preload_len;
#ifdef __APPLE__
  char* old_force_flat_namespace;
#endif

#ifdef __APPLE__
  static const char preload_env_name[] = "DYLD_INSERT_LIBRARIES";
#else
  static const char preload_env_name[] = "LD_PRELOAD";
#endif

  old_preload = dup_env_var(preload_env_name);
  preload_len = sizeof(UV_TEST_IDNA_INTERCEPT_PATH);
  if (old_preload != NULL && old_preload[0] != '\0')
    preload_len += strlen(old_preload) + 1;

  preload_value = malloc(preload_len);
  ASSERT_NOT_NULL(preload_value);

  if (old_preload != NULL && old_preload[0] != '\0')
    snprintf(preload_value,
             preload_len,
             "%s:%s",
             UV_TEST_IDNA_INTERCEPT_PATH,
             old_preload);
  else
    snprintf(preload_value, preload_len, "%s", UV_TEST_IDNA_INTERCEPT_PATH);

  ASSERT_OK(uv_os_setenv(preload_env_name, preload_value));
  free(preload_value);

#ifdef __APPLE__
  old_force_flat_namespace = dup_env_var("DYLD_FORCE_FLAT_NAMESPACE");
  ASSERT_OK(uv_os_setenv("DYLD_FORCE_FLAT_NAMESPACE", "1"));
#endif

  memset(&process, 0, sizeof(process));
  memset(&options, 0, sizeof(options));

  args[0] = executable_path;
  args[1] = "idna_toascii_helper";
  args[2] = NULL;

  options.file = executable_path;
  options.args = args;
  options.exit_cb = idna_exit_cb;

  idna_exit_status = -1;
  idna_term_signal = -1;

  ASSERT_OK(uv_spawn(uv_default_loop(), &process, &options));

  restore_env_var(preload_env_name, old_preload);
#ifdef __APPLE__
  restore_env_var("DYLD_FORCE_FLAT_NAMESPACE", old_force_flat_namespace);
#endif

  ASSERT_OK(uv_run(uv_default_loop(), UV_RUN_DEFAULT));
  ASSERT_OK(idna_exit_status);
  ASSERT_OK(idna_term_signal);
}
#endif


int idna_toascii_helper(void) {
#ifdef UV_TEST_HAS_IDNA_INTERCEPT
  assert_idna_host("example.com.", "127.0.0.10");
  assert_idna_host("xn--maana-pta.com", "127.0.0.11");
  assert_idna_host("mañana.com", "127.0.0.11");
  assert_idna_host("mañana。com", "127.0.0.11");
  assert_idna_host("mañana．com", "127.0.0.11");
  assert_idna_host("mañana｡com", "127.0.0.11");
  assert_idna_host("xn--fa-hia.de", "127.0.0.12");
  assert_idna_host("faß.de", "127.0.0.12");
  assert_idna_host("bücher.com", "127.0.0.13");
  assert_idna_host("café.com", "127.0.0.14");
  assert_idna_host("café.café.com", "127.0.0.15");
  assert_idna_host("straße.de", "127.0.0.16");

  MAKE_VALGRIND_HAPPY(uv_default_loop());
#endif

  return 0;
}


TEST_IMPL(utf8_decode1) {
#ifdef __MVS__
  RETURN_SKIP("IDNA conversion is not supported on z/OS");
#else
  uv_getaddrinfo_t req;

  ASSERT_EQ(UV_EINVAL, resolve_sync("\xC0\x80", NULL, &req));
  ASSERT_EQ(UV_EINVAL, resolve_sync("\xED\xA0\x80", NULL, &req));
  ASSERT_EQ(UV_EINVAL, resolve_sync("\xC0\x80\xC1\x80.com", NULL, &req));

  MAKE_VALGRIND_HAPPY(uv_default_loop());
  return 0;
#endif
}


TEST_IMPL(utf8_decode1_overrun) {
#ifdef __MVS__
  RETURN_SKIP("IDNA conversion is not supported on z/OS");
#else
  uv_getaddrinfo_t req;

  ASSERT_EQ(UV_EINVAL, resolve_sync("", NULL, &req));
  ASSERT_EQ(UV_EINVAL, resolve_sync("\xC2", NULL, &req));
  ASSERT_EQ(UV_EINVAL, resolve_sync("\xE0\xA0", NULL, &req));
  ASSERT_EQ(UV_EINVAL, resolve_sync("\xF0\x90\x80", NULL, &req));

  MAKE_VALGRIND_HAPPY(uv_default_loop());
  return 0;
#endif
}


TEST_IMPL(idna_toascii) {
#ifdef __MVS__
  RETURN_SKIP("IDNA conversion is not supported on z/OS");
#else
  assert_numeric_host("127.0.0.1", "127.0.0.1");
  assert_numeric_host("127。0。0。1", "127.0.0.1");
  assert_numeric_host("127．0．0．1", "127.0.0.1");
  assert_numeric_host("127｡0｡0｡1", "127.0.0.1");

#ifdef UV_TEST_HAS_IDNA_INTERCEPT
  run_idna_toascii_helper();
#endif

  MAKE_VALGRIND_HAPPY(uv_default_loop());
  return 0;
#endif
}
