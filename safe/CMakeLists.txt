cmake_minimum_required(VERSION 3.28)

project(libuv_safe VERSION 1.48.0 LANGUAGES C)

include(GNUInstallDirs)
include(CTest)

set(CMAKE_C_STANDARD 90)
set(CMAKE_C_STANDARD_REQUIRED ON)
set(CMAKE_C_EXTENSIONS ON)
set(CMAKE_C_VISIBILITY_PRESET hidden)
set(CMAKE_EXPORT_COMPILE_COMMANDS ON)

set(uv_defines
    _FILE_OFFSET_BITS=64
    _GNU_SOURCE
    _LARGEFILE_SOURCE
    _POSIX_C_SOURCE=200112)
set(uv_cflags -fno-strict-aliasing)
set(uv_libraries pthread dl rt)
set(uv_test_libraries util)

find_program(PATCHELF_EXECUTABLE patchelf REQUIRED)

set(CARGO_TARGET_DIR ${CMAKE_BINARY_DIR}/cargo-target)
if(CMAKE_BUILD_TYPE STREQUAL "Debug")
  set(CARGO_PROFILE debug)
  set(CARGO_BUILD_ARGS)
else()
  set(CARGO_PROFILE release)
  set(CARGO_BUILD_ARGS --release)
endif()

set(CARGO_SHARED_LIBRARY ${CARGO_TARGET_DIR}/${CARGO_PROFILE}/libuv.so)
set(CARGO_STATIC_LIBRARY ${CARGO_TARGET_DIR}/${CARGO_PROFILE}/libuv.a)
set(STAGED_SHARED_LIBRARY ${CMAKE_BINARY_DIR}/libuv.so.1.0.0)
set(STAGED_SHARED_LIBRARY_SONAME ${CMAKE_BINARY_DIR}/libuv.so.1)
set(STAGED_SHARED_LIBRARY_LINK ${CMAKE_BINARY_DIR}/libuv.so)
set(STAGED_STATIC_LIBRARY ${CMAKE_BINARY_DIR}/libuv.a)
set(STAGED_BUILD_MANIFEST ${CMAKE_BINARY_DIR}/libuv-build-manifest.json)
set(PACKAGE_VERSION ${PROJECT_VERSION})
set(prefix ${CMAKE_INSTALL_PREFIX})
set(libdir ${CMAKE_INSTALL_PREFIX}/${CMAKE_INSTALL_LIBDIR})
set(includedir ${CMAKE_INSTALL_PREFIX}/${CMAKE_INSTALL_INCLUDEDIR})

file(MAKE_DIRECTORY ${CMAKE_BINARY_DIR}/include)

