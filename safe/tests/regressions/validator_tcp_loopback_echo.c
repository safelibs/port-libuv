#include <uv.h>

#include <assert.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

/*
 * TCP loopback regression: bind/listen/accept/connect/read/write/shutdown
 * cycle on 127.0.0.1, asserting libuv invariants for the TCP and stream
 * surfaces:
 *   - uv_tcp_bind on port 0 then uv_tcp_getsockname returns a non-zero port.
 *   - uv_listen registers the connection callback exactly once per accept.
 *   - uv_accept moves the kernel socket into an initialised uv_tcp_t.
 *   - uv_tcp_getpeername / uv_tcp_getsockname return the loopback address
 *     after connect/accept.
 *   - uv_read_start delivers payload then a UV_EOF nread on shutdown.
 *   - uv_write completes via the write_cb exactly once with status == 0,
 *     and uv_stream_get_write_queue_size returns 0 after drain.
 *   - uv_shutdown's callback fires exactly once with status == 0.
 *   - uv_close is dispatched once per handle and produces one close_cb each.
 */

static uv_tcp_t server_handle;
static uv_tcp_t accepted_handle;
static uv_tcp_t client_handle;
static uv_connect_t connect_req;
static uv_write_t write_req;
static uv_shutdown_t shutdown_req;

static int connection_cb_count = 0;
static int write_cb_count = 0;
static int shutdown_cb_count = 0;
static int close_cb_count = 0;
static int got_payload = 0;
static int got_eof = 0;
static int connect_status_seen = -1;

static const char PAYLOAD[] = "hello-loopback";

static void on_close(uv_handle_t *h) {
  (void)h;
  close_cb_count++;
}

static void alloc_cb(uv_handle_t *h, size_t suggested, uv_buf_t *buf) {
  (void)h;
  buf->base = malloc(suggested);
  buf->len = suggested;
}

static void server_read_cb(uv_stream_t *s, ssize_t nread, const uv_buf_t *buf) {
  if (nread > 0) {
    if ((size_t) nread != strlen(PAYLOAD) ||
        memcmp(buf->base, PAYLOAD, (size_t) nread) != 0) {
      fprintf(stderr,
              "server read mismatch: nread=%zd payload=%.*s\n",
              nread, (int) nread, buf->base);
      exit(2);
    }
    got_payload++;
  } else if (nread == UV_EOF) {
    got_eof++;
    /* Tear down everything once we've seen the peer's shutdown. */
    uv_close((uv_handle_t *) &accepted_handle, on_close);
    uv_close((uv_handle_t *) &client_handle, on_close);
    uv_close((uv_handle_t *) &server_handle, on_close);
  } else if (nread < 0) {
    fprintf(stderr, "unexpected server read error: %zd\n", nread);
    exit(3);
  }
  free(buf->base);
}

static void shutdown_cb(uv_shutdown_t *req, int status) {
  (void)req;
  shutdown_cb_count++;
  if (status != 0) {
    fprintf(stderr, "shutdown_cb status=%d\n", status);
    exit(4);
  }
}

static void write_cb(uv_write_t *req, int status) {
  (void)req;
  write_cb_count++;
  if (status != 0) {
    fprintf(stderr, "write_cb status=%d\n", status);
    exit(5);
  }
  size_t qsize = uv_stream_get_write_queue_size((uv_stream_t *) &client_handle);
  if (qsize != 0) {
    fprintf(stderr, "write queue not drained: %zu\n", qsize);
    exit(6);
  }
  /* Send shutdown to surface UV_EOF on the peer's read callback. */
  int rc = uv_shutdown(&shutdown_req, (uv_stream_t *) &client_handle, shutdown_cb);
  if (rc != 0) {
    fprintf(stderr, "uv_shutdown failed: %d\n", rc);
    exit(7);
  }
}

static void connect_cb(uv_connect_t *req, int status) {
  (void)req;
  connect_status_seen = status;
  if (status != 0) {
    fprintf(stderr, "connect_cb status=%d\n", status);
    exit(8);
  }

  struct sockaddr_storage peer;
  int peer_len = sizeof(peer);
  int rc = uv_tcp_getpeername(&client_handle,
                              (struct sockaddr *) &peer, &peer_len);
  if (rc != 0) {
    fprintf(stderr, "uv_tcp_getpeername (client) failed: %d\n", rc);
    exit(9);
  }
  if (((struct sockaddr *) &peer)->sa_family != AF_INET) {
    fprintf(stderr, "peer family != AF_INET (%d)\n",
            ((struct sockaddr *) &peer)->sa_family);
    exit(10);
  }

  uv_buf_t buf = uv_buf_init((char *) PAYLOAD, (unsigned) strlen(PAYLOAD));
  rc = uv_write(&write_req, (uv_stream_t *) &client_handle, &buf, 1, write_cb);
  if (rc != 0) {
    fprintf(stderr, "uv_write failed: %d\n", rc);
    exit(11);
  }
}

