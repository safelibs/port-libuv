cmake_minimum_required(VERSION 3.20)

project(libuv_safe_upstream_tests LANGUAGES C)

enable_testing()

if(NOT CMAKE_SYSTEM_NAME STREQUAL "Linux")
  message(FATAL_ERROR "safe/tests/upstream currently supports Linux only")
endif()

set(CMAKE_C_STANDARD 90)
set(CMAKE_C_STANDARD_REQUIRED ON)
set(CMAKE_C_EXTENSIONS ON)

set(LIBUV_SAFE_STAGE "" CACHE PATH "Stage prefix created by safe/tools/stage_install.sh")

if(NOT LIBUV_SAFE_STAGE)
  message(FATAL_ERROR "Set LIBUV_SAFE_STAGE to a staged safe/libuv prefix")
endif()

set(STAGE_INCLUDE_DIR "${LIBUV_SAFE_STAGE}/include")
set(STAGE_LIB_DIR "${LIBUV_SAFE_STAGE}/lib")
set(STAGE_SHARED_LIB "${STAGE_LIB_DIR}/libuv.so")
set(STAGE_STATIC_LIB "${STAGE_LIB_DIR}/libuv.a")

foreach(required_path IN ITEMS "${STAGE_INCLUDE_DIR}/uv.h" "${STAGE_SHARED_LIB}" "${STAGE_STATIC_LIB}")
  if(NOT EXISTS "${required_path}")
    message(FATAL_ERROR "missing staged artifact: ${required_path}")
  endif()
endforeach()

find_package(Threads REQUIRED)

set(uv_defines
  _FILE_OFFSET_BITS=64
  _LARGEFILE_SOURCE
  _GNU_SOURCE
  _POSIX_C_SOURCE=200112
)

set(uv_test_libraries util)
set(uv_test_sources test/runner-unix.c)

set(uv_benchmark_sources
  ${uv_test_sources}
  test/benchmark-async-pummel.c
  test/benchmark-async.c
  test/benchmark-fs-stat.c
  test/benchmark-getaddrinfo.c
  test/benchmark-loop-count.c
  test/benchmark-queue-work.c
  test/benchmark-million-async.c
  test/benchmark-million-timers.c
  test/benchmark-multi-accept.c
  test/benchmark-ping-pongs.c
  test/benchmark-ping-udp.c
  test/benchmark-pound.c
  test/benchmark-pump.c
  test/benchmark-sizes.c
  test/benchmark-spawn.c
  test/benchmark-tcp-write-batch.c
  test/benchmark-thread.c
  test/benchmark-udp-pummel.c
  test/blackhole-server.c
  test/echo-server.c
  test/run-benchmarks.c
  test/runner.c
)