file(GLOB_RECURSE UV_CARGO_INPUTS CONFIGURE_DEPENDS
  ${PROJECT_SOURCE_DIR}/Cargo.toml
  ${PROJECT_SOURCE_DIR}/build.rs
  ${PROJECT_SOURCE_DIR}/src/*.rs
  ${PROJECT_SOURCE_DIR}/include/*
  ${PROJECT_SOURCE_DIR}/abi/*
  ${PROJECT_SOURCE_DIR}/legacy/*
)
file(GLOB_RECURSE UV_LEGACY_INPUTS CONFIGURE_DEPENDS
  ${PROJECT_SOURCE_DIR}/../original/src/*
)

configure_file(${PROJECT_SOURCE_DIR}/libuv.pc.in ${CMAKE_BINARY_DIR}/libuv.pc @ONLY)
configure_file(${PROJECT_SOURCE_DIR}/libuv-static.pc.in ${CMAKE_BINARY_DIR}/libuv-static.pc @ONLY)
configure_file(
  ${PROJECT_SOURCE_DIR}/cmake/libuvConfig.cmake.in
  ${CMAKE_BINARY_DIR}/libuvConfig.cmake
  @ONLY
)

set(uv_test_legacy_support_sources
    ../original/src/idna.c
    ../original/src/random.c
    ../original/src/threadpool.c
    ../original/src/uv-common.c
    ../original/src/unix/fs.c
    ../original/src/unix/getaddrinfo.c
    ../original/src/unix/getnameinfo.c
    ../original/src/unix/linux.c
    ../original/src/unix/process.c
    ../original/src/unix/random-devurandom.c
    ../original/src/unix/random-getrandom.c
    ../original/src/unix/random-sysctl-linux.c
)

set(uv_test_phase5_private_support_sources
    ../original/src/fs-poll.c
    ../original/src/unix/core.c
    ../original/src/unix/pipe.c
    ../original/src/unix/poll.c
    ../original/src/unix/signal.c
    ../original/src/unix/stream.c
    ../original/src/unix/tcp.c
    ../original/src/unix/tty.c
    ../original/src/unix/udp.c
)

add_library(uv_test_legacy_support STATIC ${uv_test_legacy_support_sources})
target_compile_definitions(
  uv_test_legacy_support
  PRIVATE
    SAFE_LIBUV_DEFAULT_THREADPOOL_SIZE=2
    ${uv_defines}
    SAFE_LIBUV_DISABLE_IO_URING=1
    SAFE_LIBUV_RUST_LOOP_CORE=1
    uv_loop_configure=uv_test_support_uv_loop_configure
)
target_compile_options(
  uv_test_legacy_support
  PRIVATE
    ${uv_cflags}
    -fvisibility=default
    -include
    ${PROJECT_SOURCE_DIR}/legacy/generated/legacy_rename.h
)
target_include_directories(
  uv_test_legacy_support
  PRIVATE
    ${PROJECT_SOURCE_DIR}/include
    ${PROJECT_SOURCE_DIR}/../original/include
    ${PROJECT_SOURCE_DIR}/../original/src
)
target_link_libraries(uv_test_legacy_support PRIVATE ${uv_libraries})
set_target_properties(
  uv_test_legacy_support
  PROPERTIES
    C_VISIBILITY_PRESET default
    ARCHIVE_OUTPUT_DIRECTORY ${CMAKE_BINARY_DIR}
    OUTPUT_NAME uv_test_legacy_support
    POSITION_INDEPENDENT_CODE ON
)

add_library(uv_test_phase5_private_support STATIC ${uv_test_phase5_private_support_sources})
target_compile_definitions(
  uv_test_phase5_private_support
  PRIVATE
    ${uv_defines}
    SAFE_LIBUV_RUST_LOOP_CORE=1
)
target_compile_options(
  uv_test_phase5_private_support
  PRIVATE
    ${uv_cflags}
    -fvisibility=default
    -include
    ${PROJECT_SOURCE_DIR}/legacy/generated/phase5_private_rename.h
)
target_include_directories(
  uv_test_phase5_private_support
  PRIVATE
    ${PROJECT_SOURCE_DIR}/include
    ${PROJECT_SOURCE_DIR}/../original/include
    ${PROJECT_SOURCE_DIR}/../original/src
)
target_link_libraries(uv_test_phase5_private_support PRIVATE ${uv_libraries})
set_target_properties(
  uv_test_phase5_private_support
  PROPERTIES
    C_VISIBILITY_PRESET default
    ARCHIVE_OUTPUT_DIRECTORY ${CMAKE_BINARY_DIR}
    OUTPUT_NAME uv_test_phase5_private_support
    POSITION_INDEPENDENT_CODE ON
)

add_library(
  uv_test_phase5_forwarders
  STATIC
    legacy/generated/phase5_private_forwarders.c
)
target_compile_options(
  uv_test_phase5_forwarders
  PRIVATE
    ${uv_cflags}
    -fvisibility=default
)
set_target_properties(
  uv_test_phase5_forwarders
  PROPERTIES
    C_VISIBILITY_PRESET default
    ARCHIVE_OUTPUT_DIRECTORY ${CMAKE_BINARY_DIR}
    OUTPUT_NAME uv_test_phase5_forwarders
    POSITION_INDEPENDENT_CODE ON
)

add_custom_command(
  OUTPUT
    ${STAGED_SHARED_LIBRARY}
    ${STAGED_SHARED_LIBRARY_SONAME}
    ${STAGED_SHARED_LIBRARY_LINK}
    ${STAGED_STATIC_LIBRARY}
    ${STAGED_BUILD_MANIFEST}
  COMMAND
    ${CMAKE_COMMAND} -E env
      CARGO_TARGET_DIR=${CARGO_TARGET_DIR}
      cargo build --manifest-path ${PROJECT_SOURCE_DIR}/Cargo.toml ${CARGO_BUILD_ARGS}
  COMMAND
    /bin/bash -lc
    "set -euo pipefail; \
     manifest=$(find '${CARGO_TARGET_DIR}/${CARGO_PROFILE}/build' -path '*/out/libuv-build-manifest.json' | sort | tail -n1); \
     test -n \"$manifest\"; \
     cp '${CARGO_SHARED_LIBRARY}' '${STAGED_SHARED_LIBRARY}'; \
     '${PATCHELF_EXECUTABLE}' --clear-symbol-version uv_loop_configure '${STAGED_SHARED_LIBRARY}'; \
     cp '${CARGO_STATIC_LIBRARY}' '${STAGED_STATIC_LIBRARY}'; \
     cp \"$manifest\" '${STAGED_BUILD_MANIFEST}'; \
     rm -f '${STAGED_SHARED_LIBRARY_SONAME}' '${STAGED_SHARED_LIBRARY_LINK}'; \
     ln -s 'libuv.so.1.0.0' '${STAGED_SHARED_LIBRARY_SONAME}'; \
     ln -s 'libuv.so.1.0.0' '${STAGED_SHARED_LIBRARY_LINK}'; \
     rm -rf '${CMAKE_BINARY_DIR}/include'; \
     cp -a '${PROJECT_SOURCE_DIR}/include' '${CMAKE_BINARY_DIR}/include'"
  DEPENDS
    ${UV_CARGO_INPUTS}
    ${UV_LEGACY_INPUTS}
  VERBATIM
)

