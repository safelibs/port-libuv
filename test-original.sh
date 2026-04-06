#!/usr/bin/env bash
set -euo pipefail

script_dir="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd)"
repo_root="$script_dir"

if ! command -v docker >/dev/null 2>&1; then
  printf 'docker is required\n' >&2
  exit 1
fi

docker run --rm -i \
  --pull=missing \
  -v "$repo_root:/work" \
  -w /work \
  ubuntu:24.04 \
  bash -seuo pipefail <<'CONTAINER_SCRIPT'
export DEBIAN_FRONTEND=noninteractive

printf '#!/bin/sh\nexit 101\n' >/usr/sbin/policy-rc.d
chmod +x /usr/sbin/policy-rc.d

apt-get update
apt-get install -y --no-install-recommends \
  bind9 \
  build-essential \
  ca-certificates \
  cmake \
  curl \
  dnsutils \
  jq \
  knot-resolver \
  libdqlite-dev \
  libh2o-dev \
  libluv-ocaml-dev \
  libraft-tools \
  libssl-dev \
  libtensorpipe-dev \
  libwebsockets-evlib-uv \
  lua-luv \
  lua5.4 \
  moarvm \
  neovim \
  ninja-build \
  nodejs \
  nqp \
  ocaml \
  ocaml-findlib \
  pkg-config \
  python3 \
  python3-uvloop \
  r-base-core \
  r-cran-fs \
  r-cran-httpuv \
  siridb-server \
  storj \
  ttyd

multiarch="$(gcc -print-multiarch)"
libuv_prefix="/usr/local"
libuv_pkgconfig="$libuv_prefix/lib/$multiarch/pkgconfig"
libuv_libdir="$libuv_prefix/lib/$multiarch"

cmake -S /work/original -B /tmp/libuv-build -G Ninja \
  -DCMAKE_BUILD_TYPE=Release \
  -DBUILD_TESTING=OFF \
  -DLIBUV_BUILD_TESTS=OFF \
  -DLIBUV_BUILD_BENCH=OFF
cmake --build /tmp/libuv-build -j"$(nproc)"
cmake --install /tmp/libuv-build --prefix "$libuv_prefix"
ldconfig

export PKG_CONFIG_PATH="$libuv_pkgconfig:$libuv_prefix/lib/pkgconfig${PKG_CONFIG_PATH:+:$PKG_CONFIG_PATH}"
export LD_LIBRARY_PATH="$libuv_libdir:$libuv_prefix/lib${LD_LIBRARY_PATH:+:$LD_LIBRARY_PATH}"

workdir="/tmp/libuv-dependent-tests"
rm -rf "$workdir"
mkdir -p "$workdir"

cleanup_pids=()

log() {
  printf '==> %s\n' "$*"
}

fail() {
  printf 'ERROR: %s\n' "$*" >&2
  exit 1
}

cleanup() {
  local pid
  for pid in "${cleanup_pids[@]:-}"; do
    kill "$pid" 2>/dev/null || true
    wait "$pid" 2>/dev/null || true
  done
}

trap cleanup EXIT

require_file_contains() {
  local file="$1"
  local needle="$2"
  grep -Fq -- "$needle" "$file" || fail "expected '$needle' in $file"
}

stop_pid() {
  local pid="$1"
  kill "$pid" 2>/dev/null || true
  wait "$pid" 2>/dev/null || true
}

wait_for_http() {
  local url="$1"
  local outfile="$2"
  local tries="$3"
  local i
  for ((i = 0; i < tries; i++)); do
    if curl -fsS "$url" >"$outfile" 2>/dev/null; then
      return 0
    fi
    sleep 0.5
  done
  return 1
}

