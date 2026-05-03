#include <uv.h>

#include <assert.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

struct rand_ctx {
  uv_random_t req;
  unsigned char buf[64];
  int status;
  int called;
};

static int async_seen = 0;

static int all_zero(const unsigned char *p, size_t n) {
  for (size_t i = 0; i < n; i++) {
    if (p[i] != 0) return 0;
  }
  return 1;
}

static void on_random(uv_random_t *req, int status, void *buf, size_t buflen) {
  struct rand_ctx *ctx = (struct rand_ctx *) req->data;
  ctx->status = status;
  ctx->called++;
  async_seen++;
  if (status != 0) {
    fprintf(stderr, "async uv_random status %d\n", status);
    exit(1);
  }
  if (buf != ctx->buf) {
    fprintf(stderr, "async uv_random buf pointer mismatch\n");
    exit(1);
  }
  if (buflen != sizeof(ctx->buf)) {
    fprintf(stderr,
            "async uv_random buflen %zu (expected %zu)\n",
            buflen, sizeof(ctx->buf));
    exit(1);
  }
}

int main(void) {
  uv_loop_t *loop = uv_default_loop();
  unsigned char sync_buf[64];
  int rc;

  /* Synchronous path: loop=NULL, req=NULL, cb=NULL must fill the buffer. */
  memset(sync_buf, 0, sizeof(sync_buf));
  rc = uv_random(NULL, NULL, sync_buf, sizeof(sync_buf), 0, NULL);
  if (rc != 0) {
    fprintf(stderr, "sync uv_random failed: %d\n", rc);
    return 1;
  }
  if (all_zero(sync_buf, sizeof(sync_buf))) {
    fprintf(stderr, "sync uv_random produced all zeros\n");
    return 1;
  }

  /* Reject unknown flags (libuv defines flags as reserved). */
  rc = uv_random(NULL, NULL, sync_buf, sizeof(sync_buf), 0xffff, NULL);
  if (rc != UV_EINVAL) {
    fprintf(stderr, "expected UV_EINVAL for bad flags, got %d\n", rc);
    return 1;
  }

  /* Asynchronous path: a request and a callback must be provided. */
  struct rand_ctx ctx;
  memset(&ctx, 0, sizeof(ctx));
  ctx.req.data = &ctx;
  rc = uv_random(loop, &ctx.req, ctx.buf, sizeof(ctx.buf), 0, on_random);
  if (rc != 0) {
    fprintf(stderr, "async uv_random submission failed: %d\n", rc);
    return 1;
  }

  rc = uv_run(loop, UV_RUN_DEFAULT);
  if (rc != 0) {
    fprintf(stderr, "uv_run returned %d after async uv_random\n", rc);
    return 1;
  }
  if (async_seen != 1) {
    fprintf(stderr, "async uv_random callback fired %d times\n", async_seen);
    return 1;
  }
  if (ctx.called != 1 || ctx.status != 0) {
    fprintf(stderr, "async ctx state called=%d status=%d\n",
            ctx.called, ctx.status);
    return 1;
  }
  if (all_zero(ctx.buf, sizeof(ctx.buf))) {
    fprintf(stderr, "async uv_random produced all zeros\n");
    return 1;
  }

  if (uv_loop_alive(loop) != 0) {
    fprintf(stderr, "loop alive after random drain\n");
    return 1;
  }
  rc = uv_loop_close(loop);
  if (rc != 0) {
    fprintf(stderr, "uv_loop_close returned %d\n", rc);
    return 1;
  }
  return 0;
}
