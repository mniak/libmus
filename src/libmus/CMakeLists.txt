cmake_minimum_required (VERSION 3.22)

file(GLOB cpp_files *.cpp)
add_library(libmus ${cpp_files})
