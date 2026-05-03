#include <uv.h>

#include <assert.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <sys/socket.h>
#include <unistd.h>

/*
 * Pipe / stream regression: a SOCK_STREAM socketpair adopted by two
 * uv_pipe_t handles, exercising:
 *   - uv_pipe_init + uv_pipe_open take ownership of an existing fd.
 *   - uv_read_start on one end delivers payload then a UV_EOF nread once
 *     the peer shuts down.
 *   - uv_write completes via write_cb exactly once, leaving
 *     uv_stream_get_write_queue_size at 0.
 *   - uv_try_write succeeds on a fresh, drained stream and returns the
 *     number of bytes written (or UV_EAGAIN when not writable).
 *   - uv_shutdown's callback fires exactly once with status == 0.
 *   - uv_close is dispatched once per pipe and produces one close_cb each.
 */

static uv_pipe_t reader;
static uv_pipe_t writer;
static uv_write_t write_req;
static uv_shutdown_t shutdown_req;

static int read_cb_eof = 0;
static int write_cb_count = 0;
static int shutdown_cb_count = 0;
static int close_cb_count = 0;
static char read_accum[256];
static size_t read_accum_len = 0;
static size_t expected_total = 0;

static const char PAYLOAD[] = "pipe-payload-bytes";
static const char TRY_PAYLOAD[] = "try:";

static void on_close(uv_handle_t *h) {
  (void)h;
  close_cb_count++;
}

static void alloc_cb(uv_handle_t *h, size_t suggested, uv_buf_t *buf) {
  (void)h;
  buf->base = malloc(suggested);
  buf->len = suggested;
}

static void read_cb(uv_stream_t *s, ssize_t nread, const uv_buf_t *buf) {
  (void)s;
  if (nread > 0) {
    if (read_accum_len + (size_t) nread > sizeof(read_accum)) {
      fprintf(stderr,
              "pipe read overflow: have=%zu nread=%zd\n",
              read_accum_len, nread);
      exit(2);
    }
    memcpy(read_accum + read_accum_len, buf->base, (size_t) nread);
    read_accum_len += (size_t) nread;
  } else if (nread == UV_EOF) {
    read_cb_eof++;
    uv_close((uv_handle_t *) &reader, on_close);
    uv_close((uv_handle_t *) &writer, on_close);
  } else if (nread < 0) {
    fprintf(stderr, "unexpected pipe read error: %zd\n", nread);
    exit(3);
  }
  free(buf->base);
}

static void shutdown_cb(uv_shutdown_t *req, int status) {
  (void)req;
  shutdown_cb_count++;
  if (status != 0) {
    fprintf(stderr, "pipe shutdown_cb status=%d\n", status);
    exit(4);
  }
}

static void write_cb(uv_write_t *req, int status) {
  (void)req;
  write_cb_count++;
  if (status != 0) {
    fprintf(stderr, "pipe write_cb status=%d\n", status);
    exit(5);
  }
  size_t qsize = uv_stream_get_write_queue_size((uv_stream_t *) &writer);
  if (qsize != 0) {
    fprintf(stderr, "pipe write queue not drained: %zu\n", qsize);
    exit(6);
  }
  int rc = uv_shutdown(&shutdown_req, (uv_stream_t *) &writer, shutdown_cb);
  if (rc != 0) {
    fprintf(stderr, "uv_shutdown failed: %d\n", rc);
    exit(7);
  }
}

int main(void) {
  uv_loop_t *loop = uv_default_loop();
  int sv[2];
  int rc;

  if (socketpair(AF_UNIX, SOCK_STREAM, 0, sv) != 0) {
    perror("socketpair");
    return 1;
  }

  rc = uv_pipe_init(loop, &reader, 0);
  assert(rc == 0);
  rc = uv_pipe_init(loop, &writer, 0);
  assert(rc == 0);

  rc = uv_pipe_open(&reader, sv[0]);
  if (rc != 0) {
    fprintf(stderr, "uv_pipe_open(reader) failed: %d\n", rc);
    return 1;
  }
  rc = uv_pipe_open(&writer, sv[1]);
  if (rc != 0) {
    fprintf(stderr, "uv_pipe_open(writer) failed: %d\n", rc);
    return 1;
  }

  rc = uv_read_start((uv_stream_t *) &reader, alloc_cb, read_cb);
  if (rc != 0) {
    fprintf(stderr, "uv_read_start failed: %d\n", rc);
    return 1;
  }

  /* uv_try_write must succeed on a fresh, drained stream. The kernel
     socketpair buffer easily holds a small payload, so all bytes are
     accepted in one call. */
  uv_buf_t try_buf = uv_buf_init((char *) TRY_PAYLOAD,
                                 (unsigned) strlen(TRY_PAYLOAD));
  int wrote = uv_try_write((uv_stream_t *) &writer, &try_buf, 1);
  if (wrote < 0 && wrote != UV_EAGAIN) {
    fprintf(stderr, "uv_try_write returned %d\n", wrote);
    return 1;
  }
  if (wrote > 0) {
    /* The reader will see the partial "try:" prefix in addition to the
       full PAYLOAD that uv_write below sends. Track the expected byte
       count so the final accumulator check matches. */
    expected_total += (size_t) wrote;
  }
  expected_total += strlen(PAYLOAD);

  uv_buf_t buf = uv_buf_init((char *) PAYLOAD, (unsigned) strlen(PAYLOAD));
  rc = uv_write(&write_req, (uv_stream_t *) &writer, &buf, 1, write_cb);
  if (rc != 0) {
    fprintf(stderr, "uv_write failed: %d\n", rc);
    return 1;
  }

  rc = uv_run(loop, UV_RUN_DEFAULT);
  if (rc != 0) {
    fprintf(stderr, "uv_run returned %d\n", rc);
    return 1;
  }

  if (read_accum_len != expected_total) {
    fprintf(stderr, "expected %zu payload bytes, accumulated %zu\n",
            expected_total, read_accum_len);
    return 1;
  }
  /* Validate the accumulated stream is exactly the bytes uv_try_write
     accepted (a prefix of TRY_PAYLOAD) followed by PAYLOAD. */
  size_t try_prefix = expected_total - strlen(PAYLOAD);
  if (memcmp(read_accum, TRY_PAYLOAD, try_prefix) != 0 ||
      memcmp(read_accum + try_prefix, PAYLOAD, strlen(PAYLOAD)) != 0) {
    fprintf(stderr,
            "pipe stream content mismatch: '%.*s'\n",
            (int) read_accum_len, read_accum);
    return 1;
  }
  if (read_cb_eof != 1) {
    fprintf(stderr, "expected 1 EOF, got %d\n", read_cb_eof);
    return 1;
  }
  if (write_cb_count != 1) {
    fprintf(stderr, "expected 1 write_cb, got %d\n", write_cb_count);
    return 1;
  }
  if (shutdown_cb_count != 1) {
    fprintf(stderr, "expected 1 shutdown_cb, got %d\n", shutdown_cb_count);
    return 1;
  }
  if (close_cb_count != 2) {
    fprintf(stderr, "expected 2 close_cb, got %d\n", close_cb_count);
    return 1;
  }

  if (uv_loop_alive(loop) != 0) {
    fprintf(stderr, "loop alive after pipe drain\n");
    return 1;
  }
  rc = uv_loop_close(loop);
  if (rc != 0) {
    fprintf(stderr, "uv_loop_close returned %d\n", rc);
    return 1;
  }
  return 0;
}
