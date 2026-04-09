cmake_minimum_required(VERSION 3.20)
project(libuv_safe_harness C)

set(CMAKE_C_STANDARD 90)
set(CMAKE_C_STANDARD_REQUIRED ON)

get_filename_component(LIBUV_SOURCE_ROOT "${CMAKE_CURRENT_LIST_DIR}/../../.." ABSOLUTE)
set(STAGE_PREFIX "" CACHE PATH "Stage prefix created by safe/tools/stage_install.sh")
set(GENERATED_DIR "" CACHE PATH "Generated harness sources written by build_upstream_harness.sh")

if(NOT STAGE_PREFIX)
  message(FATAL_ERROR "Set STAGE_PREFIX to a staged safe/libuv prefix")
endif()

if(NOT GENERATED_DIR)
  message(FATAL_ERROR "Set GENERATED_DIR to the generated harness source directory")
endif()

set(ORIGINAL_TEST_DIR "${LIBUV_SOURCE_ROOT}/original/test")

add_library(uv_shared SHARED IMPORTED GLOBAL)
set_target_properties(uv_shared PROPERTIES
  IMPORTED_LOCATION "${STAGE_PREFIX}/lib/libuv.so"
  INTERFACE_INCLUDE_DIRECTORIES "${STAGE_PREFIX}/include"
)

add_library(uv_static STATIC IMPORTED GLOBAL)
set_target_properties(uv_static PROPERTIES
  IMPORTED_LOCATION "${STAGE_PREFIX}/lib/libuv.a"
  INTERFACE_INCLUDE_DIRECTORIES "${STAGE_PREFIX}/include"
)

find_package(Threads REQUIRED)

set(COMMON_INCLUDES
  "${STAGE_PREFIX}/include"
  "${ORIGINAL_TEST_DIR}"
  "${GENERATED_DIR}"
)

set(UPSTREAM_CORE_TEST_SOURCES
  "${GENERATED_DIR}/uv-safe-run-tests.c"
  "${ORIGINAL_TEST_DIR}/runner.c"
  "${ORIGINAL_TEST_DIR}/runner-unix.c"
  "${ORIGINAL_TEST_DIR}/test-async.c"
  "${ORIGINAL_TEST_DIR}/test-barrier.c"
  "${ORIGINAL_TEST_DIR}/test-condvar.c"
  "${ORIGINAL_TEST_DIR}/test-getters-setters.c"
  "${ORIGINAL_TEST_DIR}/test-loop-alive.c"
  "${ORIGINAL_TEST_DIR}/test-loop-close.c"
  "${ORIGINAL_TEST_DIR}/test-loop-configure.c"
  "${ORIGINAL_TEST_DIR}/test-loop-stop.c"
  "${ORIGINAL_TEST_DIR}/test-metrics.c"
  "${ORIGINAL_TEST_DIR}/test-mutexes.c"
  "${ORIGINAL_TEST_DIR}/test-random.c"
  "${ORIGINAL_TEST_DIR}/test-run-once.c"
  "${ORIGINAL_TEST_DIR}/test-run-nowait.c"
  "${ORIGINAL_TEST_DIR}/test-semaphore.c"
  "${ORIGINAL_TEST_DIR}/test-thread.c"
  "${ORIGINAL_TEST_DIR}/test-threadpool-cancel.c"
  "${ORIGINAL_TEST_DIR}/test-threadpool.c"
)

function(add_safe_runner name linkage)
  add_executable("${name}" ${UPSTREAM_CORE_TEST_SOURCES})
  target_include_directories("${name}" PRIVATE ${COMMON_INCLUDES})
  if("${linkage}" STREQUAL "shared")
    target_link_libraries("${name}" PRIVATE uv_shared)
    set_target_properties("${name}" PROPERTIES
      BUILD_RPATH "${STAGE_PREFIX}/lib"
    )
  else()
    target_link_libraries("${name}" PRIVATE uv_static Threads::Threads dl rt util)
  endif()
endfunction()

function(add_safe_probe name linkage wrapper source)
  add_executable("${name}" "${wrapper}" "${source}")
  target_include_directories("${name}" PRIVATE ${COMMON_INCLUDES})
  if("${linkage}" STREQUAL "shared")
    target_link_libraries("${name}" PRIVATE uv_shared)
    set_target_properties("${name}" PROPERTIES
      BUILD_RPATH "${STAGE_PREFIX}/lib"
    )
  else()
    target_link_libraries("${name}" PRIVATE uv_static Threads::Threads dl rt util)
  endif()
endfunction()

add_safe_runner(uv_safe_run_tests_shared shared)
add_safe_runner(uv_safe_run_tests_static static)

add_safe_probe(
  uv_safe_benchmark_sizes_shared
  shared
  "${GENERATED_DIR}/benchmark-sizes-main.c"
  "${ORIGINAL_TEST_DIR}/benchmark-sizes.c"
)

add_safe_probe(
  uv_safe_benchmark_sizes_static
  static
  "${GENERATED_DIR}/benchmark-sizes-main.c"
  "${ORIGINAL_TEST_DIR}/benchmark-sizes.c"
)
