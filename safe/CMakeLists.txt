cmake_minimum_required(VERSION 3.28)

project(libuv_safe LANGUAGES C)

set(CMAKE_EXPORT_COMPILE_COMMANDS ON)

add_custom_target(uv_scaffold
  COMMAND ${CMAKE_COMMAND} -E echo "libuv-safe phase 1 scaffold"
  VERBATIM
)
