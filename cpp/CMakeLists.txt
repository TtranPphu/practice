cmake_minimum_required(VERSION 3.5.0)
set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

project(practice VERSION 0.1.0 LANGUAGES C CXX)

add_subdirectory(log_util)

add_executable(practice main.cpp ${SOURCE_FILES})
target_link_libraries(practice PUBLIC log)
