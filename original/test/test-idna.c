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


static int resolve_numeric_sync(const char* hostname,
                                int family,
                                uv_getaddrinfo_t* req) {
  struct addrinfo hints;

  memset(&hints, 0, sizeof(hints));
  hints.ai_family = family;
  hints.ai_flags = AI_NUMERICHOST;

  return uv_getaddrinfo(uv_default_loop(), req, NULL, hostname, NULL, &hints);
}


static void assert_numeric_host(const char* hostname, const char* expected_ip) {
  uv_getaddrinfo_t req;
  struct sockaddr_in* addr;
  char ip[sizeof("255.255.255.255")];

  ASSERT_OK(resolve_numeric_sync(hostname, AF_INET, &req));
  ASSERT_NOT_NULL(req.addrinfo);
  ASSERT_EQ(AF_INET, req.addrinfo->ai_family);

  addr = (struct sockaddr_in*) req.addrinfo->ai_addr;
  ASSERT_OK(uv_ip4_name(addr, ip, sizeof(ip)));
  ASSERT_OK(strcmp(expected_ip, ip));

  uv_freeaddrinfo(req.addrinfo);
}


TEST_IMPL(utf8_decode1) {
#ifdef __MVS__
  RETURN_SKIP("IDNA conversion is not supported on z/OS");
#else
  uv_getaddrinfo_t req;

  ASSERT_EQ(UV_EINVAL, resolve_numeric_sync("\xC0\x80", AF_UNSPEC, &req));
  ASSERT_EQ(UV_EINVAL, resolve_numeric_sync("\xED\xA0\x80", AF_UNSPEC, &req));

  MAKE_VALGRIND_HAPPY(uv_default_loop());
  return 0;
#endif
}


TEST_IMPL(utf8_decode1_overrun) {
#ifdef __MVS__
  RETURN_SKIP("IDNA conversion is not supported on z/OS");
#else
  uv_getaddrinfo_t req;

  ASSERT_EQ(UV_EINVAL, resolve_numeric_sync("\xC2", AF_UNSPEC, &req));
  ASSERT_EQ(UV_EINVAL, resolve_numeric_sync("\xE0\xA0", AF_UNSPEC, &req));
  ASSERT_EQ(UV_EINVAL, resolve_numeric_sync("\xF0\x90\x80", AF_UNSPEC, &req));

  MAKE_VALGRIND_HAPPY(uv_default_loop());
  return 0;
#endif
}


TEST_IMPL(idna_toascii) {
#ifdef __MVS__
  RETURN_SKIP("IDNA conversion is not supported on z/OS");
#else
  uv_getaddrinfo_t req;

  assert_numeric_host("127.0.0.1", "127.0.0.1");
  assert_numeric_host("127。0。0。1", "127.0.0.1");
  assert_numeric_host("127．0．0．1", "127.0.0.1");
  assert_numeric_host("127｡0｡0｡1", "127.0.0.1");

  ASSERT_EQ(UV_EAI_NONAME,
            resolve_numeric_sync("mañana.com", AF_UNSPEC, &req));
  ASSERT_EQ(UV_EAI_NONAME,
            resolve_numeric_sync("mañana。com", AF_UNSPEC, &req));
  ASSERT_EQ(UV_EAI_NONAME,
            resolve_numeric_sync("faß.de", AF_UNSPEC, &req));

  MAKE_VALGRIND_HAPPY(uv_default_loop());
  return 0;
#endif
}
