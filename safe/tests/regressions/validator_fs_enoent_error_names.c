/* Validator regression for impl-11-validator-fs-dns-process-fixes.
 *
 * Pins the errno -> uv error name/strerror mapping that Node.js fs.* probes
 * trip over when libuv-safe lacks an entry for a given UV_E* code. Drives
 * uv_fs_access on a known-missing path, then reads back the canonical name
 * and message via uv_err_name / uv_err_name_r / uv_strerror / uv_strerror_r.
 *
 * Originally observed against:
 *   - usage-nodejs-fs-access-existing-and-missing
 *   - usage-nodejs-fs-copyfile-unlink-chain
 *   - usage-nodejs-fs-cp-recursive
 * all of which reported `Unknown system error -2` instead of `ENOENT` because
 * the safe mapping table was missing the full UV_ERRNO_MAP entries.
 */

#include <uv.h>

#include <assert.h>
#include <fcntl.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <unistd.h>

static int expect_name(int err, const char* expected) {
  const char* got = uv_err_name(err);
  if (got == NULL || strcmp(got, expected) != 0) {
    fprintf(stderr,
            "uv_err_name(%d) = %s; want %s\n",
            err,
            got ? got : "(null)",
            expected);
    return 1;
  }
  return 0;
}

static int expect_strerror(int err, const char* expected) {
  const char* got = uv_strerror(err);
  if (got == NULL || strcmp(got, expected) != 0) {
    fprintf(stderr,
            "uv_strerror(%d) = %s; want %s\n",
            err,
            got ? got : "(null)",
            expected);
    return 1;
  }
  return 0;
}

static int expect_name_r(int err, const char* expected) {
  char buf[64];
  memset(buf, 0xab, sizeof(buf));
  char* got = uv_err_name_r(err, buf, sizeof(buf));
  if (got != buf) {
    fprintf(stderr, "uv_err_name_r(%d) returned non-buf pointer\n", err);
    return 1;
  }
  if (strcmp(buf, expected) != 0) {
    fprintf(stderr,
            "uv_err_name_r(%d) = %s; want %s\n",
            err,
            buf,
            expected);
    return 1;
  }
  return 0;
}

static int expect_strerror_r(int err, const char* expected) {
  char buf[128];
  memset(buf, 0xab, sizeof(buf));
  char* got = uv_strerror_r(err, buf, sizeof(buf));
  if (got != buf) {
    fprintf(stderr, "uv_strerror_r(%d) returned non-buf pointer\n", err);
    return 1;
  }
  if (strcmp(buf, expected) != 0) {
    fprintf(stderr,
            "uv_strerror_r(%d) = %s; want %s\n",
            err,
            buf,
            expected);
    return 1;
  }
  return 0;
}

int main(void) {
  /* Drive a real uv_fs_access against a path that cannot exist; the
   * synchronous request must return -ENOENT (-2 on Linux), populate
   * req->result with the same value, and survive uv_fs_req_cleanup. */
  char missing[] = "/tmp/libuv-safe-validator-missing-XXXXXX";
  int fd = mkstemp(missing);
  if (fd < 0) {
    perror("mkstemp");
    return 1;
  }
  close(fd);
  unlink(missing);

  uv_fs_t req;
  memset(&req, 0, sizeof(req));
  int r = uv_fs_access(NULL, &req, missing, F_OK, NULL);
  if (r != UV_ENOENT) {
    fprintf(stderr,
            "uv_fs_access on missing path returned %d (%s); want UV_ENOENT (%d)\n",
            r,
            uv_err_name(r),
            UV_ENOENT);
    uv_fs_req_cleanup(&req);
    return 1;
  }
  if (req.result != UV_ENOENT) {
    fprintf(stderr,
            "uv_fs_access req.result = %lld; want %d\n",
            (long long) req.result,
            UV_ENOENT);
    uv_fs_req_cleanup(&req);
    return 1;
  }
  uv_fs_req_cleanup(&req);

  /* Re-using the same stack-allocated request must be safe after cleanup. */
  memset(&req, 0, sizeof(req));
  r = uv_fs_access(NULL, &req, missing, F_OK, NULL);
  if (r != UV_ENOENT) {
    fprintf(stderr, "second uv_fs_access returned %d; want UV_ENOENT\n", r);
    uv_fs_req_cleanup(&req);
    return 1;
  }
  uv_fs_req_cleanup(&req);

  /* Pin the canonical mappings that Node.js' fs error formatter reads. */
  int failures = 0;
  failures += expect_name(UV_ENOENT, "ENOENT");
  failures += expect_name(UV_EACCES, "EACCES");
  failures += expect_name(UV_EEXIST, "EEXIST");
  failures += expect_name(UV_ENOTDIR, "ENOTDIR");
  failures += expect_name(UV_EISDIR, "EISDIR");
  failures += expect_name(UV_ELOOP, "ELOOP");
  failures += expect_name(UV_ENAMETOOLONG, "ENAMETOOLONG");
  failures += expect_name(UV_ENOTEMPTY, "ENOTEMPTY");
  failures += expect_name(UV_EBADF, "EBADF");
  failures += expect_name(UV_EXDEV, "EXDEV");
  failures += expect_name(UV_EAI_NONAME, "EAI_NONAME");

  failures += expect_strerror(UV_ENOENT, "no such file or directory");
  failures += expect_strerror(UV_EACCES, "permission denied");
  failures += expect_strerror(UV_EEXIST, "file already exists");
  failures += expect_strerror(UV_ENOTDIR, "not a directory");
  failures += expect_strerror(UV_EISDIR, "illegal operation on a directory");
  failures += expect_strerror(UV_ENOTEMPTY, "directory not empty");

  failures += expect_name_r(UV_ENOENT, "ENOENT");
  failures += expect_name_r(UV_EAI_NONAME, "EAI_NONAME");
  failures += expect_strerror_r(UV_ENOENT, "no such file or directory");
  failures += expect_strerror_r(UV_EAI_NONAME, "unknown node or service");

  /* Unknown codes must produce the upstream "Unknown system error %d" form
   * rather than a stale "Unknown error" placeholder. */
  const char* unknown = uv_strerror(-99999);
  if (unknown == NULL || strstr(unknown, "Unknown system error") == NULL) {
    fprintf(stderr,
            "uv_strerror(-99999) = %s; want substring 'Unknown system error'\n",
            unknown ? unknown : "(null)");
    failures++;
  }

  char ubuf[64];
  uv_strerror_r(-99999, ubuf, sizeof(ubuf));
  if (strstr(ubuf, "Unknown system error") == NULL ||
      strstr(ubuf, "-99999") == NULL) {
    fprintf(stderr,
            "uv_strerror_r(-99999) = %s; want 'Unknown system error -99999'\n",
            ubuf);
    failures++;
  }

  if (failures != 0) {
    fprintf(stderr, "%d error-mapping check(s) failed\n", failures);
    return 1;
  }
  return 0;
}
