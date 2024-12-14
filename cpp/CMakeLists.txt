cmake_minimum_required(VERSION 3.5.0)
set(CMAKE_CXX_STANDARD 20)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

project(practice VERSION 0.1.0 LANGUAGES C CXX)

add_subdirectory(log_util)

include_directories(3rd_party/simple_logger/include 3rd_party/simple_logger/third_party/fmt/include)

add_executable(practice main.cpp ${SOURCE_FILES})
target_link_libraries(practice PUBLIC log)