add_custom_target(
  uv_artifacts ALL
  DEPENDS
    ${STAGED_SHARED_LIBRARY}
    ${STAGED_SHARED_LIBRARY_SONAME}
    ${STAGED_SHARED_LIBRARY_LINK}
    ${STAGED_STATIC_LIBRARY}
    ${STAGED_BUILD_MANIFEST}
)

add_library(uv SHARED IMPORTED GLOBAL)
set_target_properties(uv PROPERTIES
  IMPORTED_LOCATION ${STAGED_SHARED_LIBRARY}
  IMPORTED_SONAME libuv.so.1
  INTERFACE_INCLUDE_DIRECTORIES ${CMAKE_BINARY_DIR}/include
  INTERFACE_LINK_LIBRARIES "pthread;dl;rt"
)

add_library(uv_a STATIC IMPORTED GLOBAL)
set_target_properties(uv_a PROPERTIES
  IMPORTED_LOCATION ${STAGED_STATIC_LIBRARY}
  INTERFACE_INCLUDE_DIRECTORIES ${CMAKE_BINARY_DIR}/include
  INTERFACE_LINK_LIBRARIES "pthread;dl;rt"
)

add_dependencies(uv uv_artifacts)
add_dependencies(uv_a uv_artifacts)

set(uv_test_sources
    test/runner-unix.c)

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
    test/runner.c)

set(uv_test_sources
    ${uv_test_sources}
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
    test/test-public-api.c
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
    test/test-watcher-cross-stop.c)

add_library(uv_test_idna_intercept SHARED test/test-idna-intercept.c)
target_compile_options(uv_test_idna_intercept PRIVATE ${uv_cflags})
target_link_libraries(uv_test_idna_intercept PRIVATE ${CMAKE_DL_LIBS})
target_include_directories(uv_test_idna_intercept PRIVATE ${PROJECT_SOURCE_DIR}/include ${PROJECT_SOURCE_DIR}/test)

add_executable(uv_run_benchmarks_a ${uv_benchmark_sources})
target_compile_definitions(uv_run_benchmarks_a PRIVATE ${uv_defines})
target_compile_options(uv_run_benchmarks_a PRIVATE ${uv_cflags})
target_include_directories(uv_run_benchmarks_a PRIVATE ${PROJECT_SOURCE_DIR}/include ${PROJECT_SOURCE_DIR}/test)
target_link_options(uv_run_benchmarks_a PRIVATE -Wl,--export-dynamic)
target_link_libraries(
  uv_run_benchmarks_a
  PRIVATE
    uv_a
    "$<LINK_LIBRARY:WHOLE_ARCHIVE,uv_test_legacy_support>"
    "$<LINK_LIBRARY:WHOLE_ARCHIVE,uv_test_phase5_forwarders>"
    "$<LINK_LIBRARY:WHOLE_ARCHIVE,uv_test_phase5_private_support>"
    ${uv_test_libraries}
)
add_dependencies(uv_run_benchmarks_a uv_artifacts)

add_executable(uv_run_tests ${uv_test_sources})
target_compile_definitions(
  uv_run_tests
  PRIVATE
    ${uv_defines}
    USING_UV_SHARED=1
    UV_TEST_HAS_IDNA_INTERCEPT=1
    UV_TEST_IDNA_INTERCEPT_PATH=\"$<TARGET_FILE:uv_test_idna_intercept>\"
)
target_compile_options(uv_run_tests PRIVATE ${uv_cflags})
target_include_directories(uv_run_tests PRIVATE ${PROJECT_SOURCE_DIR}/include ${PROJECT_SOURCE_DIR}/test)
target_link_options(uv_run_tests PRIVATE -Wl,--export-dynamic)
target_link_libraries(
  uv_run_tests
  PRIVATE
    uv
    "$<LINK_LIBRARY:WHOLE_ARCHIVE,uv_test_legacy_support>"
    "$<LINK_LIBRARY:WHOLE_ARCHIVE,uv_test_phase5_forwarders>"
    "$<LINK_LIBRARY:WHOLE_ARCHIVE,uv_test_phase5_private_support>"
    uv_a
    ${uv_test_libraries}
)
target_link_directories(uv_run_tests PRIVATE ${CMAKE_BINARY_DIR})
set_target_properties(uv_run_tests PROPERTIES BUILD_RPATH ${CMAKE_BINARY_DIR})
add_dependencies(uv_run_tests uv_artifacts uv_test_idna_intercept)
add_test(NAME uv_test COMMAND uv_run_tests WORKING_DIRECTORY ${PROJECT_SOURCE_DIR})

