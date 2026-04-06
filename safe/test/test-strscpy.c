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


TEST_IMPL(strscpy) {
  static const char expected[] = "ffff:ffff:ffff:ffff:ffff:ffff:ffff:ffff";
  struct sockaddr_in6 addr;
  char exact[sizeof(expected)];
  char short_buf[sizeof(expected) - 1];

  ASSERT_OK(uv_ip6_addr(expected, TEST_PORT, &addr));

  ASSERT_EQ(UV_ENOSPC, uv_ip6_name(&addr, short_buf, sizeof(short_buf)));
  ASSERT_EQ(UV_ENOSPC, uv_ip6_name(&addr, exact, 0));

  ASSERT_OK(uv_ip6_name(&addr, exact, sizeof(exact)));
  ASSERT_OK(strcmp(exact, expected));

  return 0;
}
