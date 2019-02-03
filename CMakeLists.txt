cmake_minimum_required (VERSION 2.6)
project (toid)

add_executable(toid toid.cpp)

include_directories(portaudio/include)
add_subdirectory(portaudio)

target_link_libraries (toid portaudio)
