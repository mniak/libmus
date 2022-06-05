cmake_minimum_required (VERSION 3.22)

include(FetchContent)


## GTest
FetchContent_Declare(
  googletest
  URL https://github.com/google/googletest/archive/609281088cfefc76f9d0ce82e1ff6c30cc3591e5.zip
)
# For Windows: Prevent overriding the parent project's compiler/linker settings
set(gtest_force_shared_crt ON CACHE BOOL "" FORCE)
FetchContent_MakeAvailable(googletest)

enable_testing()


## Boost
set(Boost_USE_STATIC_LIBS OFF)
set(Boost_USE_MULTITHREADED ON)
set(Boost_USE_STATIC_RUNTIME OFF)
find_package(Boost 1.50 REQUIRED)

file(GLOB cpp_files *.cpp)
add_executable(libmus_tests ${cpp_files})

# General 
target_link_libraries(libmus_tests
  libmus
  gtest_main
  ${Boost_LIBRARIES}
)
include_directories(
  "../../src"
  ${Boost_INCLUDE_DIRS}
)

include(GoogleTest)
gtest_discover_tests(libmus_tests)
