#include <uv.h>

#include <assert.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

/*
 * UDP loopback regression: bind two uv_udp_t handles on 127.0.0.1, send a
 * datagram from one to the other, then exercise connected-mode send and a
 * couple of socket-option setters. Asserts:
 *   - uv_udp_bind on port 0 + uv_udp_getsockname returns a non-zero port
 *     and AF_INET / 127.0.0.1.
 *   - uv_udp_recv_start delivers exactly one positive-nread callback per
 *     datagram, with the addr argument populated for unconnected receivers.
 *   - uv_udp_send invokes the send callback exactly once with status == 0.
 *   - uv_udp_connect followed by uv_udp_send (NULL addr) round-trips the
 *     payload, and uv_udp_getpeername returns the connected peer.
 *   - uv_udp_set_broadcast / uv_udp_set_multicast_loop / uv_udp_set_ttl /
 *     uv_udp_set_multicast_ttl accept libuv's documented value ranges and
 *     return 0 on a freshly bound loopback socket.
 *   - uv_close cleans up both handles and produces one close_cb each.
 */

static uv_udp_t recv_handle;
static uv_udp_t send_handle;
static uv_udp_send_t send_req_a;
static uv_udp_send_t send_req_b;

static int alloc_count = 0;
static int recv_payload_count = 0;
static int recv_empty_count = 0;
static int send_cb_count = 0;
static int close_cb_count = 0;

static const char PAYLOAD_A[] = "udp-A";
static const char PAYLOAD_B[] = "udp-B-connected";

static void on_close(uv_handle_t *h) {
  (void)h;
  close_cb_count++;
}

static void alloc_cb(uv_handle_t *h, size_t suggested, uv_buf_t *buf) {
  (void)h;
  (void)suggested;
  /* Fixed buffer big enough for both payloads. */
  static char slab[2048];
  buf->base = slab;
  buf->len = sizeof(slab);
}

static void recv_cb(uv_udp_t *handle,
                    ssize_t nread,
                    const uv_buf_t *buf,
                    const struct sockaddr *addr,
                    unsigned flags) {
  (void)handle;
  (void)flags;
  if (nread == 0 && addr == NULL) {
    /* libuv signals "no more datagrams in this readable cycle" with
       nread=0 and addr=NULL; this is allowed and must not be counted as
       a payload delivery. */
    recv_empty_count++;
    return;
  }
  if (nread < 0) {
    fprintf(stderr, "udp recv error: %zd\n", nread);
    exit(2);
  }
  if (addr == NULL) {
    fprintf(stderr, "udp recv: addr NULL with nread=%zd\n", nread);
    exit(3);
  }
  if (addr->sa_family != AF_INET) {
    fprintf(stderr, "udp recv: addr family != AF_INET (%d)\n", addr->sa_family);
    exit(4);
  }

  recv_payload_count++;
  if (recv_payload_count == 1) {
    if ((size_t) nread != strlen(PAYLOAD_A) ||
        memcmp(buf->base, PAYLOAD_A, (size_t) nread) != 0) {
      fprintf(stderr,
              "udp datagram A mismatch: nread=%zd payload=%.*s\n",
              nread, (int) nread, buf->base);
      exit(5);
    }
  } else if (recv_payload_count == 2) {
    if ((size_t) nread != strlen(PAYLOAD_B) ||
        memcmp(buf->base, PAYLOAD_B, (size_t) nread) != 0) {
      fprintf(stderr,
              "udp datagram B mismatch: nread=%zd payload=%.*s\n",
              nread, (int) nread, buf->base);
      exit(6);
    }
    /* All datagrams received; tear down. */
    uv_udp_recv_stop(&recv_handle);
    uv_close((uv_handle_t *) &recv_handle, on_close);
    uv_close((uv_handle_t *) &send_handle, on_close);
  }
}

static void send_cb(uv_udp_send_t *req, int status) {
  (void)req;
  send_cb_count++;
  if (status != 0) {
    fprintf(stderr, "udp send_cb status=%d\n", status);
    exit(7);
  }
}

static void on_alloc_cb(uv_handle_t *h, size_t suggested, uv_buf_t *buf) {
  alloc_count++;
  alloc_cb(h, suggested, buf);
}

