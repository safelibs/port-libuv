#!/usr/bin/env bash
set -euo pipefail

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
docker_image="${LIBUV_TEST_ORIGINAL_IMAGE:-ubuntu:24.04}"
docker_env=()

fail() {
  printf 'ERROR: %s\n' "$*" >&2
  exit 1
}

if [[ -n "${LIBUV_SAFE_DEB_DIR:-}" ]]; then
  if [[ "${LIBUV_SAFE_DEB_DIR}" = /* ]]; then
    safe_deb_dir_host="$(realpath "${LIBUV_SAFE_DEB_DIR}")"
  else
    safe_deb_dir_host="$(realpath "${repo_root}/${LIBUV_SAFE_DEB_DIR}")"
  fi

  [[ -d "${safe_deb_dir_host}" ]] || fail "LIBUV_SAFE_DEB_DIR does not exist: ${LIBUV_SAFE_DEB_DIR}"
  case "${safe_deb_dir_host}" in
    "${repo_root}"/*) ;;
    *) fail "LIBUV_SAFE_DEB_DIR must resolve inside ${repo_root}: ${safe_deb_dir_host}" ;;
  esac

  safe_deb_dir_repo_rel="${safe_deb_dir_host#${repo_root}/}"
  [[ -f "${safe_deb_dir_host}/artifacts.env" ]] || fail "missing artifacts manifest: ${safe_deb_dir_host}/artifacts.env"
  docker_env+=( -e "LIBUV_SAFE_DEB_DIR_REPO_REL=${safe_deb_dir_repo_rel}" )
fi

docker run --rm -i \
  --mount "type=bind,src=${repo_root},target=/work,readonly" \
  "${docker_env[@]}" \
  "${docker_image}" \
  bash -s -- <<'EOF'
set -euo pipefail

export DEBIAN_FRONTEND=noninteractive
safe_package_mode=0
expected_libuv_path=""
expected_libuv_realpath=""
libuv_ldd_env=()

note() {
  printf '\n==> %s\n' "$*"
}

fail() {
  printf 'ERROR: %s\n' "$*" >&2
  exit 1
}

resolve_target_libuv() {
  local target="$1"
  env "${libuv_ldd_env[@]}" ldd "${target}" | awk '/libuv\.so\.1/ { print $3; exit }'
}

load_expected_safe_libuv() {
  expected_libuv_path="$(dpkg -L libuv1t64 | awk '/\/libuv\.so\.1$/ { print; exit }')"
  [ -n "${expected_libuv_path}" ] || fail "could not find installed libuv.so.1 in libuv1t64"
  expected_libuv_realpath="$(realpath "${expected_libuv_path}")"
  dpkg -S "${expected_libuv_realpath}" 2>/dev/null | grep -q '^libuv1t64:' || \
    fail "installed libuv realpath is not owned by libuv1t64: ${expected_libuv_realpath}"
}

assert_uses_expected_libuv() {
  local target="$1"
  local resolved resolved_realpath

  resolved="$(resolve_target_libuv "${target}")"
  [ -n "${resolved}" ] || fail "could not resolve libuv for ${target}"
  resolved_realpath="$(realpath "${resolved}")"
  [ "${resolved_realpath}" = "${expected_libuv_realpath}" ] || \
    fail "${target} resolved libuv to ${resolved} (realpath ${resolved_realpath}), expected ${expected_libuv_path}"
  if (( safe_package_mode )); then
    dpkg -S "${resolved}" 2>/dev/null | grep -q '^libuv1t64:' || \
      dpkg -S "${resolved_realpath}" 2>/dev/null | grep -q '^libuv1t64:' || \
      fail "${target} resolved libuv to non-package-managed path ${resolved}"
  fi
}

configure_libuv_mode() {
  if [[ -n "${LIBUV_SAFE_DEB_DIR_REPO_REL:-}" ]]; then
    local manifest

    safe_package_mode=1
    manifest="/work/${LIBUV_SAFE_DEB_DIR_REPO_REL}/artifacts.env"
    [ -f "${manifest}" ] || fail "missing artifacts manifest in container: ${manifest}"
    # shellcheck disable=SC1090
    . "${manifest}"

    [ -n "${LIBUV_SAFE_RUNTIME_DEB_REPO_REL:-}" ] || fail "LIBUV_SAFE_RUNTIME_DEB_REPO_REL missing from ${manifest}"
    [ -n "${LIBUV_SAFE_DEV_DEB_REPO_REL:-}" ] || fail "LIBUV_SAFE_DEV_DEB_REPO_REL missing from ${manifest}"
    [ -f "/work/${LIBUV_SAFE_RUNTIME_DEB_REPO_REL}" ] || fail "runtime package missing: /work/${LIBUV_SAFE_RUNTIME_DEB_REPO_REL}"
    [ -f "/work/${LIBUV_SAFE_DEV_DEB_REPO_REL}" ] || fail "development package missing: /work/${LIBUV_SAFE_DEV_DEB_REPO_REL}"

    note "Installing locally built safe libuv packages"
    dpkg -i "/work/${LIBUV_SAFE_RUNTIME_DEB_REPO_REL}" "/work/${LIBUV_SAFE_DEV_DEB_REPO_REL}"
    load_expected_safe_libuv
    note "Expecting dependents to resolve ${expected_libuv_path}"
    return
  fi

  note "Building the original libuv"
  cmake -S /work/original -B /tmp/libuv-build \
    -DBUILD_TESTING=OFF \
    -DCMAKE_BUILD_TYPE=RelWithDebInfo \
    -DCMAKE_INSTALL_PREFIX=/opt/libuv-original
  cmake --build /tmp/libuv-build -j"$(nproc)"
  cmake --install /tmp/libuv-build

  export LD_LIBRARY_PATH="/opt/libuv-original/lib"
  export CPPFLAGS="-I/opt/libuv-original/include"
  export LDFLAGS="-L/opt/libuv-original/lib -Wl,-rpath,/opt/libuv-original/lib"
  libuv_ldd_env=( "LD_LIBRARY_PATH=${LD_LIBRARY_PATH}" )
  expected_libuv_path="/opt/libuv-original/lib/libuv.so.1"
  expected_libuv_realpath="$(realpath "${expected_libuv_path}")"
  note "Expecting dependents to resolve ${expected_libuv_path}"
}

python_module_file() {
  python3 - "$1" <<'PY'
import importlib
import sys

module = importlib.import_module(sys.argv[1])
print(module.__file__)
PY
}

r_package_shared_object() {
  Rscript - "$1" <<'RS'
args <- commandArgs(trailingOnly = TRUE)
pkg_dir <- system.file(package = args[1])
matches <- list.files(pkg_dir, pattern = "\\.so$", recursive = TRUE, full.names = TRUE)
if (!length(matches)) {
  quit(status = 1)
}
cat(matches[[1]])
RS
}

note "Enabling Ubuntu source repositories"
cat >/etc/apt/sources.list.d/ubuntu-src.sources <<'SRC'
Types: deb-src
URIs: http://archive.ubuntu.com/ubuntu
Suites: noble noble-updates noble-backports
Components: main universe multiverse restricted
Signed-By: /usr/share/keyrings/ubuntu-archive-keyring.gpg

Types: deb-src
URIs: http://security.ubuntu.com/ubuntu
Suites: noble-security
Components: main universe multiverse restricted
Signed-By: /usr/share/keyrings/ubuntu-archive-keyring.gpg
SRC

note "Installing test dependencies"
apt-get update
apt-get install -y --no-install-recommends \
  bind9 \
  build-essential \
  ca-certificates \
  cmake \
  curl \
  cython3 \
  dnsutils \
  dpkg-dev \
  h2o \
  knot-resolver \
  libc-ares-dev \
  libdqlite-dev \
  libevent-dev \
  libev-dev \
  libffi-dev \
  libh2o-dev \
  libluv-ocaml-dev \
  libraft-dev \
  libraft-tools \
  libstorj-dev \
  lua-luv \
  lua5.4 \
  moarvm \
  neovim \
  nodejs \
  ocaml \
  ocaml-findlib \
  pkg-config \
  python3 \
  python3-all-dev \
  python3-cffi \
  python3-greenlet \
  python3-setuptools \
  python3-uvloop \
  python3-zope.event \
  python3-zope.interface \
  r-base-core \
  r-cran-fs \
  r-cran-httpuv \
  rakudo \
  ttyd

note "Checking dependents.json matches the scripted coverage set"
python3 - <<'PY'
import json
from pathlib import Path

expected = {
    "BIND 9",
    "H2O libuv library",
    "Knot Resolver",
    "Luv for OCaml",
    "MoarVM",
    "Neovim",
    "Node.js",
    "R fs package",
    "R httpuv package",
    "dqlite",
    "gevent",
    "libraft",
    "libstorj",
    "luv for Lua",
    "ttyd",
    "uvloop",
}

data = json.loads(Path("/work/dependents.json").read_text())
actual = {item["software_name"] for item in data["dependents"]}
if data["selected_dependents_count"] != len(expected):
    raise SystemExit(f"selected_dependents_count={data['selected_dependents_count']} expected {len(expected)}")
if actual != expected:
    missing = sorted(expected - actual)
    extra = sorted(actual - expected)
    raise SystemExit(f"dependents.json mismatch; missing={missing} extra={extra}")
PY

configure_libuv_mode

test_nodejs() {
  note "Testing Node.js"
  assert_uses_expected_libuv /usr/bin/node
  node <<'NODE'
const fs = require("fs");
const net = require("net");
const os = require("os");
const path = require("path");

const file = path.join(os.tmpdir(), "libuv-node-test.txt");
fs.writeFileSync(file, "ok");

const server = net.createServer((socket) => socket.end("pong"));
server.listen(0, "127.0.0.1", () => {
  const { port } = server.address();
  const client = net.createConnection({ port, host: "127.0.0.1" });
  let data = "";
  client.on("data", (chunk) => { data += chunk; });
  client.on("end", () => {
    if (data !== "pong") {
      process.exit(2);
    }
    fs.readFile(file, "utf8", (err, text) => {
      if (err || text !== "ok") {
        process.exit(3);
      }
      setTimeout(() => server.close(() => process.exit(0)), 10);
    });
  });
});
NODE
}

test_neovim() {
  note "Testing Neovim"
  assert_uses_expected_libuv /usr/bin/nvim
  nvim --headless --clean \
    '+lua local uv=vim.uv or vim.loop; assert(uv); local fired=false; local timer=uv.new_timer(); timer:start(10,0,function() fired=true; timer:stop(); timer:close(); vim.schedule(function() assert(fired); vim.cmd("qall!") end) end)'
}

test_bind9() {
  note "Testing BIND 9"
  assert_uses_expected_libuv /usr/sbin/named
  (
    set -euo pipefail
    local dir
    dir="$(mktemp -d /tmp/bind-test.XXXXXX)"
    cat >"${dir}/named.conf" <<'CFG'
options {
  directory "__DIR__";
  listen-on port 5300 { 127.0.0.1; };
  pid-file "__DIR__/named.pid";
  recursion no;
  allow-query { any; };
  dnssec-validation no;
};
zone "example.test" IN {
  type master;
  file "__DIR__/db.example.test";
};
CFG
    sed -i "s#__DIR__#${dir}#g" "${dir}/named.conf"
    cat >"${dir}/db.example.test" <<'ZONE'
$TTL 300
@ IN SOA ns1.example.test. hostmaster.example.test. 1 300 300 300 300
@ IN NS ns1.example.test.
ns1 IN A 127.0.0.1
www IN A 127.0.0.42
ZONE
    named-checkconf "${dir}/named.conf"
    /usr/sbin/named -g -c "${dir}/named.conf" >"${dir}/named.log" 2>&1 &
    pid=$!
    trap 'kill "${pid}" 2>/dev/null || true; wait "${pid}" 2>/dev/null || true' EXIT
    for _ in $(seq 1 50); do
      if dig +short @127.0.0.1 -p 5300 www.example.test A >"${dir}/dig.out" 2>/dev/null && \
         grep -qx '127.0.0.42' "${dir}/dig.out"; then
        exit 0
      fi
      sleep 0.2
    done
    cat "${dir}/named.log"
    exit 1
  )
}

test_knot_resolver() {
  note "Testing Knot Resolver"
  assert_uses_expected_libuv /usr/sbin/kresd
  (
    set -euo pipefail
    local dir
    dir="$(mktemp -d /tmp/kres-test.XXXXXX)"
    cat >"${dir}/config" <<'CFG'
net.listen("127.0.0.1", 5301, { kind = "dns" })
modules = { "hints > iterate" }
cache.size = 8 * MB
CFG
    /usr/sbin/kresd -n -q -c "${dir}/config" "${dir}" >"${dir}/kres.log" 2>&1 &
    pid=$!
    trap 'kill "${pid}" 2>/dev/null || true; wait "${pid}" 2>/dev/null || true' EXIT
    for _ in $(seq 1 80); do
      if dig +time=1 +tries=1 +short @127.0.0.1 -p 5301 example.com A >"${dir}/dig.out" 2>/dev/null && \
         [ -s "${dir}/dig.out" ]; then
        exit 0
      fi
      sleep 0.25
    done
    cat "${dir}/kres.log"
    exit 1
  )
}

test_ttyd() {
  note "Testing ttyd"
  assert_uses_expected_libuv /usr/bin/ttyd
  (
    set -euo pipefail
    local dir
    dir="$(mktemp -d /tmp/ttyd-test.XXXXXX)"
    ttyd -p 7681 sh -lc 'printf ready; sleep 5' >"${dir}/ttyd.log" 2>&1 &
    pid=$!
    trap 'kill "${pid}" 2>/dev/null || true; wait "${pid}" 2>/dev/null || true' EXIT
    for _ in $(seq 1 50); do
      if curl -fsS http://127.0.0.1:7681/ >"${dir}/index.html" 2>/dev/null; then
        break
      fi
      sleep 0.2
    done
    grep -q 'ttyd' "${dir}/index.html"
  )
}

test_lua_luv() {
  local lua_luv_so
  note "Testing luv for Lua"
  lua_luv_so="/usr/lib/x86_64-linux-gnu/lua/5.4/luv.so"
  assert_uses_expected_libuv "${lua_luv_so}"
  lua5.4 - <<'LUA'
local uv = require("luv")
local fired = false
local timer = uv.new_timer()
timer:start(10, 0, function()
  fired = true
  timer:stop()
  timer:close()
end)
uv.run()
assert(fired)
LUA
}

test_luv_ocaml() {
  note "Testing Luv for OCaml"
  assert_uses_expected_libuv /usr/lib/ocaml/stublibs/dllluv_c_stubs.so
  cat >/tmp/luv_timer.ml <<'ML'
let () =
  let fired = ref false in
  match Luv.Timer.init () with
  | Error err -> failwith (Luv.Error.strerror err)
  | Ok timer ->
      begin
        match Luv.Timer.start timer 10 (fun () ->
          fired := true;
          ignore (Luv.Timer.stop timer);
          Luv.Handle.close timer ignore
        ) with
        | Error err -> failwith (Luv.Error.strerror err)
        | Ok () -> ()
      end;
      ignore (Luv.Loop.run ());
      if not !fired then failwith "timer did not fire"
ML
  ocamlfind ocamlopt -thread -package threads,luv -linkpkg \
    -o /tmp/luv_timer /tmp/luv_timer.ml
  /tmp/luv_timer
}

test_uvloop() {
  local uvloop_so
  note "Testing uvloop"
  uvloop_so="$(python_module_file uvloop.loop)"
  assert_uses_expected_libuv "${uvloop_so}"
  python3 - <<'PY'
import asyncio
import uvloop

async def main():
    server = await asyncio.start_server(lambda r, w: (w.write(b"pong"), w.close()), "127.0.0.1", 0)
    port = server.sockets[0].getsockname()[1]
    reader, writer = await asyncio.open_connection("127.0.0.1", port)
    data = await reader.read()
    assert data == b"pong"
    writer.close()
    await writer.wait_closed()
    server.close()
    await server.wait_closed()

uvloop.install()
asyncio.run(main())
PY
}

test_r_fs() {
  local fs_so
  note "Testing R fs package"
  fs_so="$(r_package_shared_object fs)"
  assert_uses_expected_libuv "${fs_so}"
  Rscript -e 'tmp <- tempfile(); fs::dir_create(tmp); f <- fs::path(tmp, "file.txt"); fs::file_create(f); stopifnot(fs::file_exists(f))'
}

test_r_httpuv() {
  local httpuv_so
  note "Testing R httpuv package"
  httpuv_so="$(r_package_shared_object httpuv)"
  assert_uses_expected_libuv "${httpuv_so}"
  Rscript - <<'RS'
port <- 8123L
outfile <- tempfile()
app <- list(call = function(req) {
  list(status = 200L, headers = list("Content-Type" = "text/plain"), body = "ok")
})
server <- httpuv::startServer("127.0.0.1", port, app)
on.exit({ try(server$stop(), silent = TRUE) }, add = TRUE)
system2("curl", c("-fsS", sprintf("http://127.0.0.1:%d/", port), "-o", outfile), wait = FALSE)
deadline <- Sys.time() + 5
while (Sys.time() < deadline && (!file.exists(outfile) || file.info(outfile)$size == 0)) {
  httpuv::service(100)
}
stopifnot(file.exists(outfile))
stopifnot(identical(readLines(outfile, warn = FALSE), "ok"))
RS
}

test_moarvm() {
  note "Testing MoarVM"
  assert_uses_expected_libuv /usr/bin/moar
  raku -e 'my $p = Promise.in(0.1).then({ say q[ok] }); await $p;' >/tmp/moar.out
  grep -qx 'ok' /tmp/moar.out
}

test_h2o() {
  note "Testing H2O libuv library"
  assert_uses_expected_libuv /usr/lib/x86_64-linux-gnu/libh2o.so
  (
    set -euo pipefail
    local dir
    dir="$(mktemp -d /tmp/h2o-test.XXXXXX)"
    mkdir -p "${dir}/root"
    printf 'hello\n' >"${dir}/root/index.txt"
    chmod 755 "${dir}" "${dir}/root"
    chmod 644 "${dir}/root/index.txt"
    cat >"${dir}/h2o.conf" <<CFG
server-name: "h2o-test"
listen:
  host: 127.0.0.1
  port: 8081
hosts:
  default:
    paths:
      /:
        file.dir: ${dir}/root
CFG
    h2o -c "${dir}/h2o.conf" >"${dir}/h2o.log" 2>&1 &
    pid=$!
    trap 'kill "${pid}" 2>/dev/null || true; wait "${pid}" 2>/dev/null || true' EXIT
    for _ in $(seq 1 50); do
      if curl -fsS http://127.0.0.1:8081/index.txt >"${dir}/out" 2>/dev/null && \
         grep -qx 'hello' "${dir}/out"; then
        exit 0
      fi
      sleep 0.2
    done
    cat "${dir}/h2o.log"
    exit 1
  )
}

test_libraft() {
  note "Testing libraft"
  assert_uses_expected_libuv /usr/bin/raft-benchmark
  mkdir -p /tmp/raft-submit
  raft-benchmark submit -d /tmp/raft-submit -s 65536 >/tmp/raft-submit.out
  grep -q '"submit:' /tmp/raft-submit.out
}

test_dqlite() {
  note "Testing dqlite"
  assert_uses_expected_libuv /usr/lib/x86_64-linux-gnu/libdqlite.so
  cat >/tmp/dqlite_smoke.c <<'C'
#include <dqlite.h>
#include <stdio.h>
#include <stdlib.h>
#include <unistd.h>

static void fail(dqlite_node *node, const char *msg) {
  if (node != NULL) {
    const char *err = dqlite_node_errmsg(node);
    fprintf(stderr, "%s: %s\n", msg, err ? err : "<no errmsg>");
  } else {
    fprintf(stderr, "%s\n", msg);
  }
  exit(1);
}

int main(void) {
  char template[] = "/tmp/dqlite-smoke-XXXXXX";
  char *dir = mkdtemp(template);
  dqlite_node *node = NULL;

  if (dir == NULL) fail(NULL, "mkdtemp failed");
  if (dqlite_node_create(1, "1", dir, &node) != 0) fail(node, "dqlite_node_create failed");
  if (dqlite_node_set_bind_address(node, "@123") != 0) fail(node, "dqlite_node_set_bind_address failed");
  if (dqlite_node_start(node) != 0) fail(node, "dqlite_node_start failed");
  if (dqlite_node_stop(node) != 0) fail(node, "dqlite_node_stop failed");
  dqlite_node_destroy(node);
  return 0;
}
C
  cc -o /tmp/dqlite_smoke /tmp/dqlite_smoke.c $(pkg-config --cflags --libs dqlite)
  /tmp/dqlite_smoke
}

test_libstorj() {
  note "Testing libstorj"
  assert_uses_expected_libuv /usr/lib/x86_64-linux-gnu/libstorj.so
  (
    set -euo pipefail
    local dir
    dir="$(mktemp -d /tmp/storj-test.XXXXXX)"
    cat >"${dir}/server.py" <<'PY'
from http.server import BaseHTTPRequestHandler, HTTPServer

class Handler(BaseHTTPRequestHandler):
    def do_GET(self):
        if self.path == "/":
            body = b'{"info":"ok"}'
            self.send_response(200)
            self.send_header("Content-Type", "application/json")
            self.send_header("Content-Length", str(len(body)))
            self.end_headers()
            self.wfile.write(body)
        else:
            self.send_error(404)

    def log_message(self, fmt, *args):
        pass

HTTPServer(("127.0.0.1", 8091), Handler).serve_forever()
PY
    python3 "${dir}/server.py" >"${dir}/server.log" 2>&1 &
    spid=$!
    trap 'kill "${spid}" 2>/dev/null || true; wait "${spid}" 2>/dev/null || true' EXIT
    for _ in $(seq 1 50); do
      if curl -fsS http://127.0.0.1:8091/ >/dev/null 2>&1; then
        break
      fi
      sleep 0.1
    done
    cat >/tmp/storj_smoke.c <<'C'
#include <storj.h>
#include <stdio.h>
#include <stdlib.h>

static uv_loop_t *g_loop = NULL;
static int g_status = 1;

static void after_info(uv_work_t *work, int status) {
  json_request_t *req = work->data;
  if (status == 0 && req != NULL && req->response != NULL) {
    struct json_object *value = NULL;
    if (json_object_object_get_ex(req->response, "info", &value)) {
      g_status = 0;
    }
  }
  if (req != NULL && req->response != NULL) {
    json_object_put(req->response);
  }
  free(req);
  free(work);
  if (g_loop != NULL) {
    uv_stop(g_loop);
  }
}

int main(void) {
  storj_bridge_options_t options = {
    .proto = "http",
    .host = "127.0.0.1",
    .port = 8091,
    .user = NULL,
    .pass = NULL,
  };
  storj_http_options_t http_options = {
    .user_agent = "storj-smoke",
    .proxy_url = NULL,
    .low_speed_limit = 0,
    .low_speed_time = 0,
    .timeout = 5,
  };
  storj_log_options_t log_options = {
    .logger = NULL,
    .level = 0,
  };
  storj_env_t *env = storj_init_env(&options, NULL, &http_options, &log_options);
  if (env == NULL) {
    fprintf(stderr, "storj_init_env failed\n");
    return 1;
  }
  g_loop = env->loop;
  if (storj_bridge_get_info(env, NULL, after_info) != 0) {
    fprintf(stderr, "storj_bridge_get_info failed\n");
    storj_destroy_env(env);
    return 1;
  }
  uv_run(env->loop, UV_RUN_DEFAULT);
  if (storj_destroy_env(env) != 0) {
    fprintf(stderr, "storj_destroy_env failed\n");
    return 1;
  }
  return g_status;
}
C
    cc -o /tmp/storj_smoke /tmp/storj_smoke.c $(pkg-config --cflags --libs libstorj)
    /tmp/storj_smoke
  )
}

test_gevent() {
  local build_root gevent_src gevent_libuv_so
  note "Testing gevent source build with libuv"
  build_root="$(mktemp -d /tmp/gevent-build.XXXXXX)"
  (
    set -euo pipefail
    cd "${build_root}"
    apt-get source python-gevent >/tmp/gevent-source.log 2>&1
    gevent_src="$(find . -maxdepth 1 -type d -name 'python-gevent-*' | head -n 1)"
    [ -n "${gevent_src}" ] || fail "could not extract python-gevent source"
    cd "${gevent_src}"
    gevent_build_env=(
      GEVENTSETUP_EMBED_CARES=0
      GEVENTSETUP_EMBED_LIBEV=0
      GEVENTSETUP_EMBED_LIBUV=0
    )
    if (( !safe_package_mode )); then
      gevent_build_env+=( "CPPFLAGS=${CPPFLAGS}" "LDFLAGS=${LDFLAGS}" )
    fi
    env "${gevent_build_env[@]}" python3 setup.py build_ext -i >/tmp/gevent-build.log 2>&1
    gevent_libuv_so="$(find src/gevent/libuv -maxdepth 1 -name '_corecffi*.so' | head -n 1)"
    [ -n "${gevent_libuv_so}" ] || fail "gevent libuv backend was not built"
    assert_uses_expected_libuv "${gevent_libuv_so}"
    PYTHONPATH="${PWD}/src" GEVENT_LOOP=libuv-cffi python3 - <<'PY'
import gevent
from gevent import sleep, spawn_later

flag = []
spawn_later(0.01, flag.append, 1)
sleep(0.05)
assert flag == [1]
print(type(gevent.get_hub().loop).__module__)
gevent.get_hub().destroy(destroy_loop=True)
PY
  )
}

test_nodejs
test_neovim
test_bind9
test_knot_resolver
test_ttyd
test_lua_luv
test_luv_ocaml
test_uvloop
test_r_fs
test_r_httpuv
test_moarvm
test_h2o
test_libraft
test_dqlite
test_libstorj
test_gevent

note "All dependent package checks passed"
EOF