add_executable(uv_run_tests_a ${uv_test_sources})
target_compile_definitions(
  uv_run_tests_a
  PRIVATE
    ${uv_defines}
    UV_TEST_HAS_IDNA_INTERCEPT=1
    UV_TEST_IDNA_INTERCEPT_PATH=\"$<TARGET_FILE:uv_test_idna_intercept>\"
)
target_compile_options(uv_run_tests_a PRIVATE ${uv_cflags})
target_include_directories(uv_run_tests_a PRIVATE ${PROJECT_SOURCE_DIR}/include ${PROJECT_SOURCE_DIR}/test)
target_link_options(uv_run_tests_a PRIVATE -Wl,--export-dynamic)
target_link_libraries(
  uv_run_tests_a
  PRIVATE
    uv_a
    "$<LINK_LIBRARY:WHOLE_ARCHIVE,uv_test_legacy_support>"
    "$<LINK_LIBRARY:WHOLE_ARCHIVE,uv_test_phase5_forwarders>"
    "$<LINK_LIBRARY:WHOLE_ARCHIVE,uv_test_phase5_private_support>"
    ${uv_test_libraries}
)
add_dependencies(uv_run_tests_a uv_artifacts uv_test_idna_intercept)
add_test(NAME uv_test_a COMMAND uv_run_tests_a WORKING_DIRECTORY ${PROJECT_SOURCE_DIR})

add_executable(
  uv_run_safe_regressions
  test/runner-unix.c
  test/runner.c
  test-extra/privdrop-wrap.c
  test-extra/run-regressions.c
  test-extra/test-getaddrinfo-long-hostname.c
  test-extra/test-io-uring-privdrop.c
  test-extra/test-supplementary-groups-dropped.c
)
target_compile_definitions(uv_run_safe_regressions PRIVATE ${uv_defines})
target_compile_options(uv_run_safe_regressions PRIVATE ${uv_cflags})
target_include_directories(
  uv_run_safe_regressions
  PRIVATE
    ${PROJECT_SOURCE_DIR}/include
    ${PROJECT_SOURCE_DIR}/test
    ${PROJECT_SOURCE_DIR}/test-extra
)
target_link_options(uv_run_safe_regressions PRIVATE -Wl,--export-dynamic)
target_link_libraries(
  uv_run_safe_regressions
  PRIVATE
    uv_a
    "$<LINK_LIBRARY:WHOLE_ARCHIVE,uv_test_legacy_support>"
    "$<LINK_LIBRARY:WHOLE_ARCHIVE,uv_test_phase5_forwarders>"
    "$<LINK_LIBRARY:WHOLE_ARCHIVE,uv_test_phase5_private_support>"
    ${uv_test_libraries}
)
add_dependencies(uv_run_safe_regressions uv_artifacts)
add_test(NAME uv_safe_regressions COMMAND uv_run_safe_regressions WORKING_DIRECTORY ${PROJECT_SOURCE_DIR})

install(DIRECTORY ${PROJECT_SOURCE_DIR}/include/ DESTINATION ${CMAKE_INSTALL_INCLUDEDIR})
install(FILES ${CMAKE_BINARY_DIR}/libuv.pc DESTINATION ${CMAKE_INSTALL_LIBDIR}/pkgconfig)
install(FILES ${CMAKE_BINARY_DIR}/libuv-static.pc DESTINATION ${CMAKE_INSTALL_LIBDIR}/pkgconfig)
install(FILES ${CMAKE_BINARY_DIR}/libuvConfig.cmake DESTINATION ${CMAKE_INSTALL_LIBDIR}/cmake/libuv)
install(FILES ${STAGED_SHARED_LIBRARY} DESTINATION ${CMAKE_INSTALL_LIBDIR} RENAME libuv.so.1.0.0)
install(FILES ${STAGED_STATIC_LIBRARY} DESTINATION ${CMAKE_INSTALL_LIBDIR} RENAME libuv.a)
