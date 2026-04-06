#ifndef _WIN32

#include <dlfcn.h>
#include <errno.h>
#include <grp.h>
#include <stddef.h>
#include <stdint.h>
#include <unistd.h>

struct uv_test_privdrop_call {
  int kind;
  int value0;
  int value1;
};

static int record_fd = -1;


void uv_test_privdrop_set_record_fd(int fd) {
  record_fd = fd;
}


void uv_test_privdrop_reset(void) {
  record_fd = -1;
}


static int write_call(int kind, int value0, int value1) {
  struct uv_test_privdrop_call call;
  ssize_t written;

  call.kind = kind;
  call.value0 = value0;
  call.value1 = value1;

  do
    written = write(record_fd, &call, sizeof(call));
  while (written == -1 && errno == EINTR);

  return written == (ssize_t) sizeof(call) ? 0 : -1;
}


static int (*real_setgroups_fn(void))(size_t, const gid_t*) {
  static int (*fn)(size_t, const gid_t*);

  if (fn == NULL)
    fn = (int (*)(size_t, const gid_t*)) dlsym(RTLD_NEXT, "setgroups");

  return fn;
}


static int (*real_setgid_fn(void))(gid_t) {
  static int (*fn)(gid_t);

  if (fn == NULL)
    fn = (int (*)(gid_t)) dlsym(RTLD_NEXT, "setgid");

  return fn;
}


static int (*real_setuid_fn(void))(uid_t) {
  static int (*fn)(uid_t);

  if (fn == NULL)
    fn = (int (*)(uid_t)) dlsym(RTLD_NEXT, "setuid");

  return fn;
}


int setgroups(size_t size, const gid_t* list) {
  if (record_fd == -1) {
    int (*fn)(size_t, const gid_t*);

    fn = real_setgroups_fn();
    return fn != NULL ? fn(size, list) : 0;
  }

  write_call('g', (int) size, list == NULL ? -1 : (int) list[0]);
  return 0;
}


int setgid(gid_t gid) {
  if (record_fd == -1) {
    int (*fn)(gid_t);

    fn = real_setgid_fn();
    return fn != NULL ? fn(gid) : 0;
  }

  write_call('G', (int) gid, 0);
  return 0;
}


int setuid(uid_t uid) {
  if (record_fd == -1) {
    int (*fn)(uid_t);

    fn = real_setuid_fn();
    return fn != NULL ? fn(uid) : 0;
  }

  write_call('U', (int) uid, 0);
  return 0;
}

#endif
