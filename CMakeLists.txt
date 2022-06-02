cmake_minimum_required (VERSION 3.22)

project ("libmus")

add_subdirectory("src/libmus")
add_subdirectory("tests/libmus_tests")

enable_testing()
