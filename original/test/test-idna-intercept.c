#ifndef _WIN32

#include <arpa/inet.h>
#include <dlfcn.h>
#include <netdb.h>
#include <stdlib.h>
#include <string.h>

#if defined(__GNUC__)
# define UV_TEST_IDNA_INTERCEPT __attribute__((visibility("default")))
#else
# define UV_TEST_IDNA_INTERCEPT
#endif


typedef int (*getaddrinfo_fn)(const char*,
                              const char*,
                              const struct addrinfo*,
                              struct addrinfo**);
typedef void (*freeaddrinfo_fn)(struct addrinfo*);


struct fake_result {
  struct fake_result* next;
  struct addrinfo ai;
  struct sockaddr_in addr;
};


struct fake_mapping {
  const char* node;
  const char* ip;
};


static const struct fake_mapping fake_mappings[] = {
  { "example.com.", "127.0.0.10" },
  { "xn--maana-pta.com", "127.0.0.11" },
  { "xn--fa-hia.de", "127.0.0.12" },
  { "xn--bcher-kva.com", "127.0.0.13" },
  { "xn--caf-dma.com", "127.0.0.14" },
  { "xn--caf-dma.xn--caf-dma.com", "127.0.0.15" },
  { "xn--strae-oqa.de", "127.0.0.16" }
};

static struct fake_result* fake_results;
static getaddrinfo_fn real_getaddrinfo;
static freeaddrinfo_fn real_freeaddrinfo;


static void init_real_symbols(void) {
  if (real_getaddrinfo == NULL)
    real_getaddrinfo = (getaddrinfo_fn) dlsym(RTLD_NEXT, "getaddrinfo");

  if (real_freeaddrinfo == NULL)
    real_freeaddrinfo =
        (freeaddrinfo_fn) dlsym(RTLD_NEXT, "freeaddrinfo");
}


static int make_fake_result(const char* ip,
                            const struct addrinfo* hints,
                            struct addrinfo** res) {
  struct fake_result* fake;

  if (hints != NULL &&
      hints->ai_family != AF_UNSPEC &&
      hints->ai_family != AF_INET) {
    return EAI_FAMILY;
  }

  fake = calloc(1, sizeof(*fake));
  if (fake == NULL)
    return EAI_MEMORY;

  fake->ai.ai_flags = hints != NULL ? hints->ai_flags : 0;
  fake->ai.ai_family = AF_INET;
  fake->ai.ai_socktype = hints != NULL ? hints->ai_socktype : 0;
  fake->ai.ai_protocol = hints != NULL ? hints->ai_protocol : 0;
  fake->ai.ai_addrlen = sizeof(fake->addr);
  fake->ai.ai_addr = (struct sockaddr*) &fake->addr;
  fake->ai.ai_canonname = NULL;
  fake->ai.ai_next = NULL;

  fake->addr.sin_family = AF_INET;
  fake->addr.sin_port = 0;
  if (inet_pton(AF_INET, ip, &fake->addr.sin_addr) != 1) {
    free(fake);
    return EAI_FAIL;
  }

  fake->next = fake_results;
  fake_results = fake;
  *res = &fake->ai;
  return 0;
}


UV_TEST_IDNA_INTERCEPT
int getaddrinfo(const char* node,
                const char* service,
                const struct addrinfo* hints,
                struct addrinfo** res) {
  size_t i;

  if (node != NULL && service == NULL) {
    for (i = 0; i < sizeof(fake_mappings) / sizeof(fake_mappings[0]); i++) {
      if (strcmp(node, fake_mappings[i].node) == 0)
        return make_fake_result(fake_mappings[i].ip, hints, res);
    }
  }

  init_real_symbols();
  if (real_getaddrinfo == NULL)
    return EAI_FAIL;

  return real_getaddrinfo(node, service, hints, res);
}


UV_TEST_IDNA_INTERCEPT
void freeaddrinfo(struct addrinfo* ai) {
  struct fake_result** current;

  if (ai == NULL)
    return;

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
  if (real_freeaddrinfo != NULL)
    real_freeaddrinfo(ai);
}

#endif
