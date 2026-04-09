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
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#ifndef _WIN32
# include <unistd.h>
#endif

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

#ifndef _WIN32
static void resolve_ipv4(uv_loop_t* loop,
                         const char* node,
                         char ip[sizeof("255.255.255.255")]) {
  uv_getaddrinfo_t req;
  struct addrinfo hints;
  struct addrinfo* res;

  memset(&req, 0, sizeof(req));
  memset(&hints, 0, sizeof(hints));
  hints.ai_family = AF_INET;

  ASSERT_OK(uv_getaddrinfo(loop, &req, NULL, node, NULL, &hints));
  res = req.addrinfo;
  ASSERT_NOT_NULL(res);
  ASSERT_EQ(AF_INET, res->ai_family);
  ASSERT_OK(uv_ip4_name((const struct sockaddr_in*) res->ai_addr,
                        ip,
                        sizeof("255.255.255.255")));
  uv_freeaddrinfo(res);
}

static void assert_idna_alias_resolves(const char* unicode, const char* ascii) {
  uv_loop_t loop;
  char unicode_ip[sizeof("255.255.255.255")];
  char ascii_ip[sizeof("255.255.255.255")];

  ASSERT_OK(uv_loop_init(&loop));
  resolve_ipv4(&loop, unicode, unicode_ip);
  resolve_ipv4(&loop, ascii, ascii_ip);
  ASSERT_STR_EQ("127.0.0.1", unicode_ip);
  ASSERT_STR_EQ(unicode_ip, ascii_ip);
  MAKE_VALGRIND_HAPPY(&loop);
}
#endif  /* !_WIN32 */

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
#ifdef _WIN32
  assert_idna_equivalent("mañana.invalid.", "xn--maana-pta.invalid.");
  assert_idna_equivalent("bücher.invalid.", "xn--bcher-kva.invalid.");
  assert_idna_equivalent("☃-⌘.invalid.", "xn----dqo34k.invalid.");
#else
  char alias_template[] = "/tmp/libuv-idna-XXXXXX";
  char* hostaliases;
  FILE* alias_file;
  char* old_hostaliases;
  char* old_hostaliases_copy;
  int fd;

  fd = mkstemp(alias_template);
  ASSERT_GE(fd, 0);

  alias_file = fdopen(fd, "w");
  ASSERT_NOT_NULL(alias_file);
  ASSERT_GE(fputs("xn--tda localhost\n", alias_file), 0);
  ASSERT_GE(fputs("xn--maana-pta localhost\n", alias_file), 0);
  ASSERT_GE(fputs("xn--bcher-kva localhost\n", alias_file), 0);
  ASSERT_GE(fputs("xn----dqo34k localhost\n", alias_file), 0);
  ASSERT_OK(fclose(alias_file));

  old_hostaliases = getenv("HOSTALIASES");
  old_hostaliases_copy = old_hostaliases != NULL ? strdup(old_hostaliases) : NULL;
  ASSERT(old_hostaliases == NULL || old_hostaliases_copy != NULL);

  hostaliases = alias_template;
  ASSERT_OK(setenv("HOSTALIASES", hostaliases, 1));

  assert_idna_alias_resolves("ü", "xn--tda");
  assert_idna_alias_resolves("mañana", "xn--maana-pta");
  assert_idna_alias_resolves("bücher", "xn--bcher-kva");
  assert_idna_alias_resolves("☃-⌘", "xn----dqo34k");
  assert_idna_equivalent("mañana。invalid.", "xn--maana-pta.invalid.");

  if (old_hostaliases_copy != NULL) {
    ASSERT_OK(setenv("HOSTALIASES", old_hostaliases_copy, 1));
    free(old_hostaliases_copy);
  } else {
    ASSERT_OK(unsetenv("HOSTALIASES"));
  }

  ASSERT_OK(unlink(alias_template));
#endif
  return 0;
}

#endif  /* __MVS__ */