test_bind9() {
  local dir="$workdir/bind9"
  local ans=""
  local i
  mkdir -p "$dir"
  cat >"$dir/named.conf" <<'EOF'
options {
  directory "__DIR__";
  listen-on port 15353 { 127.0.0.1; };
  listen-on-v6 { none; };
  pid-file "__DIR__/named.pid";
  recursion no;
  dnssec-validation no;
  allow-query { any; };
};
zone "example.test" {
  type primary;
  file "__DIR__/example.test.zone";
};
EOF
  sed -i "s#__DIR__#$dir#g" "$dir/named.conf"
  cat >"$dir/example.test.zone" <<'EOF'
$TTL 60
@ IN SOA ns1.example.test. hostmaster.example.test. (1 60 60 60 60)
@ IN NS ns1.example.test.
ns1 IN A 127.0.0.1
www IN A 127.0.0.2
EOF
  named -c "$dir/named.conf" -g >"$dir/named.log" 2>&1 &
  local pid=$!
  cleanup_pids+=("$pid")
  for ((i = 0; i < 20; i++)); do
    ans="$(dig @127.0.0.1 -p 15353 www.example.test A +short || true)"
    if [[ "$ans" == "127.0.0.2" ]]; then
      break
    fi
    sleep 0.5
  done
  stop_pid "$pid"
  [[ "$ans" == "127.0.0.2" ]] || fail "bind9 did not answer with the expected A record"
}

test_nodejs() {
  local dir="$workdir/nodejs"
  mkdir -p "$dir"
  cat >"$dir/test.js" <<'EOF'
const fs = require("fs/promises");
const net = require("net");

(async () => {
  const path = "/tmp/node-libuv-smoke.txt";
  await fs.writeFile(path, "node-ok\n");
  const content = await fs.readFile(path, "utf8");
  if (content.trim() !== "node-ok") {
    throw new Error("filesystem round-trip failed");
  }

  await new Promise((resolve, reject) => {
    const server = net.createServer((socket) => socket.end("pong"));
    server.on("error", reject);
    server.listen(0, "127.0.0.1", () => {
      const { port } = server.address();
      const client = net.createConnection({ host: "127.0.0.1", port }, () => {
        client.write("ping");
      });
      let data = "";
      client.on("data", (chunk) => {
        data += chunk;
      });
      client.on("end", () => {
        server.close((err) => {
          if (err) {
            reject(err);
          } else if (data === "pong") {
            resolve();
          } else {
            reject(new Error(`unexpected response: ${data}`));
          }
        });
      });
      client.on("error", reject);
    });
  });

  await new Promise((resolve) => setTimeout(resolve, 10));
})();
EOF
  node "$dir/test.js"
}

test_neovim() {
  local dir="$workdir/neovim"
  mkdir -p "$dir"
  export NVIM_UV_OUT="$dir/out.txt"
  cat >"$dir/test.lua" <<'EOF'
local uv = vim.loop
local out = os.getenv("NVIM_UV_OUT")
local timer = uv.new_timer()

timer:start(10, 0, function()
  timer:stop()
  timer:close()
  local fd = assert(uv.fs_open(out, "w", 420))
  assert(uv.fs_write(fd, "nvim-ok\n", -1))
  assert(uv.fs_close(fd))
  vim.schedule(function()
    vim.cmd("qa!")
  end)
end)
EOF
  nvim --headless -u NONE +"luafile $dir/test.lua" >"$dir/nvim.log" 2>&1
  require_file_contains "$NVIM_UV_OUT" "nvim-ok"
}

test_knot_resolver() {
  local dir="$workdir/knot-resolver"
  local ans=""
  local i
  mkdir -p "$dir"
  cat >"$dir/config" <<'EOF'
net.listen("127.0.0.1", 15354, { kind = "dns" })
modules = { "hints > iterate" }
cache.size = 10 * MB
EOF
  kresd -n -q -c "$dir/config" "$dir" >"$dir/kresd.log" 2>&1 &
  local pid=$!
  cleanup_pids+=("$pid")
  for ((i = 0; i < 20; i++)); do
    ans="$(dig @127.0.0.1 -p 15354 localhost A +short || true)"
    if [[ "$ans" == "127.0.0.1" ]]; then
      break
    fi
    sleep 0.5
  done
  stop_pid "$pid"
  [[ "$ans" == "127.0.0.1" ]] || fail "knot-resolver did not resolve localhost"
}

