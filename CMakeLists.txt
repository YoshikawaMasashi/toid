cmake_minimum_required (VERSION 2.6)
project (toid)

add_executable(toid toid.cpp)

target_link_libraries (toid portaudio)
