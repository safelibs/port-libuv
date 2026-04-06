#ifndef _WIN32

#include <stdint.h>
#include <string.h>

#include "task.h"

struct uv_test_iou {
  uint32_t* sqhead;
  uint32_t* sqtail;
  uint32_t* sqarray;
  uint32_t sqmask;
  uint32_t* sqflags;
  uint32_t* cqhead;
  uint32_t* cqtail;
  uint32_t cqmask;
  void* sq;
  void* cqe;
  void* sqe;
  size_t sqlen;
  size_t cqlen;
  size_t maxlen;
  size_t sqelen;
  int ringfd;
  uint32_t in_flight;
  uint32_t flags;
};

struct uv_test_loop_metrics {
  uv_metrics_t metrics;
  uint64_t provider_entry_time;
  uint64_t provider_idle_time;
  uv_mutex_t lock;
};

struct uv_test_loop_internal_fields {
  unsigned int flags;
  struct uv_test_loop_metrics loop_metrics;
  int current_timeout;
  struct uv_test_iou ctl;
  struct uv_test_iou iou;
  void* inv;
};

static int fs_cb_called;


static void stat_cb(uv_fs_t* req) {
  ASSERT_GE(req->result, 0);
  fs_cb_called++;
  uv_fs_req_cleanup(req);
}


TEST_IMPL(io_uring_privdrop_regression) {
  uv_fs_t req;
  uv_loop_t loop;
  struct uv_test_loop_internal_fields* fields;

  memset(&req, 0, sizeof(req));

  ASSERT_OK(uv_loop_init(&loop));
  fields = loop.internal_fields;
  ASSERT_NOT_NULL(fields);
  ASSERT_EQ(-1, fields->ctl.ringfd);
  ASSERT_EQ(-1, fields->iou.ringfd);

  fs_cb_called = 0;
  ASSERT_OK(uv_fs_stat(&loop, &req, ".", stat_cb));
  ASSERT_OK(uv_run(&loop, UV_RUN_DEFAULT));
  ASSERT_EQ(1, fs_cb_called);
  ASSERT_EQ(-1, fields->ctl.ringfd);
  ASSERT_EQ(-1, fields->iou.ringfd);

  ASSERT_OK(uv_loop_close(&loop));
  return 0;
}

#endif