test_python3_uvloop() {
  python3 - <<'EOF'
import asyncio
import uvloop

asyncio.set_event_loop_policy(uvloop.EventLoopPolicy())

async def main() -> None:
    seen = []

    async def handler(reader, writer):
        data = await reader.read(16)
        seen.append(data)
        writer.write(b"pong")
        await writer.drain()
        writer.close()
        await writer.wait_closed()

    server = await asyncio.start_server(handler, "127.0.0.1", 0)
    port = server.sockets[0].getsockname()[1]
    reader, writer = await asyncio.open_connection("127.0.0.1", port)
    writer.write(b"ping")
    await writer.drain()
    data = await reader.read(16)
    writer.close()
    await writer.wait_closed()
    server.close()
    await server.wait_closed()
    assert seen == [b"ping"]
    assert data == b"pong"

asyncio.run(main())
EOF
}

test_r_cran_httpuv() {
  local dir="$workdir/r-cran-httpuv"
  local i
  mkdir -p "$dir"
  cat >"$dir/test.R" <<'EOF'
port <- 19091
app <- list(call = function(req) {
  list(
    status = 200L,
    headers = list("Content-Type" = "text/plain"),
    body = "httpuv-ok\n"
  )
})
server <- httpuv::startServer("127.0.0.1", port, app)
message("httpuv-ready")
repeat {
  httpuv::service(timeoutMs = 100)
}
EOF
  Rscript "$dir/test.R" >"$dir/httpuv.log" 2>&1 &
  local pid=$!
  cleanup_pids+=("$pid")
  for ((i = 0; i < 60; i++)); do
    if grep -Fq -- "httpuv-ready" "$dir/httpuv.log" 2>/dev/null; then
      break
    fi
    if ! kill -0 "$pid" 2>/dev/null; then
      cat "$dir/httpuv.log" >&2 || true
      fail "httpuv server exited before becoming ready"
    fi
    sleep 0.5
  done
  if ! grep -Fq -- "httpuv-ready" "$dir/httpuv.log" 2>/dev/null; then
    cat "$dir/httpuv.log" >&2 || true
    fail "httpuv server did not become ready"
  fi
  if ! wait_for_http "http://127.0.0.1:19091/" "$dir/out.txt" 60; then
    cat "$dir/httpuv.log" >&2 || true
    fail "httpuv did not answer HTTP request"
  fi
  stop_pid "$pid"
  require_file_contains "$dir/out.txt" "httpuv-ok"
}

test_r_cran_fs() {
  Rscript - <<'EOF'
dir <- tempfile("fs-test-")
fs::dir_create(dir)
path <- fs::path(dir, "value.txt")
fs::file_create(path)
writeLines("fs-ok", path)
copy <- fs::path(dir, "copy.txt")
fs::file_copy(path, copy)
stopifnot(readLines(copy) == "fs-ok")
stopifnot(length(fs::dir_ls(dir)) == 2)
EOF
}

test_moarvm() {
  local out
  out="$(nqp -e 'say("moarvm-ok")')"
  [[ "$out" == "moarvm-ok" ]] || fail "moarvm smoke test failed"
}

test_lua_luv() {
  local dir="$workdir/lua-luv"
  mkdir -p "$dir"
  cat >"$dir/test.lua" <<'EOF'
local uv = require("luv")
local out = "/tmp/lua-luv-smoke.txt"
local timer = uv.new_timer()

timer:start(10, 0, function()
  timer:stop()
  timer:close()
  local fd = assert(uv.fs_open(out, "w", 420))
  assert(uv.fs_write(fd, "lua-luv-ok\n", -1))
  assert(uv.fs_close(fd))
end)

uv.run()

local fd = assert(uv.fs_open(out, "r", 420))
local data = assert(uv.fs_read(fd, 64, 0))
assert(uv.fs_close(fd))
assert(data == "lua-luv-ok\n")
EOF
  lua "$dir/test.lua"
}