list(APPEND uv_test_sources
  test/blackhole-server.c
  test/echo-server.c
  test/run-tests.c
  test/runner.c
  test/test-active.c
  test/test-async-null-cb.c
  test/test-async.c
  test/test-barrier.c
  test/test-callback-stack.c
  test/test-close-fd.c
  test/test-close-order.c
  test/test-condvar.c
  test/test-connect-unspecified.c
  test/test-connection-fail.c
  test/test-cwd-and-chdir.c
  test/test-default-loop-close.c
  test/test-delayed-accept.c
  test/test-dlerror.c
  test/test-eintr-handling.c
  test/test-embed.c
  test/test-emfile.c
  test/test-env-vars.c
  test/test-error.c
  test/test-fail-always.c
  test/test-fork.c
  test/test-fs-copyfile.c
  test/test-fs-event.c
  test/test-fs-poll.c
  test/test-fs.c
  test/test-fs-readdir.c
  test/test-fs-fd-hash.c
  test/test-fs-open-flags.c
  test/test-get-currentexe.c
  test/test-get-loadavg.c
  test/test-get-memory.c
  test/test-get-passwd.c
  test/test-getaddrinfo.c
  test/test-gethostname.c
  test/test-getnameinfo.c
  test/test-getsockname.c
  test/test-getters-setters.c
  test/test-gettimeofday.c
  test/test-handle-fileno.c
  test/test-homedir.c
  test/test-hrtime.c
  test/test-idle.c
  test/test-idna.c
  test/test-ip4-addr.c
  test/test-ip6-addr.c
  test/test-ip-name.c
  test/test-ipc-heavy-traffic-deadlock-bug.c
  test/test-ipc-send-recv.c
  test/test-ipc.c
  test/test-loop-alive.c
  test/test-loop-close.c
  test/test-loop-configure.c
  test/test-loop-handles.c
  test/test-loop-stop.c
  test/test-loop-time.c
  test/test-metrics.c
  test/test-multiple-listen.c
  test/test-mutexes.c
  test/test-not-readable-nor-writable-on-read-error.c
  test/test-not-writable-after-shutdown.c
  test/test-osx-select.c
  test/test-pass-always.c
  test/test-ping-pong.c
  test/test-pipe-bind-error.c
  test/test-pipe-close-stdout-read-stdin.c
  test/test-pipe-connect-error.c
  test/test-pipe-connect-multiple.c
  test/test-pipe-connect-prepare.c
  test/test-pipe-getsockname.c
  test/test-pipe-pending-instances.c
  test/test-pipe-sendmsg.c
  test/test-pipe-server-close.c
  test/test-pipe-set-fchmod.c
  test/test-pipe-set-non-blocking.c
  test/test-platform-output.c
  test/test-poll-close-doesnt-corrupt-stack.c
  test/test-poll-close.c
  test/test-poll-closesocket.c
  test/test-poll-multiple-handles.c
  test/test-poll-oob.c
  test/test-poll.c
  test/test-process-priority.c
  test/test-process-title-threadsafe.c
  test/test-process-title.c
  test/test-queue-foreach-delete.c
  test/test-random.c
  test/test-readable-on-eof.c
  test/test-ref.c
  test/test-run-nowait.c
  test/test-run-once.c
  test/test-semaphore.c
  test/test-shutdown-close.c
  test/test-shutdown-eof.c
  test/test-shutdown-simultaneous.c
  test/test-shutdown-twice.c
  test/test-signal-multiple-loops.c
  test/test-signal-pending-on-close.c
  test/test-signal.c
  test/test-socket-buffer-size.c
  test/test-spawn.c
  test/test-stdio-over-pipes.c
  test/test-strscpy.c
  test/test-strtok.c
  test/test-tcp-alloc-cb-fail.c
  test/test-tcp-bind-error.c
  test/test-tcp-bind6-error.c
  test/test-tcp-close-accept.c
  test/test-tcp-close-after-read-timeout.c
  test/test-tcp-close-while-connecting.c
  test/test-tcp-close.c
  test/test-tcp-close-reset.c
  test/test-tcp-connect-error-after-write.c
  test/test-tcp-connect-error.c
  test/test-tcp-connect-timeout.c
  test/test-tcp-connect6-error.c
  test/test-tcp-create-socket-early.c
  test/test-tcp-flags.c
  test/test-tcp-oob.c
  test/test-tcp-open.c
  test/test-tcp-read-stop.c
  test/test-tcp-read-stop-start.c
  test/test-tcp-rst.c
  test/test-tcp-shutdown-after-write.c
  test/test-tcp-try-write.c
  test/test-tcp-write-in-a-row.c
  test/test-tcp-try-write-error.c
  test/test-tcp-unexpected-read.c
  test/test-tcp-write-after-connect.c
  test/test-tcp-write-fail.c
  test/test-tcp-write-queue-order.c
  test/test-tcp-write-to-half-open-connection.c
  test/test-tcp-writealot.c
  test/test-test-macros.c
  test/test-thread-affinity.c
  test/test-thread-equal.c
  test/test-thread.c
  test/test-thread-priority.c
  test/test-threadpool-cancel.c
  test/test-threadpool.c
  test/test-timer-again.c
  test/test-timer-from-check.c
  test/test-timer.c
  test/test-tmpdir.c
  test/test-tty-duplicate-key.c
  test/test-tty-escape-sequence-processing.c
  test/test-tty.c
  test/test-udp-alloc-cb-fail.c
  test/test-udp-bind.c
  test/test-udp-connect.c
  test/test-udp-connect6.c
  test/test-udp-create-socket-early.c
  test/test-udp-dgram-too-big.c
  test/test-udp-ipv6.c
  test/test-udp-mmsg.c
  test/test-udp-multicast-interface.c
  test/test-udp-multicast-interface6.c
  test/test-udp-multicast-join.c
  test/test-udp-multicast-join6.c
  test/test-udp-multicast-ttl.c
  test/test-udp-open.c
  test/test-udp-options.c
  test/test-udp-send-and-recv.c
  test/test-udp-send-hang-loop.c
  test/test-udp-send-immediate.c
  test/test-udp-sendmmsg-error.c
  test/test-udp-send-unreachable.c
  test/test-udp-try-send.c
  test/test-udp-recv-in-a-row.c
  test/test-uname.c
  test/test-walk-handles.c
  test/test-watcher-cross-stop.c
)