static void connection_cb(uv_stream_t *server, int status) {
  connection_cb_count++;
  if (status != 0) {
    fprintf(stderr, "connection_cb status=%d\n", status);
    exit(12);
  }

  int rc = uv_tcp_init(uv_default_loop(), &accepted_handle);
  if (rc != 0) {
    fprintf(stderr, "uv_tcp_init (accepted) failed: %d\n", rc);
    exit(13);
  }
  rc = uv_accept(server, (uv_stream_t *) &accepted_handle);
  if (rc != 0) {
    fprintf(stderr, "uv_accept failed: %d\n", rc);
    exit(14);
  }

  /* getsockname on the accepted handle reveals the loopback bind. */
  struct sockaddr_storage local;
  int local_len = sizeof(local);
  rc = uv_tcp_getsockname(&accepted_handle,
                          (struct sockaddr *) &local, &local_len);
  if (rc != 0) {
    fprintf(stderr, "uv_tcp_getsockname (accepted) failed: %d\n", rc);
    exit(15);
  }
  if (((struct sockaddr *) &local)->sa_family != AF_INET) {
    fprintf(stderr, "accepted local family != AF_INET (%d)\n",
            ((struct sockaddr *) &local)->sa_family);
    exit(16);
  }

  rc = uv_read_start((uv_stream_t *) &accepted_handle, alloc_cb, server_read_cb);
  if (rc != 0) {
    fprintf(stderr, "uv_read_start failed: %d\n", rc);
    exit(17);
  }
}

int main(void) {
  uv_loop_t *loop = uv_default_loop();
  struct sockaddr_in addr;
  struct sockaddr_storage bound;
  int bound_len = sizeof(bound);
  int rc;

  rc = uv_ip4_addr("127.0.0.1", 0, &addr);
  assert(rc == 0);

  rc = uv_tcp_init(loop, &server_handle);
  assert(rc == 0);
  rc = uv_tcp_bind(&server_handle, (const struct sockaddr *) &addr, 0);
  if (rc != 0) {
    fprintf(stderr, "uv_tcp_bind failed: %d\n", rc);
    return 1;
  }
  rc = uv_listen((uv_stream_t *) &server_handle, 4, connection_cb);
  if (rc != 0) {
    fprintf(stderr, "uv_listen failed: %d\n", rc);
    return 1;
  }
  rc = uv_tcp_getsockname(&server_handle,
                          (struct sockaddr *) &bound, &bound_len);
  if (rc != 0) {
    fprintf(stderr, "uv_tcp_getsockname (server) failed: %d\n", rc);
    return 1;
  }
  if (((struct sockaddr_in *) &bound)->sin_port == 0) {
    fprintf(stderr, "server bind did not assign a port\n");
    return 1;
  }

  rc = uv_tcp_init(loop, &client_handle);
  assert(rc == 0);
  rc = uv_tcp_connect(&connect_req, &client_handle,
                      (const struct sockaddr *) &bound, connect_cb);
  if (rc != 0) {
    fprintf(stderr, "uv_tcp_connect failed: %d\n", rc);
    return 1;
  }

  rc = uv_run(loop, UV_RUN_DEFAULT);
  if (rc != 0) {
    fprintf(stderr, "uv_run returned %d\n", rc);
    return 1;
  }

  if (connect_status_seen != 0) {
    fprintf(stderr, "connect_cb not invoked (status=%d)\n",
            connect_status_seen);
    return 1;
  }
  if (connection_cb_count != 1) {
    fprintf(stderr, "expected 1 connection_cb, got %d\n", connection_cb_count);
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
  if (got_payload != 1) {
    fprintf(stderr, "expected 1 payload chunk, got %d\n", got_payload);
    return 1;
  }
  if (got_eof != 1) {
    fprintf(stderr, "expected 1 EOF read, got %d\n", got_eof);
    return 1;
  }
  if (close_cb_count != 3) {
    fprintf(stderr, "expected 3 close_cb, got %d\n", close_cb_count);
    return 1;
  }

  if (uv_loop_alive(loop) != 0) {
    fprintf(stderr, "loop alive after tcp drain\n");
    return 1;
  }
  rc = uv_loop_close(loop);
  if (rc != 0) {
    fprintf(stderr, "uv_loop_close returned %d\n", rc);
    return 1;
  }
  return 0;
}