test_libluv_ocaml() {
  local dir="$workdir/libluv-ocaml"
  mkdir -p "$dir"
  cat >"$dir/test.ml" <<'EOF'
let unwrap = function
  | Ok value -> value
  | Error err -> failwith (Luv.Error.strerror err)

let fired = ref false

let () =
  let timer = unwrap (Luv.Timer.init ()) in
  ignore
    (unwrap
       (Luv.Timer.start timer 10 (fun () ->
            fired := true;
            Luv.Handle.close timer ignore)));
  ignore (Luv.Loop.run ());
  if not !fired then failwith "timer did not fire"
EOF
  ocamlfind ocamlopt -thread -package threads,luv -linkpkg -o "$dir/test" "$dir/test.ml"
  "$dir/test"
}

test_ttyd() {
  local dir="$workdir/ttyd"
  mkdir -p "$dir"
  ttyd -p 17681 sh -c 'printf ttyd-ok; sleep 5' >"$dir/ttyd.log" 2>&1 &
  local pid=$!
  cleanup_pids+=("$pid")
  wait_for_http "http://127.0.0.1:17681/" "$dir/index.html" 20 || fail "ttyd did not answer HTTP request"
  stop_pid "$pid"
  require_file_contains "$dir/index.html" "ttyd"
}

test_libh2o() {
  local dir="$workdir/libh2o0.13t64"
  mkdir -p "$dir"
  cat >"$dir/test.c" <<'EOF'
#include <assert.h>
#include <stdio.h>
#include <stdlib.h>
#include <h2o/socket.h>
#include <uv.h>

static uv_loop_t *g_loop;
static uv_tcp_t g_server;
static int g_ok = 0;
static int g_connected = 0;
static int g_accepted = 0;

static void on_handle_closed(uv_handle_t *handle)
{
    if (handle != (uv_handle_t *)&g_server)
        free(handle);
}

static void on_connect(h2o_socket_t *sock, const char *err)
{
    if (err == NULL) {
        g_ok = 1;
    } else {
        fprintf(stderr, "connect error: %s\n", err);
    }
    g_connected = 1;
    if (sock != NULL)
        h2o_socket_close(sock);
}

static void on_accept(uv_stream_t *server, int status)
{
    uv_tcp_t *client;

    assert(status == 0);
    client = malloc(sizeof(*client));
    assert(client != NULL);
    assert(uv_tcp_init(g_loop, client) == 0);
    assert(uv_accept(server, (uv_stream_t *)client) == 0);
    g_accepted = 1;
    uv_close((uv_handle_t *)client, on_handle_closed);
    uv_close((uv_handle_t *)server, on_handle_closed);
}

int main(void)
{
    struct sockaddr_in addr;
    int namelen;

    g_loop = malloc(sizeof(*g_loop));
    assert(g_loop != NULL);
    assert(uv_loop_init(g_loop) == 0);
    assert(uv_tcp_init(g_loop, &g_server) == 0);
    assert(uv_ip4_addr("127.0.0.1", 0, &addr) == 0);
    assert(uv_tcp_bind(&g_server, (const struct sockaddr *)&addr, 0) == 0);
    assert(uv_listen((uv_stream_t *)&g_server, 1, on_accept) == 0);

    namelen = sizeof(addr);
    assert(uv_tcp_getsockname(&g_server, (struct sockaddr *)&addr, &namelen) == 0);
    assert(h2o_socket_connect(g_loop, (struct sockaddr *)&addr, sizeof(addr), on_connect) != NULL);

    while (!g_connected || !g_accepted || uv_loop_alive(g_loop))
        uv_run(g_loop, UV_RUN_ONCE);

    assert(uv_loop_close(g_loop) == 0);
    free(g_loop);

    if (!g_ok)
        return 1;
    puts("libh2o-ok");
    return 0;
}
EOF
  cc -o "$dir/test" "$dir/test.c" $(pkg-config --cflags --libs libh2o libuv)
  ldd "$dir/test" >"$dir/ldd.txt"
  require_file_contains "$dir/ldd.txt" "libh2o.so.0.13"
  require_file_contains "$dir/ldd.txt" "libuv.so.1 => /usr/local/lib/libuv.so.1"
  "$dir/test" >"$dir/out.txt"
  require_file_contains "$dir/out.txt" "libh2o-ok"
}

