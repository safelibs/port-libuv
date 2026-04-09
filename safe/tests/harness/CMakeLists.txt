cmake_minimum_required(VERSION 3.20)
project(libuv_safe_harness C)

set(CMAKE_C_STANDARD 90)
set(CMAKE_C_STANDARD_REQUIRED ON)

get_filename_component(LIBUV_SOURCE_ROOT "${CMAKE_CURRENT_LIST_DIR}/../../.." ABSOLUTE)
set(STAGE_PREFIX "" CACHE PATH "Stage prefix created by safe/tools/stage_install.sh")

if(NOT STAGE_PREFIX)
  message(FATAL_ERROR "Set STAGE_PREFIX to a staged safe/libuv prefix")
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
)

file(GENERATE
  OUTPUT "${CMAKE_CURRENT_BINARY_DIR}/test-getters-setters-main.c"
  CONTENT "int run_test_getters_setters(void);\nint main(void) { return run_test_getters_setters(); }\n"
)

file(GENERATE
  OUTPUT "${CMAKE_CURRENT_BINARY_DIR}/benchmark-sizes-main.c"
  CONTENT "int run_benchmark_sizes(void);\nint main(void) { return run_benchmark_sizes(); }\n"
)

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

add_safe_probe(
  safe_test_getters_setters_shared
  shared
  "${CMAKE_CURRENT_BINARY_DIR}/test-getters-setters-main.c"
  "${ORIGINAL_TEST_DIR}/test-getters-setters.c"
)

add_safe_probe(
  safe_test_getters_setters_static
  static
  "${CMAKE_CURRENT_BINARY_DIR}/test-getters-setters-main.c"
  "${ORIGINAL_TEST_DIR}/test-getters-setters.c"
)

add_safe_probe(
  safe_benchmark_sizes_shared
  shared
  "${CMAKE_CURRENT_BINARY_DIR}/benchmark-sizes-main.c"
  "${ORIGINAL_TEST_DIR}/benchmark-sizes.c"
)

add_safe_probe(
  safe_benchmark_sizes_static
  static
  "${CMAKE_CURRENT_BINARY_DIR}/benchmark-sizes-main.c"
  "${ORIGINAL_TEST_DIR}/benchmark-sizes.c"
)
