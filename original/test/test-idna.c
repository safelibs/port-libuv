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

#include "uv.h"
#include "task.h"
#include <string.h>

static int getaddrinfo_sync(uv_loop_t* loop,
                            const char* node,
                            struct addrinfo** res) {
  uv_getaddrinfo_t req;
  int r;

  memset(&req, 0, sizeof(req));
  r = uv_getaddrinfo(loop, &req, NULL, node, NULL, NULL);
  if (res != NULL)
    *res = req.addrinfo;
  else
    uv_freeaddrinfo(req.addrinfo);

  return r;
}

static void assert_getaddrinfo_invalid(const char* node) {
  uv_loop_t loop;

  ASSERT_OK(uv_loop_init(&loop));
  ASSERT_EQ(UV_EINVAL, getaddrinfo_sync(&loop, node, NULL));
  MAKE_VALGRIND_HAPPY(&loop);
}

static void assert_idna_equivalent(const char* unicode, const char* ascii) {
  uv_loop_t loop;
  struct addrinfo* unicode_res;
  struct addrinfo* ascii_res;
  int unicode_r;
  int ascii_r;

  unicode_res = NULL;
  ascii_res = NULL;

  ASSERT_OK(uv_loop_init(&loop));

  unicode_r = getaddrinfo_sync(&loop, unicode, &unicode_res);
  ascii_r = getaddrinfo_sync(&loop, ascii, &ascii_res);

  ASSERT_EQ(ascii_r, unicode_r);
  if (unicode_r == 0) {
    ASSERT_NOT_NULL(unicode_res);
    ASSERT_NOT_NULL(ascii_res);
    ASSERT_EQ(ascii_res->ai_family, unicode_res->ai_family);
    ASSERT_EQ(ascii_res->ai_socktype, unicode_res->ai_socktype);
    ASSERT_EQ(ascii_res->ai_protocol, unicode_res->ai_protocol);
  } else {
    ASSERT_NULL(unicode_res);
    ASSERT_NULL(ascii_res);
  }

  uv_freeaddrinfo(unicode_res);
  uv_freeaddrinfo(ascii_res);
  MAKE_VALGRIND_HAPPY(&loop);
}

TEST_IMPL(utf8_decode1) {
  assert_getaddrinfo_invalid("\xC0\x80\xC1\x80.invalid.");
  assert_getaddrinfo_invalid("\xED\xA0\x80\xED\xA3\xBF.invalid.");
  assert_getaddrinfo_invalid("\xF4\x90\xC0\xC0.invalid.");
  return 0;
}

TEST_IMPL(utf8_decode1_overrun) {
  char truncated_2[] = { (char) 0xC2, '\0' };
  char truncated_3[] = { (char) 0xE0, (char) 0xA0, '\0' };
  char empty[] = "";

  assert_getaddrinfo_invalid(truncated_2);
  assert_getaddrinfo_invalid(truncated_3);
  assert_getaddrinfo_invalid(empty);
  return 0;
}

/* Doesn't work on z/OS because that platform uses EBCDIC, not ASCII. */
#ifndef __MVS__

TEST_IMPL(idna_toascii) {
  assert_idna_equivalent("mañana.invalid.", "xn--maana-pta.invalid.");
  assert_idna_equivalent("mañana。invalid.", "xn--maana-pta.invalid.");
  assert_idna_equivalent("bücher.invalid.", "xn--bcher-kva.invalid.");
  assert_idna_equivalent("☃-⌘.invalid.", "xn----dqo34k.invalid.");
  return 0;
}

#endif  /* __MVS__ */