test_libwebsockets_evlib_uv() {
  local dir="$workdir/libwebsockets-evlib-uv"
  mkdir -p "$dir"
  ttyd -p 17682 sh -c 'printf evlib-ok; sleep 5' >"$dir/ttyd.log" 2>&1 &
  local pid=$!
  cleanup_pids+=("$pid")
  wait_for_http "http://127.0.0.1:17682/" "$dir/index.html" 20 || fail "libwebsockets-evlib-uv dependent server did not answer HTTP request"
  stop_pid "$pid"
  require_file_contains "$dir/ttyd.log" "libwebsockets-evlib_uv.so"
  require_file_contains "$dir/ttyd.log" "Using foreign event loop"
}

test_libraft0() {
  local dir="$workdir/libraft0"
  mkdir -p "$dir"
  raft-benchmark submit -d "$dir" -s 8192 >"$dir/benchmark.log" 2>&1
}

test_libdqlite0() {
  local dir="$workdir/libdqlite0"
  mkdir -p "$dir/server"
  cat >"$dir/test.c" <<'EOF'
#include <dqlite.h>
#include <stdbool.h>
#include <stdio.h>
#include <stdlib.h>
#include <unistd.h>

static void check(int rc, const char *what) {
  if (rc != 0) {
    fprintf(stderr, "%s failed: %d\n", what, rc);
    exit(1);
  }
}

int main(void) {
  dqlite_server *server = NULL;
  check(dqlite_server_create("/tmp/libuv-dqlite-server", &server), "create");
  check(dqlite_server_set_address(server, "127.0.0.1:19040"), "set_address");
  check(dqlite_server_set_bind_address(server, "127.0.0.1:19040"), "set_bind_address");
  check(dqlite_server_set_auto_bootstrap(server, true), "set_auto_bootstrap");
  check(dqlite_server_start(server), "start");
  usleep(100000);
  check(dqlite_server_stop(server), "stop");
  dqlite_server_destroy(server);
  return 0;
}
EOF
  rm -rf /tmp/libuv-dqlite-server
  mkdir -p /tmp/libuv-dqlite-server
  cc -Wall -Wextra -o "$dir/test" "$dir/test.c" $(pkg-config --cflags --libs dqlite) -lsqlite3
  "$dir/test"
}

test_libtensorpipe0() {
  local dir="$workdir/libtensorpipe0"
  mkdir -p "$dir"
  cat >"$dir/test.cpp" <<'EOF'
#include <iostream>
#include <string>
#include <tensorpipe/tensorpipe.h>

int main() {
  auto transport = tensorpipe::transport::uv::create();
  if (!transport || !transport->isViable()) {
    std::cerr << "uv transport unavailable\n";
    return 1;
  }

  tensorpipe::Context context{tensorpipe::ContextOptions().name("libuv-test")};
  context.registerTransport(0, "uv", transport);
  auto listener = context.listen({"uv://127.0.0.1:0"});
  const auto url = listener->url("uv");
  if (url.rfind("uv://", 0) != 0) {
    std::cerr << "unexpected url: " << url << "\n";
    return 1;
  }

  listener->close();
  context.close();
  context.join();
  return 0;
}
EOF
  c++ -std=c++17 -Wall -Wextra -o "$dir/test" "$dir/test.cpp" -ltensorpipe -pthread
  "$dir/test"
}

