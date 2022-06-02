cmake_minimum_required (VERSION 3.22)

set(CMAKE_CXX_STANDARD 20)

include(FetchContent)
FetchContent_Declare(
  googletest
  URL https://github.com/google/googletest/archive/609281088cfefc76f9d0ce82e1ff6c30cc3591e5.zip
)
# For Windows: Prevent overriding the parent project's compiler/linker settings
set(gtest_force_shared_crt ON CACHE BOOL "" FORCE)
FetchContent_MakeAvailable(googletest)

enable_testing()

file(GLOB cpp_files *.cpp)
add_executable(libmus_tests ${cpp_files})

target_link_libraries(libmus_tests
  libmus
  gtest_main
)
include_directories("../../src")
include(GoogleTest)
gtest_discover_tests(libmus_tests)
