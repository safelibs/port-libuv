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


static int resolve_sync(const char* hostname) {
  uv_getaddrinfo_t req;
  int r;

  r = uv_getaddrinfo(uv_default_loop(), &req, NULL, hostname, NULL, NULL);
  if (r == 0)
    uv_freeaddrinfo(req.addrinfo);

  return r;
}


TEST_IMPL(utf8_decode1) {
#ifdef __MVS__
  RETURN_SKIP("IDNA conversion is not supported on z/OS");
#else
  ASSERT_EQ(UV_EINVAL, resolve_sync("\xC0\x80"));
  ASSERT_EQ(UV_EINVAL, resolve_sync("\xED\xA0\x80"));

  MAKE_VALGRIND_HAPPY(uv_default_loop());
  return 0;
#endif
}


TEST_IMPL(utf8_decode1_overrun) {
#ifdef __MVS__
  RETURN_SKIP("IDNA conversion is not supported on z/OS");
#else
  ASSERT_EQ(UV_EINVAL, resolve_sync("\xC2"));
  ASSERT_EQ(UV_EINVAL, resolve_sync("\xE0\xA0"));
  ASSERT_EQ(UV_EINVAL, resolve_sync("\xF0\x90\x80"));

  MAKE_VALGRIND_HAPPY(uv_default_loop());
  return 0;
#endif
}


TEST_IMPL(idna_toascii) {
#ifdef __MVS__
  RETURN_SKIP("IDNA conversion is not supported on z/OS");
#else
  static const char* hostnames[] = {
    "bücher.com",
    "mañana.com",
    "faß.de",
  };
  size_t i;

  for (i = 0; i < ARRAY_SIZE(hostnames); i++) {
    int r;

    r = resolve_sync(hostnames[i]);
    if (r == 0) {
      MAKE_VALGRIND_HAPPY(uv_default_loop());
      return 0;
    }

    if (r != UV_EAI_AGAIN &&
        r != UV_EAI_FAIL &&
        r != UV_EAI_NONAME) {
      ASSERT_OK(r);
    }
  }

  RETURN_SKIP("No resolvable IDN hostnames available in this environment");
#endif
}