add_library(uv SHARED IMPORTED GLOBAL)
set_target_properties(uv PROPERTIES
  IMPORTED_LOCATION "${STAGE_SHARED_LIB}"
  INTERFACE_INCLUDE_DIRECTORIES "${STAGE_INCLUDE_DIR}"
  INTERFACE_LINK_LIBRARIES "Threads::Threads;dl;rt"
)

add_library(uv_a STATIC IMPORTED GLOBAL)
set_target_properties(uv_a PROPERTIES
  IMPORTED_LOCATION "${STAGE_STATIC_LIB}"
  INTERFACE_INCLUDE_DIRECTORIES "${STAGE_INCLUDE_DIR}"
  INTERFACE_LINK_LIBRARIES "Threads::Threads;dl;rt"
)

function(configure_upstream_target target)
  target_compile_definitions("${target}" PRIVATE ${ARGN})
  target_include_directories("${target}" PRIVATE "${STAGE_INCLUDE_DIR}" "${CMAKE_CURRENT_SOURCE_DIR}/test")
  target_compile_options("${target}" PRIVATE -fno-strict-aliasing)
endfunction()

add_executable(uv_run_benchmarks_a ${uv_benchmark_sources})
configure_upstream_target(uv_run_benchmarks_a ${uv_defines})
target_link_libraries(uv_run_benchmarks_a PRIVATE uv_a ${uv_test_libraries})

add_executable(uv_run_tests ${uv_test_sources})
configure_upstream_target(uv_run_tests ${uv_defines} USING_UV_SHARED=1)
target_link_libraries(uv_run_tests PRIVATE uv ${uv_test_libraries})
set_target_properties(uv_run_tests PROPERTIES BUILD_RPATH "${STAGE_LIB_DIR}")

add_test(NAME uv_test COMMAND uv_run_tests WORKING_DIRECTORY "${CMAKE_CURRENT_SOURCE_DIR}")

add_executable(uv_run_tests_a ${uv_test_sources})
configure_upstream_target(uv_run_tests_a ${uv_defines})
target_link_libraries(uv_run_tests_a PRIVATE uv_a ${uv_test_libraries})

add_test(NAME uv_test_a COMMAND uv_run_tests_a WORKING_DIRECTORY "${CMAKE_CURRENT_SOURCE_DIR}")

set(ported_test_environment
  "UV_TEST_TIMEOUT_MULTIPLIER=2"
  "RES_OPTIONS=attempts:0"
)

set_tests_properties(
  uv_test
  uv_test_a
  PROPERTIES
    RUN_SERIAL TRUE
    ENVIRONMENT "${ported_test_environment}"
)
