#ifndef _WIN32

#include <arpa/inet.h>
#include <dlfcn.h>
#include <netdb.h>
#include <stddef.h>
#include <stdlib.h>
#include <string.h>

#include "task.h"

struct fake_result {
  struct fake_result* next;
  struct addrinfo ai;
  struct sockaddr_in addr;
};

static struct fake_result* fake_results;
static int (*real_getaddrinfo_fn)(const char*,
                                  const char*,
                                  const struct addrinfo*,
                                  struct addrinfo**);
static void (*real_freeaddrinfo_fn)(struct addrinfo*);
static char long_hostname[301];
static char truncated_hostname[256];
static int saw_full_hostname;
static int saw_truncated_hostname;


static void init_real_symbols(void) {
  if (real_getaddrinfo_fn == NULL)
    real_getaddrinfo_fn =
        (int (*)(const char*, const char*, const struct addrinfo*, struct addrinfo**))
            dlsym(RTLD_NEXT, "getaddrinfo");

  if (real_freeaddrinfo_fn == NULL)
    real_freeaddrinfo_fn = (void (*)(struct addrinfo*)) dlsym(RTLD_NEXT, "freeaddrinfo");
}


static void init_hostnames(void) {
  size_t i;

  for (i = 0; i < sizeof(long_hostname) - 1; i++)
    long_hostname[i] = 'a';
  long_hostname[sizeof(long_hostname) - 1] = '\0';

  memcpy(truncated_hostname, long_hostname, sizeof(truncated_hostname) - 1);
  truncated_hostname[sizeof(truncated_hostname) - 1] = '\0';
}


static int make_fake_result(const char* ip, struct addrinfo** res) {
  struct fake_result* fake;

  fake = calloc(1, sizeof(*fake));
  ASSERT_NOT_NULL(fake);

  fake->ai.ai_family = AF_INET;
  fake->ai.ai_addrlen = sizeof(fake->addr);
  fake->ai.ai_addr = (struct sockaddr*) &fake->addr;

  fake->addr.sin_family = AF_INET;
  ASSERT_EQ(1, inet_pton(AF_INET, ip, &fake->addr.sin_addr));

  fake->next = fake_results;
  fake_results = fake;
  *res = &fake->ai;
  return 0;
}


int getaddrinfo(const char* node,
                const char* service,
                const struct addrinfo* hints,
                struct addrinfo** res) {
  init_hostnames();

  if (node != NULL && service == NULL && hints == NULL) {
    if (strcmp(node, long_hostname) == 0) {
      saw_full_hostname++;
      return EAI_NONAME;
    }

    if (strcmp(node, truncated_hostname) == 0) {
      saw_truncated_hostname++;
      return make_fake_result("127.0.0.42", res);
    }
  }

  init_real_symbols();
  if (real_getaddrinfo_fn == NULL)
    return EAI_FAIL;

  return real_getaddrinfo_fn(node, service, hints, res);
}


void freeaddrinfo(struct addrinfo* ai) {
  struct fake_result** current;

  current = &fake_results;
  while (*current != NULL) {
    if (&(*current)->ai == ai) {
      struct fake_result* fake;

      fake = *current;
      *current = fake->next;
      free(fake);
      return;
    }

    current = &(*current)->next;
  }

  init_real_symbols();
  if (real_freeaddrinfo_fn != NULL)
    real_freeaddrinfo_fn(ai);
}


TEST_IMPL(getaddrinfo_long_hostname) {
  uv_getaddrinfo_t req;

  memset(&req, 0, sizeof(req));
  init_hostnames();
  saw_full_hostname = 0;
  saw_truncated_hostname = 0;

  ASSERT_EQ(UV__EAI_NONAME,
            uv_getaddrinfo(uv_default_loop(), &req, NULL, long_hostname, NULL, NULL));
  ASSERT_EQ(1, saw_full_hostname);
  ASSERT_OK(saw_truncated_hostname);
  ASSERT_NULL(req.addrinfo);

  MAKE_VALGRIND_HAPPY(uv_default_loop());
  return 0;
}

#endif
