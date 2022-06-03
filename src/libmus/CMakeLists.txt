cmake_minimum_required (VERSION 3.22)

file(GLOB cpp_files *.cpp)
add_library(libmus ${cpp_files})

set(Boost_USE_STATIC_LIBS OFF)
set(Boost_USE_MULTITHREADED ON)
set(Boost_USE_STATIC_RUNTIME OFF)
find_package(Boost 1.50 REQUIRED)
message(STATUS "Boost version: ${Boost_VERSION}")

include_directories(${Boost_INCLUDE_DIRS})
target_link_libraries(libmus ${Boost_LIBRARIES})