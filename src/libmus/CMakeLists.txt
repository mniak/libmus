cmake_minimum_required (VERSION 3.22)

set(CMAKE_CXX_STANDARD 20)

add_library (libmus 
	"PitchClass.cpp"
	"Pitch.cpp"
	"Utils.cpp"
)

set(Boost_USE_STATIC_LIBS OFF)
set(Boost_USE_MULTITHREADED ON)
set(Boost_USE_STATIC_RUNTIME OFF)
find_package(Boost 1.50 REQUIRED COMPONENTS filesystem)
message(STATUS "Boost version: ${Boost_VERSION}")

include_directories(${Boost_INCLUDE_DIRS})
target_link_libraries(libmus ${Boost_LIBRARIES})