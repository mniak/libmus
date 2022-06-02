cmake_minimum_required (VERSION 3.12)

set(CMAKE_CXX_STANDARD 20)

add_library (libmus 
	"PitchClass.cpp"
	"Pitch.cpp"
	"Utils.cpp"
)