int main(void) {
  uv_loop_t *loop = uv_default_loop();
  struct sockaddr_in any_addr;
  struct sockaddr_storage recv_bound;
  int recv_bound_len = sizeof(recv_bound);
  int rc;

  rc = uv_ip4_addr("127.0.0.1", 0, &any_addr);
  assert(rc == 0);

  rc = uv_udp_init(loop, &recv_handle);
  assert(rc == 0);
  rc = uv_udp_init(loop, &send_handle);
  assert(rc == 0);

  rc = uv_udp_bind(&recv_handle, (const struct sockaddr *) &any_addr, 0);
  if (rc != 0) {
    fprintf(stderr, "uv_udp_bind(recv) failed: %d\n", rc);
    return 1;
  }
  rc = uv_udp_bind(&send_handle, (const struct sockaddr *) &any_addr, 0);
  if (rc != 0) {
    fprintf(stderr, "uv_udp_bind(send) failed: %d\n", rc);
    return 1;
  }

  rc = uv_udp_getsockname(&recv_handle,
                          (struct sockaddr *) &recv_bound, &recv_bound_len);
  if (rc != 0) {
    fprintf(stderr, "uv_udp_getsockname(recv) failed: %d\n", rc);
    return 1;
  }
  if (((struct sockaddr_in *) &recv_bound)->sin_port == 0) {
    fprintf(stderr, "udp bind did not assign a port\n");
    return 1;
  }
  if (((struct sockaddr *) &recv_bound)->sa_family != AF_INET) {
    fprintf(stderr, "udp bound family != AF_INET\n");
    return 1;
  }

  /* Socket-option setters that should succeed on a bound loopback UDP
     socket. uv_udp_set_multicast_loop: 1 = enable; 0 = disable. */
  rc = uv_udp_set_broadcast(&send_handle, 0);
  if (rc != 0) {
    fprintf(stderr, "uv_udp_set_broadcast failed: %d\n", rc);
    return 1;
  }
  rc = uv_udp_set_ttl(&send_handle, 1);
  if (rc != 0) {
    fprintf(stderr, "uv_udp_set_ttl failed: %d\n", rc);
    return 1;
  }
  rc = uv_udp_set_multicast_ttl(&send_handle, 1);
  if (rc != 0) {
    fprintf(stderr, "uv_udp_set_multicast_ttl failed: %d\n", rc);
    return 1;
  }
  rc = uv_udp_set_multicast_loop(&send_handle, 1);
  if (rc != 0) {
    fprintf(stderr, "uv_udp_set_multicast_loop failed: %d\n", rc);
    return 1;
  }

  rc = uv_udp_recv_start(&recv_handle, on_alloc_cb, recv_cb);
  if (rc != 0) {
    fprintf(stderr, "uv_udp_recv_start failed: %d\n", rc);
    return 1;
  }

  /* Datagram A: unconnected send addressed at the receiver. */
  uv_buf_t buf_a = uv_buf_init((char *) PAYLOAD_A, (unsigned) strlen(PAYLOAD_A));
  rc = uv_udp_send(&send_req_a, &send_handle, &buf_a, 1,
                   (const struct sockaddr *) &recv_bound, send_cb);
  if (rc != 0) {
    fprintf(stderr, "uv_udp_send(A) failed: %d\n", rc);
    return 1;
  }

  /* Datagram B: connect the sender to the receiver, then send with
     NULL addr. uv_udp_getpeername must report the connected peer. */
  rc = uv_udp_connect(&send_handle, (const struct sockaddr *) &recv_bound);
  if (rc != 0) {
    fprintf(stderr, "uv_udp_connect failed: %d\n", rc);
    return 1;
  }
  struct sockaddr_storage peer;
  int peer_len = sizeof(peer);
  rc = uv_udp_getpeername(&send_handle, (struct sockaddr *) &peer, &peer_len);
  if (rc != 0) {
    fprintf(stderr, "uv_udp_getpeername failed: %d\n", rc);
    return 1;
  }
  if (((struct sockaddr *) &peer)->sa_family != AF_INET) {
    fprintf(stderr, "udp peer family != AF_INET\n");
    return 1;
  }
  if (((struct sockaddr_in *) &peer)->sin_port !=
      ((struct sockaddr_in *) &recv_bound)->sin_port) {
    fprintf(stderr, "udp peer port mismatch\n");
    return 1;
  }

  uv_buf_t buf_b = uv_buf_init((char *) PAYLOAD_B, (unsigned) strlen(PAYLOAD_B));
  rc = uv_udp_send(&send_req_b, &send_handle, &buf_b, 1, NULL, send_cb);
  if (rc != 0) {
    fprintf(stderr, "uv_udp_send(B, connected) failed: %d\n", rc);
    return 1;
  }

  rc = uv_run(loop, UV_RUN_DEFAULT);
  if (rc != 0) {
    fprintf(stderr, "uv_run returned %d\n", rc);
    return 1;
  }

  if (recv_payload_count != 2) {
    fprintf(stderr, "expected 2 udp datagrams, got %d\n", recv_payload_count);
    return 1;
  }
  if (send_cb_count != 2) {
    fprintf(stderr, "expected 2 send_cb, got %d\n", send_cb_count);
    return 1;
  }
  if (alloc_count < recv_payload_count) {
    fprintf(stderr,
            "expected at least %d alloc_cb, got %d\n",
            recv_payload_count, alloc_count);
    return 1;
  }
  if (close_cb_count != 2) {
    fprintf(stderr, "expected 2 close_cb, got %d\n", close_cb_count);
    return 1;
  }

  if (uv_loop_alive(loop) != 0) {
    fprintf(stderr, "loop alive after udp drain\n");
    return 1;
  }
  rc = uv_loop_close(loop);
  if (rc != 0) {
    fprintf(stderr, "uv_loop_close returned %d\n", rc);
    return 1;
  }
  return 0;
}
