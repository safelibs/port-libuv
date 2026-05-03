#include <uv.h>

#include <assert.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

struct timer_state {
  uv_timer_t handle;
  int id;
  int fired;
};

static int fire_log[8];
static int fire_count = 0;
static int close_count = 0;
static struct timer_state *repeat_target = NULL;
static int repeat_fires = 0;

static void on_close(uv_handle_t *handle) {
  (void)handle;
  close_count++;
}

static void on_timer(uv_timer_t *handle) {
  struct timer_state *state = (struct timer_state *) handle->data;
  state->fired++;
  fire_log[fire_count++] = state->id;
}

static void on_repeat(uv_timer_t *handle) {
  (void)handle;
  repeat_fires++;
  if (repeat_fires == 3) {
    int rc = uv_timer_stop(&repeat_target->handle);
    if (rc != 0) {
      fprintf(stderr, "uv_timer_stop in callback failed: %d\n", rc);
      exit(1);
    }
    uv_close((uv_handle_t *) &repeat_target->handle, on_close);
  }
}

int main(void) {
  uv_loop_t *loop = uv_default_loop();
  struct timer_state s10, s30, s60, s_stopped;
  int rc;

  memset(&s10, 0, sizeof(s10));
  memset(&s30, 0, sizeof(s30));
  memset(&s60, 0, sizeof(s60));
  memset(&s_stopped, 0, sizeof(s_stopped));

  s10.id = 10;
  s30.id = 30;
  s60.id = 60;
  s_stopped.id = 99;
  s10.handle.data = &s10;
  s30.handle.data = &s30;
  s60.handle.data = &s60;
  s_stopped.handle.data = &s_stopped;

  rc = uv_timer_init(loop, &s10.handle);
  assert(rc == 0);
  rc = uv_timer_init(loop, &s30.handle);
  assert(rc == 0);
  rc = uv_timer_init(loop, &s60.handle);
  assert(rc == 0);
  rc = uv_timer_init(loop, &s_stopped.handle);
  assert(rc == 0);

  /* Start in non-monotonic order to verify ordering by timeout. */
  rc = uv_timer_start(&s60.handle, on_timer, 60, 0);
  assert(rc == 0);
  rc = uv_timer_start(&s10.handle, on_timer, 10, 0);
  assert(rc == 0);
  rc = uv_timer_start(&s30.handle, on_timer, 30, 0);
  assert(rc == 0);

  /* This timer is started then stopped before run; it must never fire. */
  rc = uv_timer_start(&s_stopped.handle, on_timer, 5, 0);
  assert(rc == 0);
  rc = uv_timer_stop(&s_stopped.handle);
  assert(rc == 0);

  rc = uv_run(loop, UV_RUN_DEFAULT);
  if (rc != 0) {
    fprintf(stderr, "uv_run returned %d after one-shot timers\n", rc);
    return 1;
  }

  if (fire_count != 3) {
    fprintf(stderr, "expected 3 timer fires, got %d\n", fire_count);
    return 1;
  }
  if (fire_log[0] != 10 || fire_log[1] != 30 || fire_log[2] != 60) {
    fprintf(stderr,
            "timer fire order wrong: [%d, %d, %d]\n",
            fire_log[0], fire_log[1], fire_log[2]);
    return 1;
  }
  if (s_stopped.fired != 0) {
    fprintf(stderr, "stopped timer fired %d times\n", s_stopped.fired);
    return 1;
  }

  uv_close((uv_handle_t *) &s10.handle, on_close);
  uv_close((uv_handle_t *) &s30.handle, on_close);
  uv_close((uv_handle_t *) &s60.handle, on_close);
  uv_close((uv_handle_t *) &s_stopped.handle, on_close);
  rc = uv_run(loop, UV_RUN_DEFAULT);
  assert(rc == 0);
  if (close_count != 4) {
    fprintf(stderr, "expected 4 close callbacks, got %d\n", close_count);
    return 1;
  }

  /* Repeat timer: callback that stops + closes itself after N fires. */
  struct timer_state repeating;
  memset(&repeating, 0, sizeof(repeating));
  repeating.id = 77;
  repeating.handle.data = &repeating;
  repeat_target = &repeating;
  rc = uv_timer_init(loop, &repeating.handle);
  assert(rc == 0);
  rc = uv_timer_start(&repeating.handle, on_repeat, 5, 5);
  assert(rc == 0);

  uint64_t repeat_value = uv_timer_get_repeat(&repeating.handle);
  if (repeat_value != 5) {
    fprintf(stderr, "uv_timer_get_repeat returned %llu\n",
            (unsigned long long) repeat_value);
    return 1;
  }

  close_count = 0;
  rc = uv_run(loop, UV_RUN_DEFAULT);
  assert(rc == 0);
  if (repeat_fires != 3) {
    fprintf(stderr, "expected 3 repeat fires, got %d\n", repeat_fires);
    return 1;
  }
  if (close_count != 1) {
    fprintf(stderr, "expected 1 close cb after repeat, got %d\n", close_count);
    return 1;
  }

  /* After draining, the loop must report no active referenced handles. */
  if (uv_loop_alive(loop) != 0) {
    fprintf(stderr, "loop reports alive after timers closed\n");
    return 1;
  }

  rc = uv_loop_close(loop);
  if (rc != 0) {
    fprintf(stderr, "uv_loop_close returned %d\n", rc);
    return 1;
  }
  return 0;
}
