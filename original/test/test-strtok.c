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


TEST_IMPL(strtok) {
  char expected[32];
  const char* actual;
  unsigned int version;
  int len;

  version = uv_version();
  ASSERT_EQ(UV_VERSION_HEX, version);
  ASSERT_EQ(UV_VERSION_MAJOR, (version >> 16) & 0xff);
  ASSERT_EQ(UV_VERSION_MINOR, (version >> 8) & 0xff);
  ASSERT_EQ(UV_VERSION_PATCH, version & 0xff);

  len = snprintf(expected,
                 sizeof(expected),
                 "%d.%d.%d%s",
                 UV_VERSION_MAJOR,
                 UV_VERSION_MINOR,
                 UV_VERSION_PATCH,
                 UV_VERSION_SUFFIX);
  ASSERT_GT(len, 0);
  ASSERT_LT((size_t) len, sizeof(expected));

  actual = uv_version_string();
  ASSERT_NOT_NULL(actual);
  ASSERT_OK(strcmp(actual, expected));

  return 0;
}
