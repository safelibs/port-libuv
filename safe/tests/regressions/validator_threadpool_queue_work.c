#include <uv.h>

#include <assert.h>
#include <stdio.h>
#include <string.h>

struct work {
  uv_work_t req;
  int id;
  int worked;
  int after_status;
  int after_called;
};

static int total_after = 0;

static void worker(uv_work_t *req) {
  struct work *w = (struct work *) req->data;
  w->worked++;
  /* Mild busy work to ensure the request crosses the threadpool boundary. */
  volatile int sink = 0;
  for (int i = 0; i < 1000; i++) sink += i;
  (void)sink;
}

static void after_work(uv_work_t *req, int status) {
  struct work *w = (struct work *) req->data;
  w->after_status = status;
  w->after_called++;
  total_after++;
}

int main(void) {
  uv_loop_t *loop = uv_default_loop();
  struct work jobs[4];
  int rc;

  memset(jobs, 0, sizeof(jobs));
  for (int i = 0; i < 4; i++) {
    jobs[i].id = i;
    jobs[i].req.data = &jobs[i];
    rc = uv_queue_work(loop, &jobs[i].req, worker, after_work);
    if (rc != 0) {
      fprintf(stderr, "uv_queue_work[%d] failed: %d\n", i, rc);
      return 1;
    }
  }

  rc = uv_run(loop, UV_RUN_DEFAULT);
  if (rc != 0) {
    fprintf(stderr, "uv_run returned %d after threadpool batch\n", rc);
    return 1;
  }

  if (total_after != 4) {
    fprintf(stderr, "expected 4 after-work callbacks, got %d\n", total_after);
    return 1;
  }
  for (int i = 0; i < 4; i++) {
    if (jobs[i].worked != 1) {
      fprintf(stderr, "job %d worker ran %d times\n", i, jobs[i].worked);
      return 1;
    }
    if (jobs[i].after_called != 1) {
      fprintf(stderr,
              "job %d after-cb called %d times\n",
              i, jobs[i].after_called);
      return 1;
    }
    if (jobs[i].after_status != 0) {
      fprintf(stderr,
              "job %d after-cb status %d\n",
              i, jobs[i].after_status);
      return 1;
    }
  }

  /* Cancel path: queue many jobs and cancel. Cancellation may race with
     dispatch, but each request must produce exactly one after-work callback,
     and a cancelled callback must report UV_ECANCELED. */
  enum { N = 16 };
  struct work bulk[N];
  memset(bulk, 0, sizeof(bulk));
  total_after = 0;
  for (int i = 0; i < N; i++) {
    bulk[i].id = i;
    bulk[i].req.data = &bulk[i];
    rc = uv_queue_work(loop, &bulk[i].req, worker, after_work);
    if (rc != 0) {
      fprintf(stderr, "bulk uv_queue_work[%d] failed: %d\n", i, rc);
      return 1;
    }
  }

  /* Cancel half of them. Cancellation may fail (work already in flight)
     but must not produce duplicate callbacks. */
  for (int i = 0; i < N; i += 2) {
    int cr = uv_cancel((uv_req_t *) &bulk[i].req);
    if (cr != 0 && cr != UV_EBUSY) {
      fprintf(stderr, "uv_cancel[%d] returned %d\n", i, cr);
      return 1;
    }
  }

  rc = uv_run(loop, UV_RUN_DEFAULT);
  if (rc != 0) {
    fprintf(stderr, "uv_run returned %d after cancel batch\n", rc);
    return 1;
  }
  if (total_after != N) {
    fprintf(stderr, "expected %d after-work cbs after cancel, got %d\n",
            N, total_after);
    return 1;
  }
  for (int i = 0; i < N; i++) {
    if (bulk[i].after_called != 1) {
      fprintf(stderr,
              "bulk job %d after-cb called %d times\n",
              i, bulk[i].after_called);
      return 1;
    }
    if (bulk[i].after_status != 0 && bulk[i].after_status != UV_ECANCELED) {
      fprintf(stderr,
              "bulk job %d unexpected status %d\n",
              i, bulk[i].after_status);
      return 1;
    }
    if (bulk[i].after_status == UV_ECANCELED && bulk[i].worked != 0) {
      fprintf(stderr,
              "bulk job %d cancelled but worker ran %d times\n",
              i, bulk[i].worked);
      return 1;
    }
  }

  if (uv_loop_alive(loop) != 0) {
    fprintf(stderr, "loop alive after threadpool drain\n");
    return 1;
  }
  rc = uv_loop_close(loop);
  if (rc != 0) {
    fprintf(stderr, "uv_loop_close returned %d\n", rc);
    return 1;
  }
  return 0;
}