test_storj() {
  local dir="$workdir/storj"
  mkdir -p "$dir"
  cat >"$dir/server.py" <<'EOF'
from http.server import BaseHTTPRequestHandler, HTTPServer
import json

class Handler(BaseHTTPRequestHandler):
    def do_GET(self):
        payload = {
            "title": "libuv-storj-smoke",
            "description": "local bridge stub",
            "version": "1.0",
            "host": "127.0.0.1",
        }
        self.send_response(200)
        self.send_header("Content-Type", "application/json")
        self.end_headers()
        self.wfile.write(json.dumps(payload).encode())

    def log_message(self, fmt, *args):
        pass

HTTPServer(("127.0.0.1", 19001), Handler).serve_forever()
EOF
  python3 "$dir/server.py" >"$dir/server.log" 2>&1 &
  local pid=$!
  cleanup_pids+=("$pid")
  sleep 1
  storj get-info -u http://127.0.0.1:19001 >"$dir/out.txt" 2>&1
  stop_pid "$pid"
  require_file_contains "$dir/out.txt" "Storj bridge: http://127.0.0.1:19001"
  require_file_contains "$dir/out.txt" "127.0.0.1"
}

test_siridb_server() {
  local dir="$workdir/siridb-server"
  mkdir -p "$dir/db"
  cp /etc/siridb/siridb.conf "$dir/siridb.conf"
  sed -i 's#server_name = .*#server_name = 127.0.0.1:19010#' "$dir/siridb.conf"
  sed -i 's#listen_client_port = .*#listen_client_port = 19000#' "$dir/siridb.conf"
  sed -i "s#db_path = .*#db_path = $dir/db#" "$dir/siridb.conf"
  sed -i 's#http_status_port = .*#http_status_port = 19080#' "$dir/siridb.conf"
  siridb-server -c "$dir/siridb.conf" >"$dir/server.log" 2>&1 &
  local pid=$!
  cleanup_pids+=("$pid")
  wait_for_http "http://127.0.0.1:19080/status" "$dir/status.txt" 20 || fail "siridb-server did not expose its HTTP status endpoint"
  stop_pid "$pid"
  require_file_contains "$dir/status.txt" "OK"
}

run_dependent_test() {
  local dep="$1"
  case "$dep" in
    bind9) test_bind9 ;;
    nodejs) test_nodejs ;;
    neovim) test_neovim ;;
    knot-resolver) test_knot_resolver ;;
    python3-uvloop) test_python3_uvloop ;;
    r-cran-httpuv) test_r_cran_httpuv ;;
    r-cran-fs) test_r_cran_fs ;;
    moarvm) test_moarvm ;;
    lua-luv) test_lua_luv ;;
    libluv-ocaml) test_libluv_ocaml ;;
    ttyd) test_ttyd ;;
    libh2o0.13t64) test_libh2o ;;
    libwebsockets-evlib-uv) test_libwebsockets_evlib_uv ;;
    libraft0) test_libraft0 ;;
    libdqlite0) test_libdqlite0 ;;
    libtensorpipe0) test_libtensorpipe0 ;;
    storj) test_storj ;;
    siridb-server) test_siridb_server ;;
    *) fail "no test implemented for dependent '$dep'" ;;
  esac
}

jq -e '.distribution | startswith("Ubuntu 24.04")' /work/dependents.json >/dev/null

mapfile -t dependents < <(jq -r '.dependents[].name' /work/dependents.json)

for dep in "${dependents[@]}"; do
  log "testing $dep"
  run_dependent_test "$dep"
done

log "all dependent smoke tests passed"
CONTAINER_SCRIPT
